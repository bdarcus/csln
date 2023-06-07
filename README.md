## Vision

This is a project to write the [CSL-Next][CSLNJS] typescript model and supporting libraries and tools in Rust, and convert to JSON Schema from there.

At a high-level, the vision of the project is to:

1. Adapt what we've learned in almost 20 years of experience with [CSL 1.0][CSL] to modern programming idioms and formats.
2. Simplify the template part of the language, and put more, and extensible, logic in option groups, so it's easier to work with for users, style editors, and developers alike.
3. Add new features while we're at it, like multi-lingual support, advanced dates and times, narrative citations, and so forth.

More concretely, the goal is a suite of models, libraries and tools that make extremely performant advanced citation and bibliography processing available everywhere: 

- desktop and web
- batch-processing for formats like pandoc markdown, djot, LaTeX, and org-mode
- interactive real-time processing for GUI contexts like Zotero
- easy-to-use style creation wizards, both command-line and web

## The model and CSL 1.0

To understand the difference between this model and [CSL 1.0][CSL], look at [style::options][CSLNO]. 
There, you will note configuration options for many details that in CSL 1.0 are configured within the template language:

- dates
- contributors
- substitution

Plus, I've added `localization` support as such a configuration option group, with the idea it can be more easily-expanded there, than by burdening the template language with those details.

## Project Organization

I've separated the code into discrete crates, with the intention to ultimately publish them.

I'm hoping to have demonstrated enough so far that this is a promising direction for the future of CSL, at least on the technical end, that folks might be willing to help build this out. 
Ideally, I want to develop one or both of these projects sufficiently to move them to the [GitHub CSL org][CSLO] for further development and future maintenance. 
Doing so, however, will require sorting out details of how that process is managed and funded.

## Contribution

I would _love_ to have help on this, both because I'm an amateur programmer and a Rust newbie, and because the vision I am sketching out here will take a lot of work to realize.

Please contact me via discussions or the issue tracker, or by email, if you'd like to contribute.

I licensed the code here under the same terms as [citeproc-rs][CSLRS], in case code might be shared between them. 
I also understand the Mozilla 2.0 license is compatible with Apache.

A note on citeproc-rs:

In reviewing the code, it strikes me pieces of it obviously complement this code base. 
In particular, it has been optimized for the Zotero use-case, where it provides real-time formatting, while I have focused of the patch-processing case.

[CSL]: https://citationstyles.org/
[CSLNJS]: https://github.com/bdarcus/csl-next
[CSLNO]: https://github.com/bdarcus/csln/blob/main/style/src/options.rs
[CSLRS]: https://github.com/zotero/citeproc-rs)
[CSLO]: https://github.com/citation-style-language