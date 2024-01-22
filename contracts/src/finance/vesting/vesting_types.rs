use ink::env::call::{build_call, ExecutionInput};
use ink::env::DefaultEnvironment;
use pendzl::traits::{DefaultEnv, Timestamp};
use scale::{Decode, Encode};

pub type SelectorBytes = [u8; 4];

#[derive(Debug, Encode, PartialEq, Eq, Decode, Clone)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum VestingTimeConstraint {
    Default(Timestamp),
    External(AccountId, SelectorBytes),
}

#[derive(Debug, Encode, Decode, Clone)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct VestingSchedule {
    pub start: VestingTimeConstraint,
    pub end: VestingTimeConstraint,
    pub amount: Balance,
    pub released: Balance,
}

impl VestingSchedule {
    pub fn collect_releasable_rdown(&mut self) -> Result<Balance, VestingError> {
        let amount_releaseable = self.amount_releaseable_rdown()?;
        self.released += amount_releaseable;
        Ok(amount_releaseable)
    }
    pub fn amount_releaseable_rdown(&self) -> Result<Balance, VestingError> {
        let now: Timestamp = Self::env().block_timestamp();
        let start_time = self._extract_timestamp_from_constraint(&self.start)?;
        let end_time = self._extract_timestamp_from_constraint(&self.end)?;
        let is_overdue = self.is_overdue()?;
        if is_overdue {
            return Ok(self.amount - self.released);
        }
        if now < start_time || start_time == end_time {
            return Ok(0);
        }
        let total_to_release = self.amount * u128::try_from(now - start_time).unwrap()
            / u128::try_from(end_time - start_time).unwrap()
            - 1; //TODO ??
        let amount_releaseable = total_to_release - self.released;
        Ok(amount_releaseable)
    }
    pub fn is_overdue(&self) -> Result<bool, VestingError> {
        let now: Timestamp = Self::env().block_timestamp();
        let end_time = self._extract_timestamp_from_constraint(&self.end)?;
        Ok(now >= end_time)
    }
    fn _extract_timestamp_from_constraint(
        &self,
        constraint: &VestingTimeConstraint,
    ) -> Result<Timestamp, VestingError> {
        match *constraint {
            VestingTimeConstraint::Default(timestamp) => Ok(timestamp),
            VestingTimeConstraint::External(account_id, selector_bytes) => {
                let call_result = build_call::<DefaultEnvironment>()
                    .call(account_id)
                    .exec_input(ExecutionInput::new(ink::env::call::Selector::new(
                        selector_bytes,
                    )))
                    .returns::<Timestamp>()
                    .try_invoke();

                match call_result {
                    Ok(timestamp) => match timestamp {
                        Ok(timestamp) => Ok(timestamp),
                        Err(_) => Err(VestingError::CouldNotResolveTimeConstraint),
                    },
                    Err(_) => Err(VestingError::CouldNotResolveTimeConstraint),
                }
            }
        }
    }
}
