use anchor_lang::prelude::*;
use anchor_introspection::prelude::*;

mod jupiter_program {

    use super::*;

    declare_id!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");

    // Route
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
        pub destination_token_account: &'info AccountMeta,
        pub destination_mint: &'info AccountMeta,
        pub platform_fee_account: Option<&'info AccountMeta>,
        pub event_authority: &'info AccountMeta,
        pub program: &'info AccountMeta,
    }

    // Common
    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct RoutePlanStep {
        pub swap: Swap,
        pub percent: u8,
        pub input_index: u8,
        pub output_index: u8
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub enum Side {
        Bid,
        Ask
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