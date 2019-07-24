# gml_fmt

[![Build Status](https://travis-ci.org/sanboxrunner/gml_fmt.svg?branch=master)](https://travis-ci.org/sanboxrunner/gml_fmt)

gml_fmt is an autoformatter written in Rust for GML. It is fast, test-based, and works on code which will not compile, and always produces an identical output regardless of the original whitespace. Additionally, it will form the basis of a future Rust based linter for GMS2.

## In development work

gml_fmt is still is still in active development, having only began in late July 2019. Most of the features listed below have not been developed yet. This page will accurately reflect them as they are further developed.

### Platforms

It is currently only a CLI, though the following platforms will be supported:

- [x] A simple CLI to autoformat on request.
- [ ] A watcher, spawned by the CLI, to format all .gml files in a project on save.
- [ ] A GMEdit plugin to support formatting without saving.
- [ ] A CLI to allow other programs to seamlessly tap into gml_fmt.

### Features

- [x] Can handle code which will not compile. 
- [ ] Always produces the same output given the same series of tokens, regardless of whitespace. 
- [ ] Extremely fast with few allocations.
- [ ] Opinionated. It will have only a few configuration options.

## Contributing

Feel free to contact me here, submit an issue or a PR, or contact me via Discord at `jack (sanbox)#0001` if you would like to contribute. All contributions are welcome!