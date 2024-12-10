# Strip Comments

A simple dependency-less command line tool that will strip comments from source code and otherwise leave it intact.

Line comments start with `//` and run until but not including the next newline or end of file.
Multiline comments start with `/*` and run until `*/` or end of file.
Lines containing only whitespace and comments will be removed entirely. 
However, empty lines and lines containing only whitespace but no comments will be left untouched.

If comments occur inside string literals they are ignored (i.e. not stripped).
String literals are either delimited by single quote (`'`) or double quote (`"`) on each side, or as Rust raw strings starting with `r#"` and ending with `"#`. 

## Usage

Make sure you have a [Rust toolchain](https://rustup.rs/) installed on your system to build and run.

To quickly try it out, you can use cargo run.
```sh
# Inside the repository folder
cargo run -- /path/to/source/file
``` 
The program will write the result to stdout.

To properly build the project:
```sh
cargo build --release
# The executable can be found at target/release/strip-comments 
cd target/release
strip-comments path/to/source/file >> path/to/destination
```
Running the tests is as easy as running
```
cargo test
```

## Caveats
- It only supports UTF-8 as source and output. 
- The given source-path must be valid Unicode
- Some languages may have special delimiters for string literals and especially multi-line strings. `"""`, `'''`, backticks or any combination thereof are currently not supported (adding support for them or any other delimiter-pair is trivial however).
- It assumes single backslash (`\`) to be a universal escape-character inside string-literals. Languages may have different or additional escaping-mechanisms that are not currently supported.
- Unclosed string literals or multi-line-comments will silently run until end of file. Some languages may have different semantics around this.




