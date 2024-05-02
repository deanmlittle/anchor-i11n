pub use anchor_i11n_derive::{AnchorDiscriminator, TryFromAccountMetas, TryFromInstruction};
pub use anchor_lang::Discriminator;

pub mod prelude {
    pub use super::{AnchorDiscriminator, Discriminator, TryFromAccountMetas, TryFromInstruction};
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use anchor_lang::{solana_program::pubkey, AnchorSerialize};
    use hex_literal::hex;
    use solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    };

    use crate::tests::jupiter::SharedAccountsRoute;

    // This is a mini version of the Jupiter SDK that we generated for unit tests
    mod jupiter {
        use crate::prelude::*;
        use anchor_lang::prelude::*;

        declare_id!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");

        // Route
        #[derive(TryFromInstruction)]
        pub struct SharedAccountsRoute<'info> {
            pub accounts: SharedAccountsRouteAccountMetas<'info>,
            pub args: instructions::SharedAccountsRoute,
            pub remaining_accounts: Vec<&'info AccountMeta>,
        }

        pub mod instructions {
            use super::*;
            use anchor_lang::prelude::*;
            #[derive(AnchorDiscriminator, AnchorDeserialize, AnchorSerialize)]
            pub struct SharedAccountsRoute {
                pub id: u8,
                pub route_plan: Vec<RoutePlanStep>,
                pub in_amount: u64,
                pub quoted_out_amount: u64,
                pub slippage_bps: u16,
                pub platform_fee_bps: u8,
            }
        }

        #[derive(TryFromAccountMetas)]
        pub struct SharedAccountsRouteAccountMetas<'info> {
            pub token_program: &'info AccountMeta,
            pub program_authority: &'info AccountMeta,
            pub user_transfer_authority: &'info AccountMeta,
            pub source_token_account: &'info AccountMeta,
            pub program_source_token_account: &'info AccountMeta,
            pub program_destination_token_account: &'info AccountMeta,
            pub destination_token_account: &'info AccountMeta,
            pub source_mint: &'info AccountMeta,
            pub destination_mint: &'info AccountMeta,
            pub platform_fee_account: Option<&'info AccountMeta>,
            pub token_2022_program: Option<&'info AccountMeta>,
            pub event_authority: &'info AccountMeta,
            pub program: &'info AccountMeta,
        }

        // Common
        #[derive(AnchorSerialize, AnchorDeserialize)]
        pub struct RoutePlanStep {
            pub swap: Swap,
            pub percent: u8,
            pub input_index: u8,
            pub output_index: u8,
        }

        #[derive(AnchorSerialize, AnchorDeserialize)]
        pub enum Side {
            Bid,
            Ask,
        }

        #[derive(AnchorSerialize, AnchorDeserialize)]
        pub enum Swap {
            Saber,
            SaberAddDecimalsDeposit,
            SaberAddDecimalsWithdraw,
            TokenSwap,
            Sencha,
            Step,
            Cropper,
            Raydium,
            Crema {
                x_to_y: bool,
            },
            Lifinity,
            Mercurial,
            Cykura,
            Serum {
                side: Side,
            },
            MarinadeDeposit,
            MarinadeUnstake,
            Aldrin {
                side: Side,
            },
            AldrinV2 {
                side: Side,
            },
            Whirlpool {
                a_to_b: bool,
            },
            Invariant {
                x_to_y: bool,
            },
            Meteora,
            GooseFX,
            DeltaFi {
                stable: bool,
            },
            Balansol,
            MarcoPolo {
                x_to_y: bool,
            },
            Dradex {
                side: Side,
            },
            LifinityV2,
            RaydiumClmm,
            Openbook {
                side: Side,
            },
            Phoenix {
                side: Side,
            },
            Symmetry {
                from_token_id: u64,
                to_token_id: u64,
            },
            TokenSwapV2,
            HeliumTreasuryManagementRedeemV0,
            StakeDexStakeWrappedSol,
            StakeDexSwapViaStake {
                bridge_stake_seed: u32,
            },
            GooseFXV2,
            Perps,
            PerpsAddLiquidity,
            PerpsRemoveLiquidity,
            MeteoraDlmm,
            OpenBookV2 {
                side: Side,
            },
            RaydiumClmmV2,
        }
    }

    #[test]
    fn test_derive() {
        // There are 13 accounts in a Jupiter SharedAccountsRoute IX, plus a bunch of remaining accounts
        let mut accounts = vec![
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7iWnBRRhBCiNXXPhqiGzvvBkKrvFSWqqmxRyu9VyYBxE"),
                false,
            ),
            AccountMeta::new(Pubkey::new_unique(), true),
            AccountMeta::new(Pubkey::new_unique(), false),
            AccountMeta::new(
                pubkey!("8VAK5Zk2q9F4W9Kxj9K8buaHTvTduPY2KEUPxzD1oJsf"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2oL6my4QDDCfpgJZX1bZV1NgbmuNptKdgcE8wJm6efgk"),
                false,
            ),
            AccountMeta::new(Pubkey::new_unique(), false),
            AccountMeta::new_readonly(
                pubkey!("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D8cy77BBepLMngZx6ZukaTff5hCt1HrWyKk3Hnd9oitf"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"),
                false,
            ),
        ];

        // Make some random remaining accounts to make it more realistic
        for i in 0..40 {
            match i % 3 == 0 {
                true => accounts.push(AccountMeta::new_readonly(Pubkey::new_unique(), false)),
                false => accounts.push(AccountMeta::new(Pubkey::new_unique(), false)),
            }
        }

        let ix = Instruction::new_with_bytes(
            jupiter::ID, 
            &hex!("c1209b3341d69c810f04000000261e000317013c000417010a00031c00640304002027b8ba04000034f881a201000000320000"), 
            accounts
        );

        let ctx = SharedAccountsRoute::try_from(&ix).unwrap();

        assert_eq!(
            ctx.accounts.source_mint.pubkey,
            pubkey!("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263")
        );
        assert_eq!(ctx.args.try_to_vec().unwrap(), &hex!("0f04000000261e000317013c000417010a00031c00640304002027b8ba04000034f881a201000000320000"));
        assert_eq!(ctx.remaining_accounts.len(), 40);
    }

    #[test]
    fn test_discriminator() {
        #[derive(AnchorDiscriminator)]
        struct IX {}
        assert_eq!(IX::DISCRIMINATOR, hex!("1c693616723a62b5"))
    }
}
