## `globtest`

`globtest` is a command-line program to help rust developers interact with [the `glob` crate](https://doc.rust-lang.org/glob/glob/index.html), which is used to match patterns against paths across all platforms (like `fnmatch`).

### Usage

```
globtest 0.1
NeoSmart Technologies <https://neosmart.net/>
Print the results of rs-glob expressions

USAGE:
    globtest [FLAGS] <GLOB>...

FLAGS:
    -a, --all        Include hidden directories and files
    -d, --debug      Enable debug output
    -h, --help       Prints help information
    -i, --nocase     Disable case sensitivity
    -V, --version    Prints version information

ARGS:
    <GLOB>...    

```

`globtest` takes parameters that map directly to `MatchOptions` from the `glob` crate, and can be used to reproduce the results that the `glob_with` API would return. When called with the `--debug` option, `globtest` will output the details of the structure passed to the `glob_with` function.

### Example

When run against the source tree of the `globtest` repository, this is the output:

```
mqudsi@neosmart ~/globtest> globtest "*"
Cargo.lock
Cargo.toml
README.md
src
```

and here's an example that includes directory globbing:

```
mqudsi@neosmart ~/globtest> globtest "**"
Cargo.lock
Cargo.toml
README.md
src
```

**Important note: it is important to enclose filters/globs that include non-literal expressions such as `*` or `?` in double-quotes, so that they are not globbed by your shell!**

### Installation

`globtest` has been published as a crate for easy installation via cargo:

```
cargo install globtest
```

You can also clone/fork this git repository and build it by `cd`ing into the `globtest` path and executing `cargo install`.

### License & Authorship

`globtest` is written by Mahmoud Al-Qudsi \<mqudsi@neosmart.net\> of  NeoSmart Technologies. `globtest` is released under the terms of the MIT public license, and any rights not accordingly conferred are reserved and copyright NeoSmart Technologies 2017.