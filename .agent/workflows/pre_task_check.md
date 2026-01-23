---
description: Standard pre-task and pre-commit checks for csln
---

# csln Workflow Checklist

## Pre-Task Initialization
1.  **Context Check**: Verify the current branch (`git branch --show-current`).
2.  **Clean Slate**: Ensure no uncommitted changes (`git status`) or stash relevant work.
3.  **Baseline Benchmarking**: If the task involves performance-sensitive refactoring:
    // turbo
    - Run benchmarks: `cargo bench -p csln-processor`
    - Record the baseline results in the task notes.

## Pre-Commit Checklist
// turbo-all
1.  **Format**: `cargo fmt --all`
2.  **Lint**: `cargo clippy --workspace --all-targets --all-features -- -D warnings`
3.  **Test**: `cargo test --workspace`
4.  **Verification Benchmarking**: If performance was a goal:
    - Run benchmarks again: `cargo bench -p csln-processor`
    - Compare with baseline and include a summary in the commit body.
5.  **License**: Ensure new files have the SPDX header (MPL-2.0) and correct year (2023-2026).
6.  **Changelog**: If this is a significant feature, update `CHANGELOG.md` using `git-chglog`.

## Commit Quality
- **Subject**: Max 50 characters, conventional commit format (e.g., `feat:`, `perf:`).
- **Body**: Wrap at 72 characters.
- **DCO**: Ensure `Signed-off-by` is present (`git commit -s`).
- **Content**: Explain the *why*, not just the *what*.
- **Restriction**: NO backticks, backslashes, or Markdown formatting in the commit body.
