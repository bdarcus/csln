# Contributing

I would _love_ to have help on this, both because I'm an amateur programmer and a Rust newbie, and because the vision I am sketching out here will take a lot of work to realize.

## Getting Started

To build and test the project:

```bash
# Clone the repository
git clone https://github.com/bdarcus/csln.git
cd csln

# Build the project
cargo build

# Run tests
cargo test

# Run clippy for code quality checks
cargo clippy --all-targets --all-features

# Format code
cargo fmt

# Generate JSON schemas
cargo run --bin csln-schemas
```

## Development Standards

To maintain code quality and a clean history, please follow these guidelines:

1.  **Commit Messages**: Use [Conventional Commits](https://www.conventionalcommits.org/).
    - Subject: Max 50 characters.
    - Body: Wrapped at 72 characters.
    - DCO: All commits must be signed-off (`git commit -s`).
    - Format: Use plain text only (no backsticks or Markdown) in commit bodies.
2.  **Code Quality**: Ensure `cargo fmt` and `cargo clippy --workspace` pass before submitting.
3.  **Performance**: For performance-related changes, provide before/after benchmark results from `cargo bench -p csln-processor`.
4.  **Licensing**: New files must include the SPDX license header (MPL-2.0).

## Project Structure

- `csln/` - Core library with data models for styles, bibliography, and citations
- `cli/` - Command-line interface for processing citations
- `processor/` - Citation and bibliography processing engine
  - `src/lib.rs` - Library entry point
  - `src/processor.rs` - Main processor logic
  - `src/types.rs` - Core data types
  - `src/values.rs` - Value extraction logic
  - `src/render.rs` - Rendering logic

## How to Help

Please contact me via discussions or the issue tracker, or by email, if you'd like to contribute.

I licensed the code here under the same terms as [citeproc-rs][CSLRS], in case code might be shared between them. 
I also understand the Mozilla 2.0 license is compatible with Apache.

A note on citeproc-rs:

In reviewing the code, it strikes me pieces of it obviously complement this code base. 
In particular, it has been optimized for the Zotero use-case, where it provides real-time formatting, while I have focused of the batch-processing case.

[CSLRS]: https://github.com/zotero/citeproc-rs
