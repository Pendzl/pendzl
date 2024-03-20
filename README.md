## Summary

**Pendzl is a library for smart contract development on ink!.**

Why use this library?

- To make contracts **interoperable** to do **safe** cross-contract calls (by having the same function's signature among every contract)
- To ensure the usage of [Polkadot Standards Proposals](https://github.com/w3f/PSPs)
- To ensure the usage of the **latest & most secure** implementation
- Useful contracts that provide custom logic to be implemented in contracts
- To **save time** by not writing boilerplate code
- Useful features that can simplify development
- All contracts are upgradeable by default

Which Standard tokens & useful contracts does it provide?

- **PSP22** - Fungible Token (_ERC20 equivalent_) with some extensions including Vault - a modified ERC-4626 contract!
- **PSP34** - Non-Fungible Token (_ERC721 equivalent_) with some extensions
- (_not yet supported_) **PSP37** - _ERC1155 equivalent_ with extensions
- **Ownable** Restrict access to action for non-owners
- **Access Control** Define a set of roles and restrict access to action by roles
- **Pausable** Pause/Unpause the contract to disable/enable some operations
- (_not yet supported_) **Timelock Controller** Execute transactions with some delay
- (_not yet supported_) **Governor** Govern
- **General Vester** Allows for the creation of a vests

This library was created based on ideas of implementation macro, Storage trait, and storage_item macro that came from [openbrush-contracts](https://github.com/Brushfam/openbrush-contracts).

### How does it work?

Library allows for reusing implementations of the supported traits for any ink! smart-contract. It's goal is to be **modular** and extensible without unnecessary complexity overhead.

Deriving implementations of provided traits is easy. For example, in case of a PSP22:

add a field of of type PSP22Storage and annotate it with #[storage_field]
add StorageFieldGetter to storage struct's derive macro (with the above allows for access to the storage item via .data())
make your contract reuse the PSP22 implementation via #[pendzl::implementation(PSP22)] annotation on top of your contract!
The first two steps are to satisfy required boundaries by default implementations - contract's storage must implement StorageFieldGetter with appropariate generic T (in this case it's PSP22Storage).

```rust
#[pendzl::implementation(PSP22)]
#[ink::contract]
pub mod my_psp22 {
    .
    .
    .
    #[ink(storage)]
    #[derive(StorageFieldGetter)]
    pub struct Contract {
        #[storage_field]
        psp22: PSP22Data,
    }
    .
    .
    .
}
```

The #[pendzl::implementation(PSP22)] macro implements the PSP22 by including the following code after the macro is expanded:

```rust
impl  pendzl::contracts::psp22::PSP22Internal for Contract {
            fn _total_supply(&self) -> Balance {
                pendzl::contracts::psp22::implementation::PSP22InternalDefaultImpl::_total_supply_default_impl(self)
            }
            .
            .
            .
}

impl pendzl::contracts::psp22::PSP22 for Contract {
        #[ink(message)]
        fn total_supply(&self) -> Balance {
            pendzl::contracts::psp22::implementation::PSP22DefaultImpl::total_supply_default_impl(self)
        }
        .
        .
        .
}
```

One can override the default_impl of functions from PSP22 and/or PSP22Internal as follows:

```rust
    const HATED_ACCOUNT: AccountId = <some_account>

    #[overrider(PSP22Internal)]
    fn _update(
        &mut self,vault::PSP22VaultInternalDefaultImpl,
        /// one can use default_impl as in this example or provide completely new implementation.
        pendzl::contracts::psp22::implementation::PSP22InternalDefaultImpl::_update_default_impl(self, from, to, amount)
    }

    #[overrider(PSP22)]
    fn approve(
        &mut self,
        spender: AccountId,
        value: Balance,
    ) -> Result<(), PSP22Error> {
        if spender == HATED_ACCOUNT {
            return Err(PSP22Error::Custom(String::from("Hated account can not have allowance to spend tokens")));
        }
        /// one can use default_impl as in this example or provide completly new implementation.
        pendzl::contracts::psp22::implementation::PSP22DefaultImpl::approve(self, spender, value)
    }
```

the above overrider functions (marked with #[overrider(...)] ) will be consumed by the #[pendzl::implementation(PSP22)]. As result, the body of the functions will be used to implement the apropariate function in apropariate trait.
As showed in above example, one can still use the default implementation if one only wants to add some logic before/after the default implementation or one can fully reimplement the function.

Similar logic applies to storage items. One may not want to use the default PSP22Data and use his PSP22CustomData. This can be achieved in two ways:

- by implementing PSP22Storage trait for PSP22CustomData,
- overriding all functions from PSP22Internal trait

## Installation & Testing

To work with the project you need to install ink! toolchain and NodeJS's dependencies.

1. you need an installer [rustup](https://www.rust-lang.org/tools/install).
2. [ink! toolchain](https://use.ink/getting-started/setup)
3. NodeJS deps you can install via `pnpm` command inside tests/ folder

### Build

To build exapmles use

```
$ bash build_examples.sh
```

### Tests

If you want to run tests enter the tests/ foldr and run

```
$ pnpm i
$ pnpm build:debug
$ pnpm test
```

## FAQ

### Was it audited?

Contracts in this repository have not yet been audited and may contain several vulnerabilities.

## License

pendzl is released under the [MIT License](LICENSE).
