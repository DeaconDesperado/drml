# drml

A basic RML processor written in Rust.

```
Process RML mappings to generate RDF data

Usage: drml [OPTIONS] --mappingFile <MAPPING_FILE>

Options:
  -m, --mappingFile <MAPPING_FILE>
          The RML mapping file

  -o, --outputFile <OUTPUT_FILE>
          The output file

  -f, --format <FORMAT>
          The output file format

          Possible values:
          - turtle: Turtle format
          - nt:     N-Triples format
          - nq:     N-Quads format
          - jsonld: JSON-LD format
          - jelly:  Jelly format
          
          [default: nq]

  -b, --baseIRI <BASE_IRI>
          Used in resolving relative IRIs produced by the RML mapping

  -h, --help
          Print help (see a summary with '-h')
```
