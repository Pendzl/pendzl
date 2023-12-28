use pendzl::traits::DefaultEnv;
use scale::{Decode, Encode};

#[derive(Default, Debug, Encode, Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct VestingSchedule {
    pub start: Timestamp,
    pub end: Timestamp,
    pub amount: Balance,
    pub released: Balance,
}

impl VestingSchedule {
    pub fn collect_releasable_rdown(&mut self) -> Balance {
        let amount_releaseable = self.amount_releaseable_rdown();
        self.released += amount_releaseable;
        amount_releaseable
    }
    pub fn amount_releaseable_rdown(&self) -> Balance {
        let now: Timestamp = Self::env().block_timestamp();
        if self.is_overdue() {
            return self.amount - self.released;
        }
        if now < self.start || self.end == self.start {
            return 0;
        }
        let total_to_release = self.amount * u128::try_from(now - self.start).unwrap()
            / u128::try_from(self.end - self.start).unwrap()
            - 1; //TODO ??
        let amount_releaseable = total_to_release - self.released;
        amount_releaseable
    }
    pub fn is_overdue(&self) -> bool {
        let now: Timestamp = Self::env().block_timestamp();
        now >= self.end
    }
}
