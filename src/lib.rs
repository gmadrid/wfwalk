#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

//#[macro_use]
//extern crate log_derive;

#[macro_use]
extern crate futures;

#[allow(deprecated)]
pub mod errors {
    // TODO: figure out a non-deprecated way forward for error handling.
    error_chain! {
        errors {
            BadParse(nonterminal: &'static str, desc: &'static str, _text: String) {
                description("a parse error"),
                display("Parse error: {:1} {:0}", desc, nonterminal),
            }

            WeirdError(msg: &'static str) {
                description("a weird error"),
                display("A weird error: {}", msg),
            }
        }

        foreign_links{
            Clap(clap::Error);
            Hyper(hyper::error::Error);
            Io(::std::io::Error);
            ParseFloat(::std::num::ParseFloatError);
            RecvError(tokio::sync::mpsc::error::UnboundedRecvError);
            SerdeJson(serde_json::error::Error);
            TokioTimer(tokio::timer::Error);
        }
    }
}

pub mod alphavantage;
pub mod display;
pub mod ratelimiter;
pub mod stocks;
pub mod tokio_tools;
pub mod tree;
mod type_tools;
