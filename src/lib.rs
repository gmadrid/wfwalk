#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

mod tree;

mod errors {
    error_chain! {}
}
