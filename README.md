# gml_fmt: GML Code Formatter

[![Build Status](https://travis-ci.org/sanboxrunner/gml_fmt.svg?branch=master)](https://travis-ci.org/sanboxrunner/gml_fmt)

`gml_fmt` is an autoformatter written in Rust for GML. It is fast and works on code which will not compile.

# How To Install and Run

Simply go to [releases](http://link.com) and download the most recent binary in whatever OS you're on. Or, click here:

- [Windows 10](http:://link.com)
- [macOS](http:://link.com)
- [Linux](http:://link.com)

The Windows builds may be delayed from other platforms, as I have not found a suitable deployment platform for Rust. Instead, I compile it myself on each release.

Once you've downloaded the program, simply place it next to your `.yyp` file in your projects root directory. Open your native terminal (Command Prompt on Windows, Bash on other platforms) and run:

```bash
gml_fmt --version
```

If you get back a version number, the tool is working.

To format your code, first **make sure that you are using source control or have another backup. This is an autoformatter, and though it is battle tested, it could be your project that shows the bug. Have a backup ready for that case.**

Run:

```
gml_fmt
```

Run `gml_fmt -f path/to/file` to format only a single file. Otherwise, gml_fmt will format everything in the directory its in that is a `.gml` file.

Run `gml_fmt --help` to get a full listing of commands available.

Currently, watch mode is not enabled, but future updates will bring it, if the tool sees adoption.

If you would like to use the tool without moving it between projects, add it to your PATH and then invoke like so:
```
gml_fmt path/to/directory/of/project
```
Created a bash script to automate this task might be a good idea.

# Wait, wait..what are you doing to my code?
`gml_fmt` is opinionated, primarily because making configuration options makes it slower to make and slower to run. 

`gml_fmt` outputs K&R Java compliant code. In normal human terms, that means:
```js
if (formatted) {
    show_debug_message("We're K&R all day.");
}
```
Additionally, `gml_fmt` removes excess newlines, adds spacing where appropriate, does some indentation where appropriate, and always leaves an extra blank line at the end of a file.

For example:
```js
// =========INPUT=========
if (a(b[i])&& b[i] .c < 0)
    {

    var c = b[i].q
    var l = b[i].q



    b[i].q = c;
    b[i].q[0] = 30;
    
    }
//=========OUTPUT=========
if (a(b[i]) && b[i].c < 0) {
    var c = b[i].q;
    var l = b[i].q;
    
    b[i].q = c;
    b[i].q[0] = 30;
}

```
Since the amount of formatting that `gml_fmt` does is fairly large, it is recommended to download it and try to format some code yourself.

# What do I do if the formatter breaks my code?

Log an issue! To correctly fix any problems, all that is needed is the input code. Output code is appreciated, but can be remade based on the input code. 

To keep using the tool before a fix is made, appending this comment anywhere in a file:
```
// @gml_fmt ignore
```
will ask gml_fmt to ignore that file completely. In the future, line based ignores will be created.

### Platforms

It is currently only a CLI, though the following platforms will be supported:

- [x] A simple CLI to autoformat on request.
- [ ] A watcher, spawned by the CLI, to format all .gml files in a project on save.
- [ ] A GMEdit plugin to support formatting without saving.

### Features

- [x] Can handle code which will not compile.
- [x] Extremely fast with few allocations.
- [x] Opinionated. It will have only a few configuration options.

## Contributing

Please fork, contact me here, submit an issue or a PR, or contact me via Discord at `jack (sanbox)#0001` if you would like to contribute. All contributions are welcome!

The project is simple to build -- `cargo build` will build it and install the necessary dependencies.

To test, see `./benches/samples/small_test.gml` and the built in bash script `run_sample.sh`. Alternatively, run the test suit with `test.sh`. Both require you to have a file `./ignored/output.yaml` (you will need to create the "ignored" directory).

Before submitting a PR, you should run the test suit. Anything that fails the test suit will not be accepted. 
