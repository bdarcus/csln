# Specification: YAML-based Integration Test Suite

## Objective
Create a data-driven integration test suite using YAML files to validate the `csln-processor`'s rendering accuracy across various styles and reference types.

## Requirements
- **YAML Test Format:** Define a schema that includes:
    - `name`: Description of the test case.
    - `style`: The CSL style configuration (YAML/JSON).
    - `bibliography`: The input reference data.
    - `citation`: The citation to be rendered.
    - `expected`: The expected string output.
- **Test Runner:** A Rust test in the `processor` crate that iterates over all `.yaml` files in a dedicated test data directory.
- **Dynamic Execution:** The runner should dynamically load the style and data, execute the processor, and assert equality with the expected output.

## Architecture
- **Crate:** `processor`
- **Test File:** `processor/tests/integration.rs`
- **Data Directory:** `processor/tests/data/*.yaml`
