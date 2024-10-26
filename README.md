
WIP toy interpreter for the Dart language.

The ambition is to create a simple and portable implementation of Dart. It should run anywhere Rust can run.

|       Feature          |    Status                                                                                          |
| ---------------------- | -------------------------------------------------------------------------------------------------- |
| Garbage collector      | Simple mark-sweep collector implemented                                                            |
| Type system            | Not much                                                                                           |
| Standard library       | In progress                                                                                        |
| Inheritance            | ✓                                                                                                  |
| Initializer lists      | ✓                                                                                                  |
| Private members        | TODO                                                                                               |
| Static members         | TODO                                                                                               |
| Getters                | TODO                                                                                               |
| Setters                | TODO                                                                                               |
| Optional arguments     | TODO                                                                                               |
| Arrow functions        | TODO                                                                                               |
| Abstract classes       | TODO                                                                                               |
| Generics               | TODO                                                                                               |
| Isolates               | TODO                                                                                               |
| Async/await            | TODO                                                                                               |
| Exceptions             | TODO                                                                                               |
| Lists                  | Limited support                                                                                    |
| Maps                   | TODO                                                                                               |
| Sets                   | TODO                                                                                               |
| Tuples                 | TODO                                                                                               |
| const                  | Supported for top-level variables only                                                             |
| final                  | TODO                                                                                               |
| Factory constructors   | TODO                                                                                               |
| Operator overloading   | TODO                                                                                               |
| Mixins                 | TODO                                                                                               |
| Packages               | TODO                                                                                               |


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

