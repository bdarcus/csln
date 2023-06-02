
This is an experiment in writing the [CSL-Next](https://github.com/bdarcus/csl-next) model and supporting libraries and tools in Rust, and converting to JSON Schema from there.

To understand the difference between this model and CSL 1.0, look at [style::options](https://github.com/bdarcus/csln/blob/main/style/src/options.rs). 
There, you will note configuration options for many details that in CSL 1.0 are configured within the template language:

- dates
- contributors
- substitution

Plus, I've added `localization` support there, with the idea it can be more easily-expanded there, than by burdening the template language with those details.

I've separated the code into discrete crates, with the intention to ultimately publish them.

I licensed the code here under the same terms as [citeproc-rs](https://github.com/zotero/citeproc-rs), in case code might be shared between them. 
Alas, I don't understand much of that codebase. 
But I understand the Mozilla 2.0 license is also compatible with Apache.

Between the two repos, I'm hoping to have demonstrated enough so far that this is a promising direction for the future of CSL, at least on the technical end, that folks might be willing to help build this out. 
Ideally, I want to develop one or both of these projects sufficiently to move them to the GitHub CSL org for further development and future maintenance. 
I don't think I have the time or skill to do that all myself, but am happy to help, provide direction, accept PRs, etc.
