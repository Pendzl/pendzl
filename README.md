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
- **PSP22** - Fungible Token (*ERC20 equivalent*) with some extensions including Vault - a modified ERC-4626 contract!
- **PSP34** - Non-Fungible Token (*ERC721 equivalent*) with some extensions
- (*not yet supported*) **PSP37** - *ERC1155 equivalent* with extensions
- **Ownable** Restrict access to action for non-owners
- **Access Control** Define a set of roles and restrict access to action by roles
- **Pausable** Pause/Unpause the contract to disable/enable some operations
- (*not yet supported*) **Timelock Controller** Execute transactions with some delay
- (*not yet supported*) **Governor** Govern
- **General Vester** Allows for the creation of a vests 

This library was created based on ideas of implementation macro, Storage trait, and storage_item macro that came from [openbrush-contracts](https://github.com/Brushfam/openbrush-contracts).


### Additional stuff

<!-- - You can use [`test_utils`](https://github.com/Brushfam/pendzl-contracts/blob/main/lang/src/test_utils.rs#L39)
to simplify unit testing of your code.
- You can use [`traits`](https://github.com/Brushfam/pendzl-contracts/blob/main/lang/src/traits.rs) that provides some additional
functionality for your code. -->
<!-- - Read our **documentation** in [doc](https://learn.brushfam.io/docs/pendzl). -->
<!-- - Go through our **examples** in [examples](examples) to check hot to use the library and ink!.
- Check the [**example of project struct**](https://github.com/Brushfam/pendzl-contracts/tree/main/example_project_structure) and [according documentation](https://learn.brushfam.io/docs/pendzl/smart-contracts/example/overview). -->

<!-- Not sure where to start? Use [the interactive generator](https://pendzl.io) to bootstrap your contract and learn about the components offered in pendzl. -->


<!-- ## Roadmap 🚗

<!-- Current Pendzl Roadmap includes: https://docs.google.com/document/d/1b49juyKJN0W-UBHoJ4iS3P_I0Z5a94YoNLxylIf-As8 -->

## Installation & Testing
To work with the project you need to install ink! toolchain and NodeJS's dependencies.

1. you need an installer [rustup](https://www.rust-lang.org/tools/install).
2. [ink! toolchain](https://use.ink/getting-started/setup)
3. NodeJS deps you can install via `yarn` command

### Build
```
$ yarn build
```
If you want to build in release mode, you can use this command
```
$ yarn build:release
```

### Tests

You can run unit tests by `RUSTFLAGS="-D warnings" cargo test --workspace --features test-all -- --test-threads=10` command from the root of the directory.

After you run tests by the `npm run test` command, it will build all contracts required for integration tests and run them.

## FAQ

### Was it audited?

Contracts in this repository have not yet been audited and may contain several vulnerabilities. 

## License

pendzl is released under the [MIT License](LICENSE).
