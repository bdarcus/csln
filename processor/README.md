This is a Rust processor library for the [csl-next](https://github.com/bdarcus/csl-next) model.

It is far from complete, but you can see its current state in the `csln` binary.

The basic processing design is as follows:

1. sort bibliography references (the HashMap values)
2. group the sorted bibliography to derive processing hints, and return a `HashMap` of them
3. the `render_references` method will then iterates through the `Style` templates, and above `Vector` and `HashMap`, and return an AST
4. methods will then render from AST to different output formats

A fragment of the current AST returned by `render_references()` is:

```js
  [
    {
      "templateComponent": {
        "contributor": "author",
        "form": "long",
        "rendering": null
      },
      "value": "Smith, John"
    },
    {
      "templateComponent": {
        "date": "issued",
        "form": "year",
        "rendering": null
      },
      "value": "2025"
    }
  ]
```
