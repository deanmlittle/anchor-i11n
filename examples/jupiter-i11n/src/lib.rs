use anchor_lang::prelude::*;
use anchor_introspection::prelude::*;

mod jupiter_program {

    use super::*;

    declare_id!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");

    // Instructions:

    // Route [0]
    #[derive(TryFromInstruction)]
    pub struct Route<'info> {
        pub accounts: RouteAccounts<'info>,
        pub args: RouteArgs
    }

    let ctx = Route::try_from_instruction(&ix)?;

    #[derive(AnchorDeserialize, AnchorSerialize)]
    pub struct RouteArgs {
        pub route_plan: Vec<RoutePlanStep>,
        pub in_amount: u64,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8
    }

    #[derive(TryFromAccountMetas)]
    pub struct RouteAccounts<'info> {
        pub token_program: &'info AccountMeta,
        pub user_transfer_authority: &'info AccountMeta,
        pub user_source_token_account: &'info AccountMeta,
        pub user_destination_token_account: &'info AccountMeta,
        pub destination_token_account: Option<&'info AccountMeta>,
        pub destination_mint: &'info AccountMeta,
        pub platform_fee_account: Option<&'info AccountMeta>,
        pub event_authority: &'info AccountMeta,
        pub program: &'info AccountMeta,
    }

    // RouteTokenLedger [1]
    #[derive(TryFromInstruction)]
    pub struct RouteTokenLedger<'info> {
        pub accounts: RouteTokenLedgerAccounts<'info>,
        pub args: RouteTokenLedgerArgs
    }

    #[derive(AnchorDeserialize, AnchorSerialize)]
    pub struct RouteTokenLedgerArgs {
        pub route_plan: Vec<RoutePlanStep>,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8
    }

    #[derive(TryFromAccountMetas)]
    pub struct RouteTokenLedgerAccounts<'info> {
        pub token_program: &'info AccountMeta,
        pub user_transfer_authority: &'info AccountMeta,
        pub user_source_token_account: &'info AccountMeta,
        pub user_destination_token_account: &'info AccountMeta,
        pub destination_token_account: Option<&'info AccountMeta>,
        pub destination_mint: &'info AccountMeta,
        pub platform_fee_account: Option<&'info AccountMeta>,
        pub token_ledger: &'info AccountMeta,
        pub event_authority: &'info AccountMeta,
        pub program: &'info AccountMeta,
    }

    // SharedAccountsRoute [2]
    #[derive(TryFromInstruction)]
    pub struct SharedAccountsRoute<'info> {
        pub accounts: SharedAccountsRouteAccounts<'info>,
        pub args: SharedAccountsRouteArgs
    }

    #[derive(AnchorDeserialize, AnchorSerialize)]
    pub struct SharedAccountsRouteArgs {
        pub id: u8,
        pub route_plan: Vec<RoutePlanStep>,
        pub in_amount: u64,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8
    }

    #[derive(TryFromAccountMetas)]
    pub struct SharedAccountsRouteAccounts<'info> {
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
        pub token2022_program: Option<&'info AccountMeta>,
        pub event_authority: &'info AccountMeta,
        pub program: &'info AccountMeta,
    }

    // SharedAccountsRouteTokenLedger [3]
    #[derive(TryFromInstruction)]
    pub struct SharedAccountsRouteTokenLedger<'info> {
        pub accounts: SharedAccountsRouteTokenLedgerAccounts<'info>,
        pub args: SharedAccountsRouteTokenLedgerArgs
    }

    #[derive(AnchorDeserialize, AnchorSerialize)]
    pub struct SharedAccountsRouteTokenLedgerArgs {
        pub id: u8,
        pub route_plan: Vec<RoutePlanStep>,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8
    }

    #[derive(TryFromAccountMetas)]
    pub struct SharedAccountsRouteTokenLedgerAccounts<'info> {
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
        pub token2022_program: Option<&'info AccountMeta>,
        pub token_ledger: &'info AccountMeta,
        pub event_authority: &'info AccountMeta,
        pub program: &'info AccountMeta,
    }

    // ExactOutRoute [4]
    #[derive(TryFromInstruction)]
    pub struct ExactOutRoute<'info> {
        pub accounts: ExactOutRouteAccounts<'info>,
        pub args: ExactOutRouteArgs
    }

    #[derive(AnchorDeserialize, AnchorSerialize)]
    pub struct ExactOutRouteArgs {
        pub route_plan: Vec<RoutePlanStep>,
        pub out_amount: u64,
        pub quoted_in_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8
    }

    #[derive(TryFromAccountMetas)]
    pub struct ExactOutRouteAccounts<'info> {
        pub token_program: &'info AccountMeta,
        pub user_transfer_authority: &'info AccountMeta,
        pub user_source_token_account: &'info AccountMeta,
        pub user_destination_token_account: &'info AccountMeta,
        pub destination_token_account: Option<&'info AccountMeta>,
        pub source_mint: &'info AccountMeta,
        pub destination_mint: &'info AccountMeta,
        pub platform_fee_account: Option<&'info AccountMeta>,
        pub token2022_program: Option<&'info AccountMeta>,
        pub event_authority: &'info AccountMeta,
        pub program: &'info AccountMeta,
    }

    // SharedAccountsExactOutRoute [5]
    #[derive(TryFromInstruction)]
    pub struct SharedAccountsExactOutRoute<'info> {
        pub accounts: SharedAccountsExactOutRouteAccounts<'info>,
        pub args: SharedAccountsExactOutRouteArgs
    }

    #[derive(AnchorDeserialize, AnchorSerialize)]
    pub struct SharedAccountsExactOutRouteArgs {
        pub id: u8,
        pub route_plan: Vec<RoutePlanStep>,
        pub out_amount: u64,
        pub quoted_in_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8
    }

    #[derive(TryFromAccountMetas)]
    pub struct SharedAccountsExactOutRouteAccounts<'info> {
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
        pub token2022_program: Option<&'info AccountMeta>,
        pub event_authority: &'info AccountMeta,
        pub program: &'info AccountMeta,
    }

    // SetTokenLedger [6]
    #[derive(TryFromInstruction)]
    pub struct SetTokenLedger<'info> {
        pub accounts: SetTokenLedgerAccounts<'info>,
        pub args: SetTokenLedgerArgs
    }

    #[derive(AnchorDeserialize, AnchorSerialize)]
    pub struct SetTokenLedgerArgs {}

    #[derive(TryFromAccountMetas)]
    pub struct SetTokenLedgerAccounts<'info> {
        pub token_ledger: &'info AccountMeta,
        pub token_account: &'info AccountMeta,
    }

    // CreateOpenOrders [7]
    #[derive(TryFromInstruction)]
    pub struct CreateOpenOrders<'info> {
        pub accounts: CreateOpenOrdersAccounts<'info>,
        pub args: CreateOpenOrdersArgs
    }

    #[derive(AnchorDeserialize, AnchorSerialize)]
    pub struct CreateOpenOrdersArgs {}

    #[derive(TryFromAccountMetas)]
    pub struct CreateOpenOrdersAccounts<'info> {
        pub open_orders: &'info AccountMeta,
        pub payer: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub rent: &'info AccountMeta,
        pub market: &'info AccountMeta,
    }

    // CreateProgramOpenOrders [8]
    #[derive(TryFromInstruction)]
    pub struct CreateProgramOpenOrders<'info> {
        pub accounts: CreateProgramOpenOrdersAccounts<'info>,
        pub args: CreateProgramOpenOrdersArgs
    }

    #[derive(AnchorDeserialize, AnchorSerialize)]
    pub struct CreateProgramOpenOrdersArgs {
        pub id: u8
    }

    #[derive(TryFromAccountMetas)]
    pub struct CreateProgramOpenOrdersAccounts<'info> {
        pub open_orders: &'info AccountMeta,
        pub payer: &'info AccountMeta,
        pub program_authority: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub rent: &'info AccountMeta,
        pub market: &'info AccountMeta,
    }

    // Claim [9]
    #[derive(TryFromInstruction)]
    pub struct Claim<'info> {
        pub accounts: ClaimAccounts<'info>,
        pub args: ClaimArgs
    }

    #[derive(AnchorDeserialize, AnchorSerialize)]
    pub struct ClaimArgs {
        pub id: u8
    }

    #[derive(TryFromAccountMetas)]
    pub struct ClaimAccounts<'info> {
        pub wallet: &'info AccountMeta,
        pub program_authority: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    // ClaimToken [10]
    #[derive(TryFromInstruction)]
    pub struct ClaimToken<'info> {
        pub accounts: ClaimTokenAccounts<'info>,
        pub args: ClaimTokenArgs
    }

    #[derive(AnchorDeserialize, AnchorSerialize)]
    pub struct ClaimTokenArgs {
        pub id: u8
    }

    #[derive(TryFromAccountMetas)]
    pub struct ClaimTokenAccounts<'info> {
        pub payer: &'info AccountMeta,
        pub wallet: &'info AccountMeta,
        pub program_authority: &'info AccountMeta,
        pub program_token_account: &'info AccountMeta,
        pub destination_token_account: &'info AccountMeta,
        pub mint: &'info AccountMeta,
        pub associated_token_token_program: &'info AccountMeta,
        pub associated_token_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    // CreateTokenLedger [11]
    #[derive(TryFromInstruction)]
    pub struct CreateTokenLedger<'info> {
        pub accounts: CreateTokenLedgerAccounts<'info>,
        pub args: CreateTokenLedgerArgs
    }

    #[derive(AnchorDeserialize, AnchorSerialize)]
    pub struct CreateTokenLedgerArgs {}

    #[derive(TryFromAccountMetas)]
    pub struct CreateTokenLedgerAccounts<'info> {
        pub token_ledger: &'info AccountMeta,
        pub payer: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    // Types

    // AmountWithSlippage [0]
    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct AmountWithSlippage {
        pub amount: u64,
        pub slippage_bps: u16
    }

    // RoutePlanStep [1]
    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct RoutePlanStep {
        pub swap: Swap,
        pub percent: u8,
        pub input_index: u8,
        pub output_index: u8
    }

    // Side [2]
    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub enum Side {
        Bid,
        Ask
    }

    // Swap [3]
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
        Crema { x_to_y: bool },
        Lifinity,
        Mercurial,
        Cykura,
        Serum { side: Side },
        MarinadeDeposit,
        MarinadeUnstake,
        Aldrin { side: Side },
        AldrinV2 { side: Side },
        Whirlpool { a_to_b: bool },
        Invariant { x_to_y: bool },
        Meteora,
        GooseFX,
        DeltaFi { stable: bool },
        Balansol,
        MarcoPolo { x_to_y: bool },
        Dradex { side: Side },
        LifinityV2,
        RaydiumClmm,
        Openbook { side: Side },
        Phoenix { side: Side },
        Symmetry { from_token_id: u64, to_token_id: u64 },
        TokenSwapV2,
        HeliumTreasuryManagementRedeemV0,
        StakeDexStakeWrappedSol,
        StakeDexSwapViaStake { bridge_stake_seed: u32 },
        GooseFXV2,
        Perps,
        PerpsAddLiquidity,
        PerpsRemoveLiquidity,
        MeteoraDlmm,
        OpenBookV2 { side: Side },
        RaydiumClmmV2,
    }
}
  
  