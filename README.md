
This is an experiment in writing the CSL-Next model in Rust, and converting to JSON Schema from there.

It will build a `csln-schemas` binary to create the schemas.

Currently, much of the model is effectively place-holder; it doesn't match the [typescript model](https://github.com/bdarcus/csl-next) on the details (yet).

I did experiment with turning this code into discrete crates for `Style` and `Bibliography`, etc., but got stuck on import errors. 
Feel free to submit a PR if you like for that, though this may remain more a demo than anything.
