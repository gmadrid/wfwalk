#[macro_use]
extern crate clap;

use args::Args;
use tokio::prelude::*;
use wfwalk::errors::*;
use wfwalk::stocks::Stocks;

mod args;

fn real_main() -> Result<()> {
    let args = Args::parse()?;

    let tree_future = wfwalk::tree::read_tree_async(args.file())
        .and_then(|tree| {
            let stocks = Stocks::load_from_tree(&tree)?;
            Ok(())
        })
        .map_err(|e| eprintln!("{:?}", e));

    tokio::run(tree_future);

    //    let stocks = Stocks::load()?;
    //    for stock in stocks.stocks.values() {
    //        let foo = sanity_check(&stock);
    //        if foo.len() > 0 {
    //            println!("\n{}", stock.symbol);
    //            println!("{:?}", foo)
    //        }
    //    }
    //
    //    let client = alphavantage::Client::new("OVI13JKC3O31YFSR");
    //    for stock in stocks.stocks.values() {
    //        print!("{}: \n", stock.symbol);
    //        let time_series = client.get_time_series_daily(&stock.symbol).unwrap();
    //        let entry = time_series.entries.last().unwrap();
    //        println!("{:?}", entry);
    //        thread::sleep(time::Duration::from_millis(5000));
    //    }

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
