#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate lazy_static;

//#[macro_use]
//extern crate futures;

pub mod errors;
pub mod stocks;
pub mod tree;
mod type_tools;
