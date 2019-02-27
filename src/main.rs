use wfwalk::errors::*;
use wfwalk::stocks::Stocks;

fn main() -> Result<()> {
    let stocks = dbg!(Stocks::load()?);


    Ok(())
}
