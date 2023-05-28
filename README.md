
This is an experiment in writing the CSL-Next model in Rust, and converting to JSON Schema from there.

It will build a `csln-schemas` binary to create the schemas.

Currently, the model doesn't yet match the [typescript model](https://github.com/bdarcus/csl-next) on the details. 
This exercise may prompt me to make a few changes over there.

I did experiment with turning this code into discrete crates for `Style` and `Bibliography`, etc., but got stuck on import errors. 
Feel free to submit a PR if you like for that, though this may remain more a demo than anything.

I have started to port the processor logic from the typescript code as well.
