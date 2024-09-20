
WIP toy interpreter for the Dart language.

The ambition is to create a simpler and more portable
implementation of Dart. It should run anywhere Rust can run.

## Top missing features:
* ~~Garbage collector~~.
* Type system.
* Standard library. (In progress)
* Inheritance.
* Private members.
* Getters/setters.
* Abstract classes.
* Isolates.
* Async/await.
* Static members.
* Exceptions.
* ~~Lists~~.
* Maps.
* Tuples.
* Static members.
* const.
* final.
* Factory constructors.
* Operator overloading.
* Mixins.
* Packages.

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

