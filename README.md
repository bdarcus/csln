
This is an experiment in writing the [CSL-Next](https://github.com/bdarcus/csl-next) model in Rust, and converting to JSON Schema from there.

In experimenting more with code generation, I ended up diving in the deep end, and implementing the current state of the typescript model in Rust.

It produces two binaries:

- `csln` when done (hopefully soon) take bib and style files as input and spits out pre-rendered JSON
- `csln-schemas` generates JSON schemas for the two models

I am comfortable in Rust even less than typescript, but it is close to working; my goal is the first deserializes the same style and bibliography files as the typescript code, and produces the same JSON.

I licensed both under the same terms as [citeproc-rs](https://github.com/zotero/citeproc-rs), in case code might be shared between them. 
Alas, I don't understand much of that codebase.

Between the two repos, I'm hoping to have demonstrated enough so far that this is a promising direction for the future of CSL, at least on the technical end, that folks might be willing to help build this out. 
Ideally, I want to build one or both of these projects out sufficiently to move them to the GitHub CSL org for further development and future maintenance. 
I don't think I have the time or skill to do that all myself, but am happy to help, provide direction, accept PRs, etc.
