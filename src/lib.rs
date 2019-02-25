#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

pub mod tree;

mod errors {
    error_chain! {}
}
