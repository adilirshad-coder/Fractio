pub const GLOBAL_SEED: &[u8] = b"global";
pub const PROJECT_SEED: &[u8] = b"project";
pub const OFFERING_SEED: &[u8] = b"offering";
pub const POSITION_SEED: &[u8] = b"position";
pub const SEED_VAULT_SEED: &[u8] = b"seed-vault";
pub const CURVE_SEED: &[u8] = b"curve";
pub const ESCROW_SEED: &[u8] = b"escrow";
pub const MILESTONE_SEED: &[u8] = b"milestone";
pub const GRADUATION_SEED: &[u8] = b"graduation";
pub const ALLOCATION_SEED_BPS: u16 = 1_500;
pub const ALLOCATION_CURVE_BPS: u16 = 4_500;
pub const ALLOCATION_CAPITAL_BPS: u16 = 1_500;
pub const ALLOCATION_OPEN_MARKET_BPS: u16 = 2_500;
pub const OWNERSHIP_CAP_BPS: u16 = 500;
pub const SEED_BACKING_BPS: u16 = 8_000;
pub const SEED_VESTING_SECONDS: i64 = 365 * 86_400;
pub const SEED_VESTING_CADENCE_SECONDS: i64 = 86_400;
pub const MIN_TOTAL_SUPPLY: u64 = 100_000_000;
pub const MAX_TOTAL_SUPPLY: u64 = 1_000_000_000;

pub const fn valid_total_supply(total_supply: u64) -> bool {
    matches!(total_supply, MIN_TOTAL_SUPPLY | MAX_TOTAL_SUPPLY)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn standard_allocation_sums_to_ten_thousand() {
        assert_eq!(ALLOCATION_SEED_BPS + ALLOCATION_CURVE_BPS + ALLOCATION_CAPITAL_BPS + ALLOCATION_OPEN_MARKET_BPS, 10_000);
    }
    #[test]
    fn supply_presets_are_enforced() {
        assert!(valid_total_supply(MIN_TOTAL_SUPPLY));
        assert!(valid_total_supply(MAX_TOTAL_SUPPLY));
        assert!(!valid_total_supply(100_000_001));
        assert!(!valid_total_supply(0));
        assert!(!valid_total_supply(u64::MAX));
    }
}
