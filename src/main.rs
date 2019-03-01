#[macro_use]
extern crate clap;

use wfwalk::errors::*;
use wfwalk::stocks::sanity_check;
use wfwalk::stocks::Stocks;

mod args;

use args::Args;

fn real_main() -> Result<()> {
        let args = Args::parse()?;

    let stocks = Stocks::load()?;
    for stock in stocks.stocks.values() {
        let foo = sanity_check(&stock);
        if foo.len() > 0 {
            println!("\n{}", stock.symbol);
            println!("{:?}", foo)
        }
    }
    Ok(())
}

fn main() {
    match real_main() {
        Ok(_) => (),
        Err(err) => {
            match err {
                // Clap gets special attention. ('-h' for example is better handled by clap.)
                Error(ErrorKind::Clap(ce), _) => ce.exit(),
                _ => println!("Error: {}", err),
            }
        }
    }
}
