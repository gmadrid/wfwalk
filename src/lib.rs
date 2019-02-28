#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate lazy_static;

pub mod stocks;
pub mod tree;
mod type_tools;

pub mod errors {
    error_chain! {
        errors {
            BadParse(nonterminal: String, desc: String, _text: String) {
                description("a parse error"),
                display("Parse error: {:1} {:0}", desc, nonterminal),
            }
        }

        foreign_links{
            Io(::std::io::Error);
            ParseFloat(::std::num::ParseFloatError);
        }
    }
}

use type_tools::{BoolTools, OptionTools, VecTools};