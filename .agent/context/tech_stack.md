# Technology Stack

## Core Language & Runtime
- **Rust:** The primary programming language, chosen for its memory safety, performance, and modern tooling. The project uses a Cargo workspace (resolver 2) to manage its components.

## Data Serialization & Standards
- **Serialization:** Native support for **JSON** and **YAML** using `serde` and `serde_json`.
- **Date/Time Standards:** Adherence to **EDTF** (Extended Date/Time Format) for robust and standardized date handling.
- **Schema Generation:** Automated generation of JSON schemas from Rust models to ensure cross-language compatibility.

## Project Architecture
- **Monorepo (Workspace):**
  - `csln`: Core library defining the data models for bibliography, citations, and styles.
  - `processor`: The citation processing engine and rendering logic.
  - `cli`: A command-line interface for interacting with the processor.

## Development & Quality Tools
- **Build System:** Cargo
- **Linting:** `cargo clippy` (with workspace-level lint configurations)
- **Formatting:** `cargo fmt`
- **Testing:** `cargo test` for unit and integration tests.
- **Benchmarking:** `cargo bench` (using `criterion` or similar) for performance tracking in `csln-processor`.

## Deployment & Distribution
- **Binary:** Single, statically-linked binaries for the CLI and schema generation tools.
