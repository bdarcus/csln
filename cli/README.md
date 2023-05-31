Right now, this has two simple binaries:

1. `csln` runs the processor
2. `csln-schemas` creates the schemas

I'm thinking to merge them in a single, richer, cli; something like:

```console
csln make schemas -d /tno
csln process bibliography -t latex -b bib.yanl -s style.json
csln find style
csln make style
```

