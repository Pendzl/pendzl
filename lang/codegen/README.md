## 1. Code Generation Module

This module contains the implementations of macros used for code generation in the library.

- **Contains:**

  - **Implementation of the `codegen::implementation` macro**:
    - Implements the `implementation` macro, which injects default fn implementations (& overrides) of standard traits.
  - **Boilerplate code for implementations injecting**:
    - Every implementation of a trait (PSP22, PSP34, PSP22Metadata etc) is injected with the default implementation of the trait.
  - **Implementation of the `codegen::storage_item` macro:**
    - Implements functionality for the `storage_item` macro, handling storage-related code generation.
  - **Implementation of the `codegen::storage_field_getter_derive` macro:**
    - Contains the implementation for deriving the `StorageFieldGetter` trait for storage structures.

### How does it work?

## Macro Implementation: The `generate` Function

The `generate` function serves as the core entry point for the procedural macro, processing an ink! module, injecting default implementations for specified traits & handling overridden methods.

### Parsing Attributes and Module Input

The function begins by parsing the attributes provided to the macro to determine which traits require default implementations. The attributes are expected to be a list of trait names, such as `#[implementation(PSP22, PSP34)]`. These are parsed using the `syn::parse2::<AttributeArgs>` function, which results in a vector of trait names (`to_inject_default_impls_vec`).

### Identifying the Storage Struct

The storage struct is the central data structure for an ink! contract, representing the contract's state. The `extract_storage_struct_name` function iterates over the module's items to find the struct annotated with `#[ink(storage)]`. Once found, it extracts the struct's identifier (name) for later use in implementing traits.

### Processing Overridden Methods

Users can provide custom implementations for specific trait methods by annotating functions with `#[overrider(TraitName)]`. The `consume_overriders` function searches for these annotated functions, extracts them, and removes them from the module's items to prevent duplication. It collects these overrides into an `OverridenFnMap`, mapping trait names to the corresponding overridden functions.

### Injecting Trait Implementations

For each trait specified in the attributes (eg. for pendzl::implementation(PSP22, Pausable) would be PSP22 and Pausable), it calls the corresponding implementation function (e.g., `impl_psp22`, `impl_pausable`), passing `impl_args` as the context. These implementation functions generate the default implementations of the traits, modify the module's items, and handle any necessary imports.
Note that all of the impl_XYZ functions follow the same pattern - injecting default implementations for internal and public traits & potentially injecting required imports. Such boilerplate is necessary for simplicity and further DX down the line.

### Cleaning Up Imports

Pendzl's codegen removes unnecessary base trait imports when extended traits are present to avoiding potential conflicts or redundancy. This optimization ensures that the generated code includes only the necessary imports.
It also auto injects `StorageFieldGetter` trait and any overridden trait implementations collected earlier.

## Trait Implementation Functions

_following section talks about the contents of `implenentation.rs` file_

The Pendzl codegen provides several functions to generate default implementations for specific traits, such as `impl_psp22`, `impl_pausable`, `impl_psp22_vault`, `impl_psp34`, `impl_psp34_burnable`, `impl_ownable`, `impl_vesting`, `impl_set_code_hash`... and more.

. Each of these functions follows a similar pattern:

1. **Context Retrieval:** Accesses the storage struct name and other necessary context from `impl_args`.
2. **Implementation Preparation:** Constructs default implementations for internal traits and public traits, defining methods that either provide default behavior or delegate to internal methods.
3. **Override Handling:** Calls `override_functions` to apply any user-provided overrides to the trait implementations.
4. **Import Management:** Adds necessary imports to `impl_args.imports`, ensuring all required types and traits are available in the generated code.
5. **Updating Module Items:** Appends the constructed implementations to `impl_args.items`.

### Example: Implementing the PSP22 Trait

The `impl_psp22` function injects the default implementation of the `PSP22` trait into the module. It constructs the internal default implementation (`PSP22InternalDefaultImpl`), the internal trait implementation (`PSP22Internal`), the public default implementation (`PSP22DefaultImpl`), and the public trait implementation (`PSP22`). Each of these implementations defines methods that provide default behaviors or delegate to internal methods.

The function then calls `override_functions` to handle any overrides provided by the user for both `PSP22Internal` and `PSP22`. It manages necessary imports, such as `pendzl::contracts::psp22::*`, and appends the implementations to the module items.

## Storage Item Handling

The `storage_item` function implements the `#[pendzl::storage_item]` macro, preparing structs or enums to be part of the contract's storage. It allows the use of the `#[lazy]` attribute to mark fields that should be lazily loaded and wrapped in `::ink::storage::Lazy`. The macro also generates constant storage keys for every mapping or lazy field, following recommendations from [ink!'s storage layout documentation](https://use.ink/datastructures/storage-layout).
