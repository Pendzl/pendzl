## Addressing Kudelski Security Audit Findings

### 1.2 KS–PNZ–F–2 Lack of Input Validation in Ownable Library

#### Description

Within the Pendzl library, the `ownable` module provides a standardized approach for managing contract ownership. However, several functions within this module lack comprehensive input validation mechanisms, which are essential for maintaining the contract's integrity and security. Specifically, functions responsible for initializing and modifying ownership do not verify the validity of the provided addresses, potentially leading to unintended consequences.

#### Impact

The absence of input validation in the following functions can have significant repercussions:

- **Affected Functions:**

  - `new`
  - `set_owner`
  - `transfer_ownership_default_impl`
  - `_update_owner_default_impl`

- **Potential Issues:**
  - **Initialization with No Owner:** Without validating the provided address, the contract could be initialized without an owner if the zero-address (`address(0)`) is inadvertently set.
  - **Unnecessary Ownership Transfers:** Transferring ownership to the same address or to the zero-address can lead to redundant operations, increased transaction costs, and confusion due to unnecessary event emissions.

These vulnerabilities compromise the contract's reliability and can be exploited to disrupt its intended functionality.

#### Developer's Response

In the latest OpenZeppelin [Ownable release](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/access/Ownable.sol), input validation occurs exclusively within the `transferOwnership` function. This validation ensures that ownership is not transferred to the zero-address (`address(0)`), which, in Solidity's context, equates to renouncing ownership. The `renounceOwnership` function is specifically designed for this purpose.

However, in the context of ink!, transferring ownership to `address(0)` does not equate to renouncing ownership. Instead, ownership is managed using `Option<AccountId>`, where renouncing ownership is achieved by setting the owner to `None`. Consequently, the Pendzl library's `transfer_ownership` function, which accepts an `AccountId` as the new owner, does not facilitate ownership renouncement by assigning `None`.

Furthermore, the Pendzl library allows for the initialization of a contract without an owner. This flexibility enables scenarios where an instance of an already deployed ownable contract can be created without assigning an owner during initialization.

It is important to highlight that in ink!, the zero-address (`address(0)`) possesses a known private key in cryptographic schemes like sr25519 and ed25519. Assigning ownership to this address poses significant security risks, as entities aware of this private key could potentially gain unauthorized control over the contract. Therefore, while the current implementation does not enforce zero-address verification, users must exercise caution to prevent inadvertent ownership assignments to `address(0)`.

---

### 1.3 KS–PNZ–F–3 Potential Integer Overflow in `_decimals_offset` Usage

#### Description

The `_decimals_offset` function is utilized to calculate an offset that serves as an exponent in power-of-10 operations within the Pendzl library. If the value returned by `_decimals_offset` exceeds 39, it can lead to an integer overflow. This is because the `u128` type in ink!, which stores the result of the power operation, has a maximum value of 2<sup>128</sup> − 1 (approximately 3.4 × 10<sup>38</sup>). Consequently, an offset value of 40 or higher results in a power operation that exceeds the `u128` limit, causing the value to wrap around unexpectedly.

#### Impact

An integer overflow in this context can lead to severe consequences:

- **Incorrect Token Calculations:** Overflowing values can result in erroneous token balance computations, undermining the contract's financial integrity.
- **Contract Instability:** Unexpected behavior due to overflow can compromise the contract's reliability, leading to potential vulnerabilities and exploitation.
- **User Trust Erosion:** Inaccurate token balances and contract instability can erode user trust, impacting the contract's adoption and credibility.

Such vulnerabilities are especially detrimental in blockchain and financial applications where precision and reliability are paramount.

#### Developer's Response

Acknowledged. A usage comment has been added to the `_decimals_offset` function to inform developers of the valid range and prevent potential integer overflows.

To further enhance the safety and reliability of the Pendzl library, the `_decimals_offset` function now includes explicit documentation outlining the permissible range of offset values. Developers are advised to ensure that the offset does not exceed 39 to maintain the integrity of power-of-10 calculations. This precautionary measure serves as a safeguard against unintended integer overflows, thereby preserving accurate token balance computations and preventing contract instability.

Additionally, the function's implementation has been reviewed to ensure that any internal logic generating or utilizing the offset adheres to the defined constraints.

---

### 1.4 KS–PNZ–F–4 Lack of Input Validation in `PSP22` Library

#### Description

The ink! implementation of the `PSP22` token standard within the Pendzl library encompasses several functions responsible for managing token allowances. These functions include `transfer_default_impl`, `transfer_from_default_impl`, `_transfer_default_impl`, `_mint_to_default_impl`, and `_burn_from_default_impl`. Currently, these functions lack comprehensive input validation mechanisms, which are essential for ensuring the contract's integrity and security. The absence of such validations can lead to misuse or unintended behaviors, such as transferring tokens to the zero-address or manipulating allowances in ways that were not intended.

#### Impact

The lack of input validations in the `PSP22` library can have the following consequences:

- **Accidental Token Transfers to Zero-Address:** Without validating recipient addresses, tokens could be inadvertently sent to the zero-address (`address(0)`), effectively burning them unintentionally.
- **Zero-Amount Transfers:** Allowing transfers of zero tokens can trigger events or invoke contract logic without performing any actual token movement. This could lead to unnecessary gas consumption and potential confusion.
- **Redundant Allowance Operations:** Transferring tokens to the same address or manipulating allowances without proper checks can result in redundant operations, increasing transaction costs and cluttering event logs.
- **Security Vulnerabilities:** Malicious actors could exploit the lack of validations to manipulate token balances or allowances in unintended ways, potentially leading to unauthorized token transfers or depletion of token reserves.

**Impacted Functions:**

- **Recipient Address Validations:**

  - `transfer_default_impl`
  - `transfer_from_default_impl`
  - `_transfer_default_impl`
  - `_mint_to_default_impl`
  - `_burn_from_default_impl`

- **Spender Address Validations:**

  - `approve_default_impl`
  - `_approve_default_impl`
  - `_decrease_allowance_from_to_default_impl`
  - `_increase_allowance_from_to_default_impl`

- **Amount Validations:**
  - `increase_total_supply`
  - `decrease_total_supply`
  - `increase_balance_of`
  - `decrease_balance_of`
  - `increase_allowance`
  - `decrease_allowance`
  - `set_allowance`

#### Developer's Response

There is no need to verify transfers with an amount of zero, as it does not constitute a security vulnerability and would unnecessarily increase the gas cost of transactions. The [OpenZeppelin ERC20](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/ERC20.sol) implementation similarly does not validate zero-amount inputs.

Regarding zero-address validations, in ink!, the zero-address (`address(0)`) is not used for burning funds; instead, `Option::None` serves this purpose. As a result, verifying against `address(0)` is not necessary within the Pendzl library, since `address(0)` does not hold the same semantic significance as in Solidity-based implementations. This design choice ensures that roles and token transfers are managed using appropriate and secure mechanisms inherent to the ink!.

---

### 2.1 KS–PNZ–O–1 Zero-Address Not Checked

#### Description

It has been observed that the Pendzl library does not perform verifications against the zero-address (`address(0)`). This omission allows roles to be assigned to the zero-address or tokens to be transferred to it. Given that the zero-address has a known private key in cryptographic schemes like sr25519 and ed25519, this could lead to unauthorized assignments and transfers, posing significant security risks.

The decision to exclude zero-address checks was intentional. The developers believe that ensuring the correct assignment of roles and token transfers to valid addresses is the user's responsibility. This approach contrasts with OpenZeppelin's standard library for Ethereum smart contracts, where zero-address verification is enabled by default, with users having the option to disable it. In Pendzl, the default stance is the opposite.

#### Developer's Response

In ink! Lang, the zero-address (`address(0)`) is not equivalent to Solidity's `address(0)`. In Solidity, the private key of `address(0)` is unknown, and it is commonly used in events to indicate that something (tokens, ownership, access) was burned or revoked. However, in ink! and the Pendzl library, there is no such problem because `Option::None` is used to denote that something has been burned or revoked.

Assigning roles or transferring tokens to the zero-address in ink! does not carry the same semantics as in Solidity. Instead, developers should utilize `Option<AccountId>` to manage ownership and role assignments effectively. This design ensures that the zero-address does not inadvertently become a point of vulnerability due to its association with a known private key.

Moreover, in cryptographic schemes like sr25519 and ed25519, the zero-address possesses a known private key. Assigning roles or transferring tokens to this address could allow malicious entities to exploit the contract by leveraging the known private key to gain unauthorized access or manipulate token balances. Therefore, while the Pendzl library does not enforce zero-address checks by default, developers must be vigilant in ensuring that roles and token transfers are assigned to valid and secure addresses. Utilizing `Option<AccountId>` provides a robust and type-safe mechanism to prevent such vulnerabilities, maintaining the contract's security and integrity.

---

### 2.4 KS–PNZ–O–4 Lack of Access Control in Pausable System

#### Description

The Pendzl library introduces standard and default methods for pausing and unpausing a system, specifically through the functions `_pause_default_impl` and `_unpause_default_impl`. However, these methods currently lack access control mechanisms to restrict their invocation. Without such controls, any account could potentially pause or unpause the system, leading to misuse or unintended disruptions in the system's operation.

#### Developer's Response

The `Pausable` trait in the Pendzl library includes only one non-mutable (view) method, `paused`, which returns `true` if the contract is paused and `false` otherwise. The pausable modules provide internal methods `_pause` and `_unpause` that developers can utilize to control the contract's paused state. Importantly, the module itself does not expose public messages to pause or unpause the contract.

This design ensures that access control over pausing and unpausing the system is delegated to the developer. By keeping these methods internal, the library allows developers to integrate their own access control mechanisms, such as role-based permissions, to govern who can invoke `_pause` and `_unpause`. This approach provides flexibility and security, enabling developers to tailor the pausable functionality to their specific contract requirements without imposing rigid access control policies.

For instance, developers can integrate the `Ownable` trait to restrict pausing and unpausing operations to the contract owner:

```rust
impl Ownable for MyContract {
    #[ink(message)]
    fn pause(&mut self) {
        assert!(self.owner == Some(self.env().caller()), "Only the owner can pause the contract.");
        self._pause();
    }

    #[ink(message)]
    fn unpause(&mut self) {
        assert!(self.owner == Some(self.env().caller()), "Only the owner can unpause the contract.");
        self._unpause();
    }
}
```

By implementing such access controls, developers ensure that only authorized accounts can alter the contract's paused state, thereby safeguarding against unauthorized disruptions and maintaining the contract's intended functionality.

---

### 2.6 KS–PNZ–O–6 Potential Reentrancy Vulnerability

#### Description

Reentrancy is a vulnerability that occurs when a function can be interrupted during execution and called again before the first call is finished. This can lead to unexpected behavior, such as funds being withdrawn multiple times in a single transaction. In the context of this code, the `_withdraw_default_impl` and `_deposit_default_impl` functions are potentially vulnerable to reentrancy attacks. Both functions call `self._asset().transfer` or `self._asset().transfer_from` (which are external calls) and then change the state of the contract with `self._mint_to(receiver, shares)` or `self._burn_from(owner, shares)`. If the `transfer` or `transfer_from` functions are compromised, they could call back into `_withdraw_default_impl` or `_deposit_default_impl` and reenter the function before the state changes have been committed.

#### Developer's Response

To mitigate potential reentrancy vulnerabilities, developers have the option to override the `_withdraw_default_impl` and `_deposit_default_impl` functions. The current implementation is designed to be more gas-efficient than recommended patterns and does not incorporate additional security mechanisms beyond the inherent safeguards. The recommended reentrancy protection methods, while widely adopted, do not provide additional security advantages in the context of the Pendzl library's implementation.

By allowing developers to override these functions, the Pendzl library offers the flexibility to implement custom reentrancy protections if deemed necessary. This approach ensures that the library remains efficient while providing the means for developers to enhance security based on their specific use cases and threat models.

For example, developers can implement a simple reentrancy guard using a mutex pattern:

```rust
use std::sync::Mutex;

struct MyContract {
    reentrancy_lock: Mutex<bool>,
    // other fields
}

impl MyContract {
    fn _withdraw_default_impl(&mut self, receiver: AccountId, shares: Balance) -> Result<(), Error> {
        let mut lock = self.reentrancy_lock.lock().unwrap();
        if *lock {
            return Err(Error::ReentrancyDetected);
        }
        *lock = true;

        // External call
        self._asset().transfer(receiver, shares)?;

        // State change
        self._mint_to(receiver, shares)?;

        *lock = false;
        Ok(())
    }

    fn _deposit_default_impl(&mut self, owner: AccountId, shares: Balance) -> Result<(), Error> {
        let mut lock = self.reentrancy_lock.lock().unwrap();
        if *lock {
            return Err(Error::ReentrancyDetected);
        }
        *lock = true;

        // External call
        self._asset().transfer_from(owner, shares)?;

        // State change
        self._burn_from(owner, shares)?;

        *lock = false;
        Ok(())
    }
}
```

This example demonstrates how developers can introduce a reentrancy guard to prevent multiple simultaneous calls to sensitive functions.

---

### 2.7 KS–PNZ–O–7 Lack of Functionality to Revoke Allowances in PSP22 Contract

#### Description

The PSP22 token contract in the Pendzl library includes functions for managing allowances, such as `_approve_default_impl`, `_decrease_allowance_from_to_default_impl`, and `_increase_allowance_from_to_default_impl`. However, it currently lacks a dedicated function for revoking allowances. While users can set allowances to zero using the `_approve_default_impl` or `_decrease_allowance_from_to_default_impl` functions, the absence of explicit revocation functions like `_revoke_allowance` or `_revoke_all_allowances` can lead to potential vulnerabilities and inefficiencies.

**Potential Risks:**

- **Malicious Spender Exploitation:** If a user grants an allowance to a malicious spender and later acquires additional funds, the spender could exploit the existing allowance to drain these new funds, especially if the allowance was not explicitly revoked.
- **User Confusion:** Relying on indirect methods to revoke allowances can be less intuitive for users, increasing the likelihood of accidental misuse or overlooking the need to revoke allowances.

#### Developer's Response

Allowance revocation can be effectively managed using the existing `set_allowance` function by passing an allowance amount of zero. This approach aligns with standard practices observed in widely adopted token implementations, such as OpenZeppelin's ERC20 contracts, which do not provide dedicated functions solely for revoking allowances.

**Implementation Example:**

```rust
// Revoking a single allowance
self.set_allowance(spender_address, 0)?;

// Revoking all allowances (if supported)
for spender in self.get_all_spenders() {
    self.set_allowance(spender, 0)?;
}
```

By utilizing the set_allowance function to set allowances to zero, users can effectively revoke permissions granted to spenders without the need for additional specialized functions. This method maintains the contract's simplicity and gas efficiency while providing the necessary functionality to manage allowances securely.

---
