// SPDX-License-Identifier: MIT
pub mod errors {

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum MathError {
        Underflow,
        Overflow,
        DivByZero,
    }
}

pub mod operations {

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Rounding {
        Up,
        Down,
    }

    use super::errors::MathError;
    use ethnum::U256;

    /// Performs multiplication followed by division with full precision, handling potential overflows,
    /// and allows for rounding up or down.
    ///
    /// The `mul_div` function computes `(x * y) / denominator` with full precision, ensuring that
    /// intermediate overflows are correctly handled using 256-bit arithmetic (`ethnum::U256`). It also
    /// supports rounding the result up or down based on the `Rounding` parameter.
    ///
    /// # Arguments
    ///
    /// - `x`: The multiplicand (`u128`).
    /// - `y`: The multiplier (`u128`).
    /// - `denominator`: The divisor (`u128`). Must not be zero.
    /// - `round`: Specifies whether to round the result up or down (`Rounding` enum).
    ///
    /// # Returns
    ///
    /// - `Ok(u128)`: The computed result after multiplication and division, possibly rounded.
    /// - `Err(MathError)`: An error if division by zero occurs or if the result overflows `u128`.
    ///
    /// # Errors
    ///
    /// - `MathError::DivByZero`: Returned if `denominator` is zero.
    /// - `MathError::Overflow`: Returned if the result does not fit into `u128`.
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// let result = mul_div(100, 200, 50, Rounding::Down);
    /// assert_eq!(result, Ok(400));
    ///
    /// let result = mul_div(u128::MAX, u128::MAX, 1, Rounding::Down);
    /// assert_eq!(result, Err(MathError::Overflow));
    pub fn mul_div(
        x: u128,
        y: u128,
        denominator: u128,
        round: Rounding,
    ) -> Result<u128, MathError> {
        if denominator == 0 {
            return Err(MathError::DivByZero);
        }

        if x == 0 || y == 0 {
            return Ok(0);
        }

        let x_u256 = U256::from(x);
        let y_u256 = U256::from(y);
        let denominator_u256 = U256::from(denominator);

        // this can not overflow
        let mul_u256 = x_u256.checked_mul(y_u256).unwrap();
        // denom is not 0
        let res_u256: U256 = mul_u256.checked_div(denominator_u256).unwrap();
        let res = match u128::try_from(res_u256) {
            Ok(v) => Ok(v),
            _ => Err(MathError::Overflow),
        }?;

        if round == Rounding::Up && mul_u256 % denominator_u256 != 0 {
            Ok(res.checked_add(1).ok_or(MathError::Overflow)?)
        } else {
            Ok(res)
        }
    }

    #[cfg(test)]
    pub mod test {
        use super::*;
        #[test]
        fn test_mul_div() {
            let x = 1_000_000_000_000_u128;
            assert_eq!(mul_div(x, x, 2 * x, Rounding::Down), Ok(x / 2));
        }

        #[test]
        fn round_up() {
            assert_eq!(mul_div(100, 100, 1000, Rounding::Up), Ok(10));
            assert_eq!(mul_div(101, 100, 1000, Rounding::Up), Ok(11));
            assert_eq!(mul_div(3643, 6393, 11645, Rounding::Up), Ok(2000));
        }

        #[test]
        fn round_down() {
            assert_eq!(mul_div(4000, 2001, 2001, Rounding::Down), Ok(4000));
        }
    }
}
