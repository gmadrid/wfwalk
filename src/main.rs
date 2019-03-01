use wfwalk::errors::*;
use wfwalk::stocks::sanity_check;
use wfwalk::stocks::Stocks;

fn main() -> Result<()> {
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
