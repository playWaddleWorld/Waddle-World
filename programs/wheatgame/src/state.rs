use anchor_lang::prelude::*;

// ─────────────────────────────────────────────────────────────────────────────
// World constants
// ─────────────────────────────────────────────────────────────────────────────

pub const WORLD_PLOTS:           u8  = 100;
pub const MAX_PLOTS_PER_WALLET:  u8  = 5;
pub const TRIBE_MAX_MEMBERS:     u8  = 10;
pub const ALLIANCE_MAX_MEMBERS:  u8  = 5;

// Protocol fees, expressed in basis points of 10_000.
pub const FEE_BPS:               u64 = 250;     // 2.5% on filled trades
pub const ABANDON_BURN_BPS:      u64 = 5_000;   // 50% of stake burned
pub const ABANDON_TREASURY_BPS:  u64 = 5_000;   // 50% to treasury

// Yield engine constants.
pub const GOLDEN_HOUR_CYCLE_S:   i64 = 6 * 60 * 60;
pub const GOLDEN_HOUR_WINDOW_S:  i64 = 1 * 60 * 60;
pub const GOLDEN_HOUR_BPS:       u64 = 12_000;       // 1.20×

pub const TRIBE_MEMBER_BPS:           u64 = 11_000;  // 1.10×
pub const TRIBE_LEADER_PER_MEMBER:    u64 = 500;     // +5% per other member
pub const ALLIANCE_BONUS_BPS:         u64 = 10_500;  // 1.05×

pub const HARVEST_COOLDOWN_S:    i64 = 60;           // anti-spam floor

// ─────────────────────────────────────────────────────────────────────────────
// Plot — a position in the world. PDA seed: ["plot", plot_id]
// ─────────────────────────────────────────────────────────────────────────────

#[account]
pub struct Plot {
    pub id:            u8,            // 0..100
    pub owner:         Pubkey,        // Pubkey::default() when unclaimed
    pub tier:          PlotTier,
    pub locked_amount: u64,           // USDC base units locked in vault
    pub claimed_at:    i64,
    pub upgrade_level: u8,            // 1..=4
    pub last_harvest:  i64,
    pub bump:          u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlotTier {
    Bronze,
    Silver,
    Gold,
    Diamond,
}

impl PlotTier {
    pub const fn claim_cost(&self) -> u64 {
        match self {
            PlotTier::Bronze  => 1_000_000,    //  1 USDC
            PlotTier::Silver  => 5_000_000,    //  5 USDC
            PlotTier::Gold    => 15_000_000,   // 15 USDC
            PlotTier::Diamond => 50_000_000,   // 50 USDC
        }
    }

    pub const fn supply(&self) -> u8 {
        match self {
            PlotTier::Bronze  => 50,
            PlotTier::Silver  => 30,
            PlotTier::Gold    => 15,
            PlotTier::Diamond => 5,
        }
    }

    pub const fn yield_bps(&self) -> u64 {
        match self {
            PlotTier::Bronze  => 10_000,
            PlotTier::Silver  => 15_000,
            PlotTier::Gold    => 20_000,
            PlotTier::Diamond => 30_000,
        }
    }
}

pub const fn upgrade_bps(level: u8) -> u64 {
    match level {
        1 => 10_000,
        2 => 12_500,
        3 => 15_000,
        4 => 20_000,
        _ => 10_000,
    }
}

pub const fn upgrade_cost(level: u8) -> u64 {
    match level {
        2 =>  5_000_000,
        3 => 15_000_000,
        4 => 40_000_000,
        _ => 0,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Player — wallet-bound state. PDA seed: ["player", wallet]
// ─────────────────────────────────────────────────────────────────────────────

#[account]
pub struct Player {
    pub wallet:      Pubkey,
    pub plot_count:  u8,
    pub tribe:       Option<Pubkey>,
    pub alliance:    Option<Pubkey>,
    pub created_at:  i64,
    pub bump:        u8,
}

// ─────────────────────────────────────────────────────────────────────────────
// PlotOffer — a peer-to-peer plot listing. PDA seed: ["offer", plot]
// ─────────────────────────────────────────────────────────────────────────────

#[account]
pub struct PlotOffer {
    pub plot:       Pubkey,
    pub seller:     Pubkey,
    pub buyer:      Option<Pubkey>,
    pub price:      u64,
    pub status:     OfferStatus,
    pub created_at: i64,
    pub bump:       u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum OfferStatus { Open, Accepted, Cancelled }

// ─────────────────────────────────────────────────────────────────────────────
// Order — a row in the crop order book. PDA seed: ["order", id]
// ─────────────────────────────────────────────────────────────────────────────

#[account]
pub struct Order {
    pub id:         u64,
    pub maker:      Pubkey,
    pub item:       ItemType,
    pub side:       Side,
    pub quantity:   u64,
    pub remaining:  u64,
    pub price:      u64,
    pub status:     OrderStatus,
    pub created_at: i64,
    pub bump:       u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Side         { Bid, Ask }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum OrderStatus  { Open, Filled, Cancelled }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ItemType {
    Wheat,
    Corn,
    Pumpkin,
    Egg,
    Milk,
    Wool,
    Honey,
    Truffle,
    MagicHerb,
    Fish,
}

// ─────────────────────────────────────────────────────────────────────────────
// Tribe — long-form clan. PDA seed: ["tribe", id]
// ─────────────────────────────────────────────────────────────────────────────

#[account]
pub struct Tribe {
    pub id:           u64,
    pub name:         [u8; 32],
    pub tag:          [u8; 4],
    pub leader:       Pubkey,
    pub home_plot:    Pubkey,
    pub member_count: u8,
    pub invite_code:  [u8; 8],
    pub created_at:   i64,
    pub bump:         u8,
}

#[account]
pub struct TribeMember {
    pub tribe:     Pubkey,
    pub wallet:    Pubkey,
    pub joined_at: i64,
    pub is_leader: bool,
    pub bump:      u8,
}

// ─────────────────────────────────────────────────────────────────────────────
// Alliance — lightweight pact. PDA seed: ["alliance", id]
// ─────────────────────────────────────────────────────────────────────────────

#[account]
pub struct Alliance {
    pub id:           u64,
    pub name:         [u8; 32],
    pub member_count: u8,
    pub created_at:   i64,
    pub bump:         u8,
}

#[account]
pub struct AllianceMember {
    pub alliance:  Pubkey,
    pub wallet:    Pubkey,
    pub joined_at: i64,
    pub bump:      u8,
}

// ─────────────────────────────────────────────────────────────────────────────
// Yield engine — multiplicative composition: tier × upgrade × tribe × alliance × golden.
// ─────────────────────────────────────────────────────────────────────────────

pub fn compute_yield(
    base:            u64,
    tier:            PlotTier,
    upgrade_level:   u8,
    in_tribe:        bool,
    is_tribe_leader: bool,
    tribe_members:   u8,
    in_alliance:     bool,
    golden_hour:     bool,
) -> u64 {
    let tier_bps     = tier.yield_bps();
    let upgrade_bps  = upgrade_bps(upgrade_level);

    let tribe_bps = if !in_tribe {
        10_000
    } else if is_tribe_leader {
        10_000 + TRIBE_LEADER_PER_MEMBER * tribe_members.saturating_sub(1) as u64
    } else {
        TRIBE_MEMBER_BPS
    };

    let alliance_bps = if in_alliance { ALLIANCE_BONUS_BPS } else { 10_000 };
    let golden_bps   = if golden_hour { GOLDEN_HOUR_BPS    } else { 10_000 };

    base
        .saturating_mul(tier_bps)     / 10_000
        .saturating_mul(upgrade_bps)  / 10_000
        .saturating_mul(tribe_bps)    / 10_000
        .saturating_mul(alliance_bps) / 10_000
        .saturating_mul(golden_bps)   / 10_000
}

pub fn is_golden_hour(now: i64) -> bool {
    let phase = now.rem_euclid(GOLDEN_HOUR_CYCLE_S);
    phase < GOLDEN_HOUR_WINDOW_S
}
