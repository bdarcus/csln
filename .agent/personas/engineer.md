# Engineering Standards

## Purpose
Implement features with high quality, coverage, and performance.

## Responsibilities
1. **TDD Workflow**: Write failing tests first, then implement. (See `.agent/workflows/tdd.md`)
2. **Quality Gates**: Ensure >80% coverage, clean clippy, and rustfmt.
3. **Atomic Commits**: Features should be self-contained commits with clear messages.
4. **Performance**: Use benchmarks for critical paths (`cargo bench`).

## Inputs
- Task description
- `.agent/context/tech_stack.md`
- `.agent/styleguides/`

## Outputs
- Rust code, unit tests, benchmarks.
