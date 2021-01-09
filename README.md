bib
===
BibTex bibliography tools, written in Rust using `biblatex` library.

The following utilities are included:

- `bibfmt`: format BibTex
- `bib2ref`: convert BibTex to GNU Refer format
- `bib2tsv`: convert BibTex to TSV (only author, title columns for now)


Usage
-----

	BIN < FILE.bib

Every tool reads from a filename or from a standard input and writes to standard output.
