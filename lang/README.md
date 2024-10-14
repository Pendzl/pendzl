# Pendzl functionality and utils

The library is organized into three main components

## 1. Code Generation Module ([`codegen`](../pendzl_lang_codegen))

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

See more about the code generation module [here](../pendzl_lang_codegen).

## 2. Macro Definitions Module ([macro](../pendzl_lang_macro))

This module defines the public macros that are exposed to end users.

- **Contains:**
  - **Definition of the `implementation` macro:**
    - Utilizes `codegen::implementation` internally to provide default implementations of standard traits.
  - **Definition of the `storage_item` macro:**
    - Uses `codegen::storage_item` to handle storage-related macros.
  - **Definition of the `StorageFieldGetter` derive macro:**
    - Employs `codegen::storage_field_getter_derive` to automatically derive getter methods for storage fields.

## 3. Source Module (`src`)

This module serves as the main entry point and provides utility functions and re-exports for end users.

- **Features:**
  - **Re-exports the `macro` module:**
    - Makes the macros defined in the `macro` module available to users.
  - **Contains mathematical utilities:**
    - Provides utility functions for mathematical operations used in the library.
  - **Includes various trait and type helpers:**
    - Contains helper traits and types to assist both the library and end users in developing smart contracts.
