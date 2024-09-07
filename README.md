
WIP toy interpreter for the Dart language.

## Top missing features:
* ~~Garbage collector~~.
* Standard library.
* Inheritance.
* Isolates.
* Async/await.
* Getters/setters.

## Installation
* Install rust/cargo.
* `git clone https://github.com/ktullavik/redart.git`
* `cd redart`
* `cargo build`

You will find the redart executable in the target/debug dir.

## Examples
`redart <filename.dart>` will interpret the file.  
`redart test` will run all bundled tests.  
`redart test n` will run bundled test nr *n*.   
`redart test lex n` will run the lex stage on test nr *n*.  
`redart test parse n` will run the lex and parse stage on test nr *n*.

