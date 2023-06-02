Right now, this has two simple binaries:

1. `csln` runs the processor
2. `csln-schemas` creates the schemas

I'm thinking to merge them in a single, richer, cli; something like:

```console
csln make schemas -d /tmp/schemas
csln process bibliography -t latex -b bib.yaml -s style.json
csln process document -t djot -b bib.yaml -s style.json mymanuscript.dj
csln find style abc
csln make style xyz
```

