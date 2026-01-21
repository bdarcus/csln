## Vision

At a high-level, the vision of the project is to provide a simpler, easier-to-extend, and more featureful successor to CSL, with a model defined in Rust code, and JSON schemas generated from it.

More specifically, the idea is to:

1. Adapt what we've learned in almost 20 years of experience with [CSL 1.0][CSL] to modern programming idioms and formats.
2. Simplify the template part of the language, and put more, and extensible, logic in option groups, so it's easier to work with for users, style editors, and developers alike.
3. Add new features while we're at it, like multi-lingual support, advanced dates and times, narrative citations, and so forth.
4. Align code and schemas by generating the latter from the former, and so also provide a common meeting point for developers and domain experts.

More concretely, the goal is a suite of models, libraries and tools that make extremely performant advanced citation and bibliography processing available everywhere: 

- desktop and web
- batch-processing for formats like pandoc markdown, djot, LaTeX, and org-mode
- interactive real-time processing for GUI contexts like Zotero
- easy-to-use style creation wizards, both command-line and web

## Principles

For the `Style` model:

1. As with [CSL 1.0][CSL], styling is agnostic of input and output formats, including whether one is using an author-date citation style, numeric, or note-based. 
2. Keep the template language as simple as possible, in the hopes we can keep it stable going forward, while still enabling innnovation. In a GUI, behavior (sorting, substitution, etc) would be configured in those options, and not in the templates.
3. Add new functionality primarily via option groups.

For the `InputReference` and `Citation` models:

3. No string-parsing, with the sole exception of the [EDTF date format][EDTF], which is now ISO-standardized as an extension profile of ISO 8601, with well-defined parsing rules, and parsing libraries available in multiple languages.
4. Provide structure where needed, but offer alternatives where not. EDTF is available for diverse date-time encoding, but dates fields will fallback to a plain string. Likewise, the `Contributor` model offers similar flexibility, and power where needed.

## Caveats and Status

This is not particularly close to ready for actual use, and needs more development, testing, and input.

A very high-level summary of where this at ATM:

- complete-ish draft models for bibliography, citations, styles, locales
- YAML and JSON serialization and deserialization of these models, and a `csln-schemas` binary that will create JSON schemas to validate them
- a processor which can create formatted string output using the above inputs, but which is designed for pluggable renderers (see [#105](https://github.com/bdarcus/csln/issues/105)); includes basic author substitution, basic EDTF date parsing and formatting, and a few other things I'm likely forgetting
- a `csln` CLI that uses the above; it's Rust, so a single binary, and very fast.

## The model

### Influences

1. The [CSL 1.0 specification][CSL-spec] [options][CSL-options], and its template language (aka [layout][CSL-templates] and [rendering elements][CSL-render]), most notably from names, dates, and other formatting.
2. Patterns observed in the [CSL 1.0 styles repository][CSL-styles].
3. The [BibLaTeX preamble][BLTX] options.
4. The [Typst Hayagriva][haya] project has some interesting details; particularly its input data model, and its [selector macro][sel].

### Comparison to CSL 1.0 and BibLaTeX

To understand the difference between this model and [CSL 1.0][CSL], look at [style::options][CSLNO]. 
There, you will note configuration options for many details that in CSL 1.0 are configured within the template language:

- dates
- contributors
- substitution

Plus, I've added `localization` support as such a configuration option group, with the idea it can be more easily-expanded there, than by burdening the template language with those details.

In that sense, this design is closer to [BibLaTeX][BLTX], which has a very long list of flat options that handle much of the configuration. 
Like that project, here we standardize on [EDTF dates][EDTF]. 

On the citation end, CSL in general has been most akin to the BibLaTeX `autocite` commands rather than the lower-level ones. This is to ensure documents are portable across radically-different output styles. But this model adds a basic distinction between "integral" (aka narrative or text) citations, and "non-integral."

## Project Organization

I've separated the code into discrete crates, with the intention to ultimately publish them.

I'm hoping to have demonstrated enough so far that this is a promising direction for the future of CSL, at least on the technical end, that folks might be willing to help build this out. 
Ideally, I want to develop this project sufficiently to move it to the [GitHub CSL org][CSLO] for further development and future maintenance. 
Doing so, however, will require sorting out details of how that process is managed and funded going forward.

## Contributing

I would _love_ to have help on this, both because I'm an amateur programmer and a Rust newbie, and because the vision I am sketching out here will take a lot of work to realize.

### Getting Started

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

### Project Structure

- `csln/` - Core library with data models for styles, bibliography, and citations
- `cli/` - Command-line interface for processing citations
- `processor/` - Citation and bibliography processing engine
  - `src/lib.rs` - Library entry point
  - `src/processor.rs` - Main processor logic
  - `src/types.rs` - Core data types
  - `src/values.rs` - Value extraction logic
  - `src/render.rs` - Rendering logic

### How to Help

Please contact me via discussions or the issue tracker, or by email, if you'd like to contribute.

I licensed the code here under the same terms as [citeproc-rs][CSLRS], in case code might be shared between them. 
I also understand the Mozilla 2.0 license is compatible with Apache.

A note on citeproc-rs:

In reviewing the code, it strikes me pieces of it obviously complement this code base. 
In particular, it has been optimized for the Zotero use-case, where it provides real-time formatting, while I have focused of the batch-processing case.

[CSL]: https://citationstyles.org/
[CSLNJS]: https://github.com/bdarcus/csl-next
[CSLNO]: https://github.com/bdarcus/csln/blob/main/csln/src/style/options.rs
[CSLRS]: https://github.com/zotero/citeproc-rs
[CSLO]: https://github.com/citation-style-language
[CSL-spec]: https://docs.citationstyles.org/en/stable/specification.html
[CSL-styles]: https://github.com/citation-style-language/styles
[CSL-macros]: https://docs.citationstyles.org/en/stable/specification.html#macros
[CSL-templates]: https://docs.citationstyles.org/en/stable/specification.html#layout-1
[CSL-render]: https://docs.citationstyles.org/en/stable/specification.html#rendering-elements
[CSL-options]: https://docs.citationstyles.org/en/stable/specification.html#options
[BLTX]: https://github.com/plk/biblatex
[EDTF]: https://www.loc.gov/standards/datetime/
[haya]: https://github.com/typst/hayagriva
[sel]: https://github.com/typst/hayagriva/blob/main/docs/selectors.md
