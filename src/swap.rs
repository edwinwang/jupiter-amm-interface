use anyhow::anyhow;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Copy, Clone, Debug, PartialEq)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(BorshSerialize, BorshDeserialize, Copy, Clone, Debug, PartialEq)]
pub enum HyloSwapType {
    MintStable,
    RedeemStable,
    MintLever,
    RedeemLever,
    SwapStableToLever,
    SwapLeverToStable,
    StabilityPoolDeposit,
    StabilityPoolWithdraw,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub enum CandidateSwap {
    HumidiFi {
        swap_id: u64,
        is_base_to_quote: bool,
    },
    TesseraV {
        side: Side,
    },
    HumidiFiV2 {
        swap_id: u64,
        is_base_to_quote: bool,
    },
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub struct CandidateSwapWithBps {
    pub candidate_swap: CandidateSwap,
    pub bps: u16,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
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
        a_to_b: bool,
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
    StakeDexPrefundWithdrawStakeAndDepositStake {
        bridge_stake_seed: u32,
    },
    Clone {
        pool_index: u8,
        quantity_is_input: bool,
        quantity_is_collateral: bool,
    },
    SanctumS {
        src_lst_value_calc_accs: u8,
        dst_lst_value_calc_accs: u8,
        src_lst_index: u32,
        dst_lst_index: u32,
    },
    SanctumSAddLiquidity {
        lst_value_calc_accs: u8,
        lst_index: u32,
    },
    SanctumSRemoveLiquidity {
        lst_value_calc_accs: u8,
        lst_index: u32,
    },
    RaydiumCP,
    WhirlpoolSwapV2 {
        a_to_b: bool,
        remaining_accounts_info: Option<RemainingAccountsInfo>,
    },
    OneIntro,
    PumpWrappedBuy,
    PumpWrappedSell,
    PerpsV2,
    PerpsV2AddLiquidity,
    PerpsV2RemoveLiquidity,
    MoonshotWrappedBuy,
    MoonshotWrappedSell,
    StabbleStableSwap,
    StabbleWeightedSwap,
    Obric {
        x_to_y: bool,
    },
    FoxBuyFromEstimatedCost,
    FoxClaimPartial {
        is_y: bool,
    },
    SolFi {
        is_quote_to_base: bool,
    },
    SolayerDelegateNoInit,
    SolayerUndelegateNoInit,
    TokenMill {
        side: Side,
    },
    DaosFunBuy,
    DaosFunSell,
    ZeroFi,
    StakeDexWithdrawWrappedSol,
    VirtualsBuy,
    VirtualsSell,
    Perena {
        in_index: u8,
        out_index: u8,
    },
    PumpSwapBuy,
    PumpSwapSell,
    Gamma,
    MeteoraDlmmSwapV2 {
        remaining_accounts_info: RemainingAccountsInfo,
    },
    Woofi,
    MeteoraDammV2,
    MeteoraDynamicBondingCurveSwap,
    StabbleStableSwapV2,
    StabbleWeightedSwapV2,
    RaydiumLaunchlabBuy {
        share_fee_rate: u64,
    },
    RaydiumLaunchlabSell {
        share_fee_rate: u64,
    },
    BoopdotfunWrappedBuy,
    BoopdotfunWrappedSell,
    Plasma {
        side: Side,
    },
    GoonFi {
        is_bid: bool,
        blacklist_bump: u8,
    },
    HumidiFi {
        swap_id: u64,
        is_base_to_quote: bool,
    },
    MeteoraDynamicBondingCurveSwapWithRemainingAccounts,
    TesseraV {
        side: Side,
    },
    PumpWrappedBuyV2,
    PumpWrappedSellV2,
    PumpSwapBuyV2,
    PumpSwapSellV2,
    Heaven {
        a_to_b: bool,
    },
    SolFiV2 {
        is_quote_to_base: bool,
    },
    Aquifer,
    PumpWrappedBuyV3,
    PumpWrappedSellV3,
    PumpSwapBuyV3,
    PumpSwapSellV3,
    JupiterLendDeposit,
    JupiterLendRedeem,
    DefiTuna {
        a_to_b: bool,
        remaining_accounts_info: Option<RemainingAccountsInfo>,
    },
    AlphaQ {
        a_to_b: bool,
    },
    RaydiumV2,
    SarosDlmm {
        swap_for_y: bool,
    },
    Futarchy {
        side: Side,
    },
    MeteoraDammV2WithRemainingAccounts,
    Obsidian,
    WhaleStreet {
        side: Side,
    },
    DynamicV1 {
        candidate_swaps: Vec<CandidateSwap>,
        best_position: Option<u8>,
    },
    PumpWrappedBuyV4,
    PumpWrappedSellV4,
    CarrotIssue,
    CarrotRedeem,
    Manifest {
        side: Side,
    },
    BisonFi {
        a_to_b: bool,
    },
    HumidiFiV2 {
        swap_id: u64,
        is_base_to_quote: bool,
    },
    PerenaStar {
        is_mint: bool,
    },
    JupiterRfqV2 {
        side: Side,
        fill_data: Vec<u8>,
    },
    GoonFiV2 {
        is_bid: bool,
    },
    Scorch {
        swap_id: u128,
    },
    VaultLiquidUnstake {
        lst_amounts: [u64; 5],
        seed: u64,
    },
    XOrca,
    Quantum {
        side: Side,
    },
    WhaleStreetV2 {
        side: Side,
        auth_amount_in: u64,
        auth: u64,
    },
    Riptide {
        amount_is_token_a: bool,
    },
    RunnerRodeo,
    TaurusFi {
        is_base_in: bool,
    },
    Omnipair,
    MSwap,
    Hylo {
        swap_type: HyloSwapType,
    },
    VoltrDeposit,
    VoltrWithdraw,
    SanctumSV2 {
        src_lst_value_calc_accs: u8,
        dst_lst_value_calc_accs: u8,
        src_lst_index: u32,
        dst_lst_index: u32,
    },
    LemmingsFi {
        is_base_in: bool,
    },
    ScaleVmmBuy,
    ScaleVmmSell,
    ScaleAmmBuy,
    ScaleAmmSell,
    BisonFiV2 {
        a_to_b: bool,
    },
    Trends,
    HumaDeposit,
    HumaInstantWithdraw,
    Kipseli {
        is_base_to_quote: bool,
    },
    DynamicV2 {
        candidate_swaps: Vec<CandidateSwapWithBps>,
        max_split_quote_calls: u8,
        max_split_candidates: u8,
    },
    PumpSwapBuyV3WithCashbackClaim,
    PumpSwapSellV3WithCashbackClaim,
    PumpWrappedBuyV4WithCashbackClaim,
    PumpWrappedSellV4WithCashbackClaim,
    GoonFiV3 {
        is_bid: bool,
    },
    PumpWrappedBuyV5 {
        claim_cashback: bool,
    },
    PumpWrappedSellV5 {
        claim_cashback: bool,
    },
    ZeroFiSwapV2,
    BisonFiPredict {
        side: Side,
        is_buy: bool,
    },
    ByrealDynamicV3,
    Flux {
        swap_id: u64,
        base_to_quote: bool,
    },
    VaultLiquidSellLst,
    VaultLiquidBuyLst {
        lst_amount: u64,
    },
    KipseliV2 {
        is_base_to_quote: bool,
    },
    // tag 161 — Deriverse (DRVSpZ2YUYYKgZP8XtLhAGtT1zYSCKzeHfb4DgRnrgqD)
    // 必须紧跟 KipseliV2(=160)：Borsh 判别号 = 声明顺序 = 链上 tag 161。
    // 字段布局取自 Deriverse 官方 Jupiter 集成的扩展 Swap enum。
    Deriverse {
        side: Side,
        instr_id: u32,
    },
    // tag 162 — Hadron (HADRoNbLovyqhCsocfYQYB7QdfCAAinN9HTePvBCVDQ8)
    // 必须紧跟 Deriverse(=161)：Borsh 判别号 = 声明顺序 = 链上 tag 162。
    // 字段 = 单个 bool isX（卖 base=true）；amount/minOut/expiration 由 route 上下文注入。
    // 已 IDA 实证 jupiter-v6-0626 deserializer(handler@0x1fb00→loc_1F428): sub_12550=bool 读取器，
    // 仅读 1 个 bool → 1 字段。Jupiter route 经 litesvm 字节级对账通过(raw-Step 池 5/5 EXACT)。
    Hadron {
        is_x: bool,
    },
    // tag 163 — BinaryFi (B72M6nyCLFgWiJtAN4naUTminMiTmyGcEqQHXwVeRdht)。unit variant（IDL 实证）。
    BinaryFi,
    // tag 164 — Metric。占位保判别号对齐，未接入。
    Metric {
        zero_for_one: bool,
    },
    // ── 0721 轮 (jupiter-v6-0721.so, 链上 Jul 15 2026 升级) 新增 tag 165–168 ──
    // program_id 均 IDA 字节实证(常量 dref 唯一 + vanity 自证)；字段布局仅 IDA 线索、
    // 未 litesvm 字节对账，故空 {} 占位保判别号对齐，未接入——勿直接构造上链。
    // 精确 borsh 字段待展开子反序列化器；证据链见 tmp/jupiter/swap_enum_diff_0721.md。
    // tag 165 — JupiterLendDexSwap (jupZ4m2GqUCJ5iueMfzQf8khFfH31d4XAQt3RzCT9Vd，复用 Jupiter Lend)
    // deser@0x1F590: 枚举判别(0x84–0x86) + sub_0x206B8；字段待对账。
    JupiterLendDexSwap {},
    // tag 166 — Gatorswap (gatorLx9aC1e5ZWAXscv5QRKiLXnLPLXjftVc81h1Hr)
    // deser@0x1FCD0: index(<4) + 两字段(sub_0x18C128)；字段待对账。
    Gatorswap {},
    // tag 167 — Flint (FLiNTXPwppyoJabCoxc2uiiRygAHpmMXajiDXo2Ub1z)
    // deser@0x1FBA8: bool taker_buy/is_global + index(<4)；字段待对账。
    Flint {},
    // tag 168 — Denali (DNL1tgEj3nJovHw9jtyCCQD3arssCJzkmpDizknwzey4)
    // deser@0x1FA48: 两枚举判别(0x99–0x9f / 0x87–0x88) + 子反序列化器；字段待对账。
    Denali {},
}

#[derive(BorshSerialize, BorshDeserialize, Copy, Clone, PartialEq, Eq, Debug)]
pub enum AccountsType {
    TransferHookA,
    TransferHookB,
    TransferHookReward,
    TransferHookInput,
    TransferHookIntermediate,
    TransferHookOutput,
    SupplementalTickArrays,
    SupplementalTickArraysOne,
    SupplementalTickArraysTwo,
}

#[derive(BorshSerialize, BorshDeserialize, Copy, Clone, PartialEq, Eq, Debug)]
pub enum DefiTunaAccountsType {
    TransferHookA,
    TransferHookB,
    TransferHookInput,
    TransferHookIntermediate,
    TransferHookOutput,
    SupplementalTickArrays,
    SupplementalTickArraysOne,
    SupplementalTickArraysTwo,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, Debug, PartialEq)]
pub struct RemainingAccountsSlice {
    pub accounts_type: u8,
    pub length: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub struct RemainingAccountsInfo {
    pub slices: Vec<RemainingAccountsSlice>,
}

impl TryInto<CandidateSwap> for Swap {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<CandidateSwap, Self::Error> {
        let candidate_swap = match self {
            Swap::HumidiFi {
                swap_id,
                is_base_to_quote,
            } => CandidateSwap::HumidiFi {
                swap_id,
                is_base_to_quote,
            },
            Swap::TesseraV { side } => CandidateSwap::TesseraV { side },
            Swap::HumidiFiV2 {
                swap_id,
                is_base_to_quote,
            } => CandidateSwap::HumidiFiV2 {
                swap_id,
                is_base_to_quote,
            },
            _ => return Err(anyhow!("Swap {self:?} is not a valid candidate swap")),
        };
        Ok(candidate_swap)
    }
}
