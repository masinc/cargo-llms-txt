# cargo-llms-txt

A comprehensive Cargo subcommand that generates `llms.txt` and `llms-full.txt` files from Rust projects for use with Large Language Models (LLMs). Supports all 15 types of Rust public items with advanced code analysis and formatting.

## Features

- **Complete Rust Item Coverage**: Supports all 15 types of Rust public items
- **Comprehensive API Documentation**: Extracts public APIs, documentation comments, and code signatures
- **Dual Output Formats**: 
  - `llms.txt`: Concise overview with table of contents
  - `llms-full.txt`: Complete API documentation with detailed descriptions
- **Advanced Code Analysis**:
  - Actual parameter names (not placeholders)
  - Complete where clause extraction with type bounds
  - Full type name resolution for generics and complex types
  - Detailed enum variant fields (Named, Unnamed, Unit)
  - CFG attribute parsing for conditional compilation
  - FFI function detection with proper extern block formatting
- **Project Metadata**: Extracts version, authors, license, dependencies, and features from `Cargo.toml`

## Installation

```bash
cargo install --path .
```

Or add as a cargo subcommand:

```bash
git clone https://github.com/masinc/cargo-llms-txt
cd cargo-llms-txt
cargo install --path .
```

## Usage

Run in any Rust project directory:

```bash
cargo llms-txt
```

This will generate two files:
- `llms.txt` - Concise project overview and API summary
- `llms-full.txt` - Complete API documentation with detailed descriptions

### Options

```bash
cargo llms-txt [OPTIONS]

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

## Output Format

### llms.txt
- Project metadata (version, authors, license, etc.)
- Dependencies and features list
- API overview with file-by-file summaries
- Links to complete documentation

### llms-full.txt
- Complete table of contents
- Full API documentation for each module
- Function signatures with actual parameter names
- Where clauses with detailed type bounds
- Struct and enum definitions with all fields
- Implementation blocks with method signatures
- CFG attributes for conditional compilation
- Documentation comments and examples

## Example Output

### Function with Advanced Features
```rust
pub fn sample_weighted<R, F, X>(
    rng: &mut R,
    length: usize,
    weight: F,
    amount: usize
) -> Result<IndexVec, WeightError>
where
    R: Rng + Sized,
    F: Fn(usize) -> X,
    X: Into<f64>
```

### FFI Functions with Proper Formatting
```rust
// External function declaration
extern "C" {
    pub fn external_function(x: i32) -> i32;
}

// Exported function with attributes
#[no_mangle]
pub extern "C" fn exported_function(x: i32) -> i32
```

### Trait Alias with Where Clause
```rust
pub trait SendSync<T> = Send + Sync + Clone
where
    T: std::fmt::Display;
```

### Union with Field Information
```rust
#[repr(C)]
pub union DataUnion {
    pub int_val: i32,
    pub float_val: f32,
}
```

### Re-exports and Use Declarations
```rust
pub use std::collections::HashMap;
pub use std::vec::Vec as SimpleVec;
```

## Technical Details

This tool uses:
- **syn**: For parsing Rust source code into Abstract Syntax Trees (AST)
- **Visitor Pattern**: For traversing and extracting information from ASTs
- **Advanced Pattern Matching**: For handling complex Rust language constructs
- **Type Resolution**: For displaying actual type names instead of placeholders

### Supported Rust Constructs

**All 15 types of Rust public items:**

1. **Functions** (`pub fn`) - Complex signatures with generics and where clauses
2. **Structs** (`pub struct`) - Named, unnamed, and unit fields with attributes
3. **Enums** (`pub enum`) - All variant types with detailed field information
4. **Traits** (`pub trait`) - With associated types and lifetime parameters
5. **Implementations** (`pub impl`) - Both inherent and trait implementations
6. **Constants** (`pub const`) - With type information and values
7. **Static Variables** (`pub static`) - Including mutable statics
8. **Type Aliases** (`pub type`) - With generic parameters and where clauses
9. **Modules** (`pub mod`) - With nested structure support
10. **Re-exports** (`pub use`) - Including path aliases and glob imports
11. **Macros** (`pub macro_rules!`) - Macro definitions with documentation
12. **External Crates** (`pub extern crate`) - Crate re-exports
13. **FFI Functions** (`pub extern "C" fn`) - With proper extern block formatting
14. **Unions** (`pub union`) - With field information and attributes
15. **Trait Aliases** (`pub trait Alias = ...`) - Trait alias definitions

**Additional features:**
- Generic parameters and where clauses
- CFG attributes and conditional compilation
- Documentation comments and examples
- Proper indentation and code formatting

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

- Built with [syn](https://github.com/dtolnay/syn) for Rust syntax parsing
- Inspired by the need for better LLM integration with Rust projects
- Part of the broader ecosystem for AI-assisted development tools