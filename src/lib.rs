#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

mod arena;
mod ntree;
mod treereader;

mod errors {
    error_chain! {}
}
