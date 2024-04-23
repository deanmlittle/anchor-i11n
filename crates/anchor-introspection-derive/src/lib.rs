use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Type};
use sha2::{Sha256, Digest};

#[proc_macro_derive(TryFromInstruction)]
pub fn try_from_instruction(input: TokenStream) -> TokenStream {
    let context_struct = parse_macro_input!(input as DeriveInput);
    let context_name = &context_struct.ident;

    let mut hasher = Sha256::new();
    hasher.update(format!("global:{}", context_name.to_string()).as_bytes());

    let mut discriminator_bytes: [u8;8] = [0u8;8];
    discriminator_bytes.clone_from_slice(&hasher.finalize().to_vec()[..8]);

    let discriminator: Vec<_> = discriminator_bytes.into_iter().map(|i| {
        let idx = i as u8;
        quote! { #idx }
    }).collect();

    // Extract lifetime from the generic parameters, if any
    let lifetime = match context_struct
    .generics
    .lifetimes()
    .next() {
        Some(l) => {
            let lifetime_name = &l.lifetime;
            quote! {#lifetime_name}
        },
        None => quote!{}
    };

    let mut has_accounts_type = None;
    let mut has_args_type = None;

    if let Data::Struct(data_struct) = &context_struct.data {
        if let Fields::Named(fields) = &data_struct.fields {
            for field in &fields.named {
                if let Some(ident) = &field.ident {
                    match ident.to_string().as_str() {
                        "accounts" => {
                            if let Type::Path(type_path) = &field.ty {
                                if let Some(segment) = type_path.path.segments.last() {
                                    has_accounts_type = Some(segment.ident.clone());
                                }
                            }
                        }
                        "args" => {
                            if let Type::Path(type_path) = &field.ty {
                                if let Some(segment) = type_path.path.segments.last() {
                                    has_args_type = Some(segment.ident.clone());
                                }
                            }
                        }
                        _ => panic!("Expected field name of \"accounts\" or \"args\"."),
                    }
                }
            }
        } else {
            panic!("Expected named fields in the struct.");
        }
    } else {
        panic!("Expected named fields in the struct.");
    }

    // Unwrap is safe here because we ensure both fields are present
    let (accounts_type, args_type) = (has_accounts_type.unwrap(), has_args_type.unwrap());

    // Generate the discriminator
    let expanded = quote! {
        impl<#lifetime> Discriminator for #context_name<#lifetime> {
            const DISCRIMINATOR: [u8; 8] = [#(#discriminator),*];
            fn discriminator() -> [u8; 8] {
                Self::DISCRIMINATOR
            }
        }

        impl<#lifetime> TryFrom<&#lifetime anchor_lang::solana_program::instruction::Instruction> for #context_name<#lifetime> {
            type Error = Error;
    
            fn try_from(ix: &#lifetime anchor_lang::solana_program::instruction::Instruction) -> Result<#context_name<#lifetime>> {
                require_keys_eq!(ix.program_id, ID, ErrorCode::InvalidProgramId);

                require!(ix.data[..8].eq(&Self::DISCRIMINATOR), ErrorCode::InstructionDidNotDeserialize);

                let accounts = #accounts_type::<#lifetime>::try_from(&ix.accounts)?;
                let args = #args_type::try_from_slice(&ix.data[8..])?;

                Ok(#context_name {
                    accounts,
                    args
                })
            }
        }
    };

    // Convert the generated implementation back into tokens and return it
    TokenStream::from(expanded)
}

#[proc_macro_derive(TryFromAccountMetas)]
pub fn try_from_account_metas(input: TokenStream) -> TokenStream {
    let accounts_struct = parse_macro_input!(input as DeriveInput);
    let accounts_name = &accounts_struct.ident;

    // Extract lifetime from the generic parameters, if any
    let lifetime = match accounts_struct
    .generics
    .lifetimes()
    .next() {
        Some(l) => {
            let lifetime_name = &l.lifetime;
            quote! {#lifetime_name}
        },
        None => quote!{}
    };

    // Extract the field names from the struct
    let mut optional_account_names: Vec<&Ident> = vec![];
    let account_names = if let Data::Struct(data_struct) = &accounts_struct.data {
        if let syn::Fields::Named(fields) = &data_struct.fields {
            fields
                .named
                .iter()
                .map(|field| {
                    if let syn::Type::Path(type_path) = &field.ty {
                        if let Some(segment) = type_path.path.segments.last() {
                            if segment.ident.to_string() == "Option" {
                                optional_account_names.push(field.ident.as_ref().unwrap())
                            }
                        }
                    }
                    field.ident.as_ref().unwrap()
                })
                .collect::<Vec<_>>()
        } else {
            Vec::new() // Handle tuple structs or unit structs
        }
    } else {
        Vec::new() // Handle enums
    };

    // Handle optional accounts
    let optional_accounts: Vec<_> = optional_account_names
    .iter()
    .map(|ident| {
        let id = ident.to_token_stream();
        quote! {
            let #id = match &#id.pubkey.eq(&ID) {
                true => None,
                false => Some(#id), // Dereference the reference here
            };
        }
    })
    .collect();

    // Extract the number of fields in the struct
    let accounts_length = account_names.len();

    // Generate array access expressions for the value vector
    let value_indices: Vec<_> = (0..account_names.len()).map(|i| {
        let idx = syn::Index::from(i);
        quote! { &value[#idx] }
    }).collect();
    
    // Generate the implementation
    let expanded = quote! {
        impl<#lifetime> TryFrom<&#lifetime Vec<AccountMeta>> for #accounts_name<#lifetime> {
            type Error = Error;
    
            fn try_from(value: &#lifetime Vec<AccountMeta>) -> Result<Self> {
                if value.len() < #accounts_length {
                    return Err(ProgramError::NotEnoughAccountKeys.into());
                }

                let [#(#account_names),*] = [#(#value_indices),*];

                #(#optional_accounts)*

                Ok(Self {
                    #(#account_names),*
                })
            }
        }
    };

    // Convert the generated implementation back into tokens and return it
    TokenStream::from(expanded)
}
