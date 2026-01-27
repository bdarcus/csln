# Test Coverage Extension Plan

This document outlines a strategy to significantly increase test coverage across the `csln` workspace, focusing on the core processing logic and data extraction.

## 1. Processor Component Logic (`processor/src/values.rs`)

The extraction of values from bibliographic references into template components is currently the most complex and least tested area.

### Priorities:
- [ ] **Field Extraction**: Implement unit tests for all `Variables` and `Numbers` variants across all `InputReference` types (Monograph, SerialComponent, etc.).
- [ ] **Contributor Formatting**: Expand tests for `role_to_string` and `TemplateContributor::values` to cover all forms (Long, Short, Verb) and role substitution logic.
- [ ] **Date Processing**: Add comprehensive tests for `TemplateDate` including EDTF parsing edge cases and disambiguation suffixes (`int_to_letter`).

## 2. Rendering & Formatting (`processor/src/render.rs`)

Ensure the final string output matches expected citation standards.

### Priorities:
- [ ] **Display Trait**: Add tests for `ProcTemplateComponent`'s `Display` implementation, specifically verifying prefix/suffix handling and `WrapPunctuation` (Parentheses, Brackets).
- [ ] **Collection Rendering**: Test `refs_to_string` with multiple references to ensure proper separators and final punctuation.

## 3. Top-Level Processor Features (`processor/src/processor.rs`)

Verify the orchestration of styles, bibliographies, and citations.

### Priorities:
- [ ] **Sorting**: Test `sort_references` with complex `SortKey` arrays (e.g., Author -> Year -> Title).
- [ ] **Disambiguation**: Verify hint calculation and how it affects output (e.g., adding 'a', 'b' to years).
- [ ] **Error Handling**: Test `process_citations` with missing references or invalid style configurations.

## 4. Model Integrity (`csln/src/`)

### Priorities:
- [ ] **Style Validation**: Add tests for `Style` deserialization from YAML/JSON to ensure complex templates are parsed correctly.
- [ ] **Locale Handling**: Verify locale merging and term lookup safety.

## Execution Strategy

1.  **Phase 1**: Add unit tests in the same files as the logic (using `#[cfg(test)]`) for fast feedback.
2.  **Phase 2**: Create a `tests/` directory in the `processor` crate for high-level integration tests using real styles and bibliographies.
3.  **Phase 3**: Integrate coverage reporting into CI to maintain a >80% threshold for new PRs.
