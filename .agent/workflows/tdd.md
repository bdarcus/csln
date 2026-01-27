---
description: Test-Driven Development workflow with high safety and auditable results.
---

# TDD Workflow (Red-Green-Refactor)

## 1. Red Phase: Failing Tests
- Create or update a test file (e.g., `tests/*.rs`).
- Write unit tests for the specific task requirements.
- **Run tests**: `cargo test --test <name>` and confirm failure.

## 2. Green Phase: Implementation
- Write the minimum code required to pass the failing tests.
- **Verify**: `cargo test --workspace` until all tests pass.

## 3. Refactor Phase
- Clean up the code and tests while keeping them green.
- Check performance: `cargo bench` if applicable.

## 4. Verification & Coverage
- Ensure code coverage is >80% for new logic.
- Run linters: `cargo clippy`.
- Run formatters: `cargo fmt`.

## 5. Commit & Audit
- **Commit Message**: Use conventional commits (e.g., `feat:`, `fix:`).
  - **Subject**: Max 50 chars, lowercase, no period.
  - **Body**: Wrap at 72 chars. Why, not just what.
- **Git Note**: Attach a summary of the task using `git notes add -m "<summary>" <hash>`.
- **Note Content**: Task name, changes, files modified, and "why".

