## Summary
**pendzl is a library for smart contract development on ink!.**

Why use this library?
- To make contracts **interoperable** to do **safe** cross-contracts calls (by having the same functions signature among every contracts)
- To ensure the usage of [Polkadot Standards Proposals](https://github.com/w3f/PSPs)
- To ensure the usage of the **latest & most secure** implementation
- Useful contracts that provide custom logic to be implemented in contracts
- To **save time** by not writing boilerplate code
- Useful features which can simplify development
- All contracts are upgradeable by default

Which Standard tokens & useful contracts does it provide?
- **PSP22** - Fungible Token (*ERC20 equivalent*) with extensions
- **PSP34** - Non-Fungible Token (*ERC721 equivalent*) with extensions
- **PSP37** - *ERC1155 equivalent* with extensions
- **Ownable** Restrict access to action for non-owners
- **Access Control** Define set of roles and restrict access to action by roles
- **Reentrancy guard** Prevent reentrant calls to a function (not supported yet)
- **Pausable** Pause/Unpause the contract to disable/enable some operations
- **Timelock Controller** Execute transactions with some delay (not supported yet)
- **Payment Splitter** Split amount of native tokens between participants (not supported yet)

### Additional stuff

<!-- - You can use [`test_utils`](https://github.com/Brushfam/pendzl-contracts/blob/main/lang/src/test_utils.rs#L39)
to simplify unit testing of you code.
- You can use [`traits`](https://github.com/Brushfam/pendzl-contracts/blob/main/lang/src/traits.rs) that provides some additional
functionality for your code. -->
<!-- - Read our **documentation** in [doc](https://learn.brushfam.io/docs/pendzl). -->
<!-- - Go through our **examples** in [examples](examples) to check hot to use the library and ink!.
- Check the [**example of project struct**](https://github.com/Brushfam/pendzl-contracts/tree/main/example_project_structure) and [according documentation](https://learn.brushfam.io/docs/pendzl/smart-contracts/example/overview). -->

<!-- Not sure where to start? Use [the interactive generator](https://pendzl.io) to bootstrap your contract and learn about the components offered in pendzl. -->

### ‼️ Important ‼️

Events are not supported currently due to how ink! currently handles them.  This will be changeded with ink!-5.0
The identifiers of events must be based on the name of the trait. At the moment, ink! doesn't support it,
but it must be fixed with this [issue](https://github.com/paritytech/ink/issues/809).

### Issues to be resolved before the library becomes production-ready:
* [Event's identifiers are based on the naming of the storage structure](https://github.com/Brushfam/pendzl-contracts/issues/2)

<!-- ## Roadmap 🚗

Current pendzl Roadmap includes: https://docs.google.com/document/d/1b49juyKJN0W-UBHoJ4iS3P_I0Z5a94YoNLxylIf-As8 -->

## Installation & Testing
To work with project you need to install ink! toolchain and NodeJS's dependencies.

1. So, you need an actual installer [rustup](https://www.rust-lang.org/tools/install).
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

After you can run tests by `npm run test` command. It will build all contracts required for integration tests and run them.

## FAQ

### Was it audited?

Contracts in this repository have not yet been audited and contain several vulnerabilities due to the specific of the ink!. The goal is to have it done after ink! 5 release.

## License

pendzl is released under the [MIT License](LICENSE).
