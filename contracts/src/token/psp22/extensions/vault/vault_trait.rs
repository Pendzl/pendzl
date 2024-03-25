// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

/// Trait based on the ERC-4626 "Tokenized Vault Standard", as defined in
/// https://eips.ethereum.org/EIPS/eip-4626.
#[ink::trait_definition]
pub trait PSP22Vault {
    /// Returns the address of the underlying token used for the Vault for accounting,
    /// depositing, and withdrawing.
    ///
    /// - MUST be an PSP22 token contract.
    /// - MUST NOT revert.
    #[ink(message)]
    fn asset(&self) -> AccountId;

    /// Returns the total amount of the underlying asset that is “managed” by Vault.
    ///
    /// - SHOULD include any compounding that occurs from yield.
    /// - MUST be inclusive of any fees that are charged against assets in the Vault.
    /// - MUST NOT revert.
    #[ink(message)]
    fn total_assets(&self) -> Balance;

    /// Returns the amount of shares that the Vault would exchange for the amount of assets provided,
    /// in an ideal scenario where all the conditions are met.
    ///
    /// - MUST NOT be inclusive of any fees that are charged against assets in the Vault.
    /// - MUST NOT show any variations depending on the caller.
    /// - MUST NOT reflect slippage or other on-chain conditions, when performing the actual exchange.
    /// - MUST NOT revert.
    ///
    /// NOTE: This calculation MAY NOT reflect the “per-user” price-per-share, and instead should reflect the
    /// “average-user’s” price-per-share, meaning what the average user should expect to see when exchanging to and
    /// from.
    #[ink(message)]
    fn convert_to_shares(
        &self,
        assets: Balance,
        round: Rounding,
    ) -> Result<Balance, MathError>;

    /// Returns the amount of assets that the Vault would exchange for the amount of shares provided,
    /// in an ideal scenario where all the conditions are met.
    ///
    /// - MUST NOT be inclusive of any fees that are charged against assets in the Vault.
    /// - MUST NOT show any variations depending on the caller.
    /// - MUST NOT reflect slippage or other on-chain conditions, when performing the actual exchange.
    /// - MUST NOT revert.
    ///
    /// NOTE: This calculation MAY NOT reflect the “per-user” price-per-share, and instead should reflect the
    /// “average-user’s” price-per-share, meaning what the average user should expect to see when exchanging to and
    /// from.
    #[ink(message)]
    fn convert_to_assets(
        &self,
        shares: Balance,
        round: Rounding,
    ) -> Result<Balance, MathError>;

    /// Returns the maximum amount of the underlying asset that can be deposited into the Vault for the receiver,
    /// through a deposit call.
    ///
    /// - MUST return a limited value if receiver is subject to some deposit limit.
    /// - MUST return 2 ** 256 - 1 if there is no limit on the maximum amount of assets that may be deposited.
    /// - MUST NOT revert.
    #[ink(message)]
    fn max_deposit(&self, to: AccountId) -> Balance;

    /// Returns the maximum amount of the Vault shares that can be minted for the receiver, through a mint call.
    ///
    /// - MUST return a limited value if receiver is subject to some mint limit.
    /// - MUST return 2 ** 256 - 1 if there is no limit on the maximum amount of shares that may be minted.
    /// - MUST NOT revert.
    #[ink(message)]
    fn max_mint(&self, to: AccountId) -> Balance;

    /// Returns the maximum amount of the underlying asset that can be withdrawn from the owner balance in the
    /// Vault, through a withdraw call.
    ///
    /// - MUST return a limited value if owner is subject to some withdrawal limit or timelock.
    /// - MUST NOT revert.
    #[ink(message)]
    fn max_withdraw(&self, owner: AccountId) -> Balance;

    /// Returns the maximum amount of Vault shares that can be redeemed from the owner balance in the Vault,
    /// through a redeem call.
    ///
    /// - MUST return a limited value if owner is subject to some withdrawal limit or timelock.
    /// - MUST return balanceOf(owner) if owner is not subject to any withdrawal limit or timelock.
    /// - MUST NOT revert.
    #[ink(message)]
    fn max_redeem(&self, owner: AccountId) -> Balance;

    /// Allows an on-chain or off-chain user to simulate the effects of their deposit at the current block, given
    /// current on-chain conditions.
    ///
    /// - MUST return as close to and no more than the exact amount of Vault shares that would be minted in a deposit
    ///   call in the same transaction.
    /// - MUST NOT account for deposit limits like those returned from maxDeposit and should always act as though the
    ///   deposit would be accepted, regardless if the user has enough tokens approved, etc.
    /// - MUST be inclusive of deposit fees.
    /// - MUST NOT revert.
    ///
    /// NOTE: any unfavorable discrepancy between convertToShares and previewDeposit SHOULD be considered slippage in
    /// share price or some other type of condition, meaning the depositor will lose assets by depositing.
    #[ink(message)]
    fn preview_deposit(&self, assets: Balance) -> Result<Balance, MathError>;

    /// Allows an on-chain or off-chain user to simulate the effects of their mint at the current block, given
    /// current on-chain conditions.
    ///
    /// - MUST return as close to and no fewer than the exact amount of assets that would be deposited in a mint call
    ///   in the same transaction.
    /// - MUST NOT account for mint limits like those returned from maxMint and should always act as though the mint
    ///   would be accepted, regardless if the user has enough tokens approved, etc.
    /// - MUST be inclusive of deposit fees.
    /// - MUST NOT revert.
    ///
    /// NOTE: any unfavorable discrepancy between convertToAssets and previewMint SHOULD be considered slippage in
    /// share price or some other type of condition, meaning the depositor will lose assets by minting.
    #[ink(message)]
    fn preview_mint(&self, shares: Balance) -> Result<Balance, MathError>;

    /// Allows an on-chain or off-chain user to simulate the effects of their withdrawal at the current block,
    /// given current on-chain conditions.
    ///
    /// - MUST return as close to and no fewer than the exact amount of Vault shares that would be burned in a withdraw
    ///   call in the same transaction.
    /// - MUST NOT account for withdrawal limits like those returned from maxWithdraw.
    /// - MUST be inclusive of withdrawal fees.
    /// - MUST NOT revert.
    ///
    /// NOTE: any unfavorable discrepancy between convertToShares and previewWithdraw SHOULD be considered slippage in
    /// share price or some other type of condition, meaning the depositor will lose assets by withdrawing.
    #[ink(message)]
    fn preview_withdraw(&self, assets: Balance) -> Result<Balance, MathError>;

    /// Allows an on-chain or off-chain user to simulate the effects of their redemption at the current block,
    /// given current on-chain conditions.
    ///
    /// - MUST return as close to and no more than the exact amount of assets that would be withdrawn in a redeem call
    ///   in the same transaction.
    /// - MUST NOT account for redemption limits like those returned from maxRedeem.
    /// - MUST be inclusive of withdrawal fees.
    /// - MUST NOT revert.
    ///
    /// NOTE: any unfavorable discrepancy between convertToAssets and previewRedeem SHOULD be considered slippage in
    /// share price or some other type of condition, meaning the depositor will lose assets by redeeming.
    #[ink(message)]
    fn preview_redeem(&self, shares: Balance) -> Result<Balance, MathError>;

    /// Mints shares Vault shares to receiver by depositing exactly amount of underlying tokens.
    ///
    /// - MUST emit the Deposit event.
    /// - MAY support an additional flow in which the underlying tokens are owned by the Vault contract before the
    ///   deposit execution, and are accounted for during deposit.
    /// - MUST revert if all of assets cannot be deposited.
    ///
    /// NOTE: most implementations will require pre-approval of the Vault with the Vault’s underlying asset token.
    #[ink(message)]
    fn deposit(
        &mut self,
        assets: Balance,
        receiver: AccountId,
    ) -> Result<Balance, PSP22Error>;

    /// Mints exactly shares Vault shares to receiver by depositing amount of underlying tokens.
    ///
    /// - MUST emit the Deposit event.
    /// - MAY support an additional flow in which the underlying tokens are owned by the Vault contract before the mint
    ///   execution, and are accounted for during mint.
    /// - MUST revert if all of shares cannot be minted.
    ///
    /// NOTE: most implementations will require pre-approval of the Vault with the Vault’s underlying asset token.
    #[ink(message)]
    fn mint(
        &mut self,
        shares: Balance,
        receiver: AccountId,
    ) -> Result<Balance, PSP22Error>;

    /// Burns shares from owner and sends exactly assets of underlying tokens to receiver.
    ///
    /// - MUST emit the Withdraw event.
    /// - MAY support an additional flow in which the underlying tokens are owned by the Vault contract before the
    ///   withdraw execution, and are accounted for during withdraw.
    /// - MUST revert if all of assets cannot be withdrawn.
    ///
    /// Note that some implementations will require pre-requesting to the Vault before a withdrawal may be performed.
    /// Those methods should be performed separately.
    #[ink(message)]
    fn withdraw(
        &mut self,
        assets: Balance,
        receiver: AccountId,
        owner: AccountId,
    ) -> Result<Balance, PSP22Error>;

    /// Burns exactly shares from owner and sends assets of underlying tokens to receiver.
    ///
    /// - MUST emit the Withdraw event.
    /// - MAY support an additional flow in which the underlying tokens are owned by the Vault contract before the
    ///   redeem execution, and are accounted for during redeem.
    /// - MUST revert if all of shares cannot be redeemed (due to withdrawal limit being reached, slippage, the owner
    ///   not having enough shares, etc).
    ///
    /// NOTE: Some implementations will require pre-requesting to the Vault before a redemption may be performed.
    /// Those methods should be performed separately. This function is critical for the overall ERC-4626 standard
    /// compliance as it provides a mechanism for share redemption, aligning with the standard's emphasis on flexibility
    /// and efficiency in tokenized vault interactions. The implementation should ensure accurate and secure handling
    /// of shares and underlying assets, respecting all applicable limits and conditions as outlined in the standard.
    #[ink(message)]
    fn redeem(
        &mut self,
        shares: Balance,
        receiver: AccountId,
        owner: AccountId,
    ) -> Result<Balance, PSP22Error>;
}

/// trait that is derived by Pendzl Pausable implementation macro assuming StorageFieldGetter<PSP22VaultStorage> is implemented
///
/// functions of this trait are recomended to use while writing ink::messages
pub trait PSP22VaultInternal {
    /// Provides an offset for decimals, used in internal calculations.
    ///
    /// - Returns a fixed decimal offset value. Override as needed for specific implementations.
    fn _decimals_offset(&self) -> u8;

    /// Attempts to fetch the asset decimals.
    ///
    /// - Returns a tuple of a boolean and a uint8. The boolean indicates success, and the uint8 represents the decimals.
    /// - A return value of false indicates that the attempt failed in some way.
    fn _try_get_asset_decimals(&self) -> (bool, u8);

    /// returns reference to asset that can be deposited and withdrawn
    fn _asset(&self) -> PSP22Ref;

    fn _total_assets(&self) -> Balance;
    /// Internal conversion function from assets to shares with support for rounding direction.
    ///
    /// - Performs multiplication and division for asset to share conversion with specified rounding.
    fn _convert_to_shares(
        &self,
        assets: &Balance,
        round: Rounding,
    ) -> Result<Balance, MathError>;

    /// Internal conversion function from shares to assets with support for rounding direction.
    ///
    /// - Performs multiplication and division for share to asset conversion with specified rounding.
    fn _convert_to_assets(
        &self,
        shares: &Balance,
        round: Rounding,
    ) -> Result<Balance, MathError>;

    /// doc @ PSP22Vault::max_deposit
    fn _max_deposit(&self, to: &AccountId) -> Balance;

    /// doc @ PSP22Vault::max_mint
    fn _max_mint(&self, to: &AccountId) -> Balance;

    /// doc @ PSP22Vault::max_withdraw
    fn _max_withdraw(&self, owner: &AccountId) -> Balance;
    /// doc @ PSP22Vault::max_redeem
    fn _max_redeem(&self, owner: &AccountId) -> Balance;
    /// doc @ PSP22Vault::preview_deposit
    fn _preview_deposit(&self, assets: &Balance) -> Result<Balance, MathError>;

    /// doc @ PSP22Vault::preview_mint
    fn _preview_mint(&self, shares: &Balance) -> Result<Balance, MathError>;

    /// doc @ PSP22Vault::preview_withdraw

    fn _preview_withdraw(&self, assets: &Balance)
        -> Result<Balance, MathError>;

    /// doc @ PSP22Vault::preview_redeem
    fn _preview_redeem(&self, shares: &Balance) -> Result<Balance, MathError>;

    /// Common workflow for deposit/mint operations.
    ///
    /// - Handles transfer of assets from caller to contract, followed by minting of shares to the receiver.
    /// - Ensures safety against reentrancy attacks when dealing with ERC777 tokens.
    fn _deposit(
        &mut self,
        caller: &AccountId,
        receiver: &AccountId,
        assets: &Balance,
        shares: &Balance,
    ) -> Result<(), PSP22Error>;

    /// Common workflow for withdraw/redeem operations.
    ///
    /// - Verifies allowances, burns shares from the owner, and handles transfer of assets to the receiver.
    /// - Ensures safety against reentrancy attacks when dealing with ERC777 tokens.
    fn _withdraw(
        &mut self,
        caller: &AccountId,
        receiver: &AccountId,
        owner: &AccountId,
        assets: &Balance,
        shares: &Balance,
    ) -> Result<(), PSP22Error>;
}

/// trait that must be implemented by exactly one storage field of a contract storage
/// together with PSP22Storage so the Pendzl PSP22VaultInternal and PSP22Vault implementation can be derived.
pub trait PSP22VaultStorage {
    fn asset(&self) -> PSP22Ref;

    fn underlying_decimals(&self) -> u8;
}
