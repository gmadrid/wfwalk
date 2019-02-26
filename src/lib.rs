#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

pub mod tree;

pub mod errors {
    error_chain! {
        foreign_links{
            Io(::std::io::Error);
        }
    }
}
