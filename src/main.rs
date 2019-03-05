#[macro_use]
extern crate clap;

use args::Args;
use tokio::prelude::*;
use wfwalk::errors::*;
use wfwalk::stocks::Stocks;
use wfwalk::ratelimiter::Limiter;
use tokio::fs::File;
use std::time::Duration;

mod args;

fn real_main() -> Result<()> {
    env_logger::init();

    let args = Args::parse()?;

    let do_sanity_check = args.do_sanity_check();

    let tree_future = wfwalk::tree::read_tree_async(args.file())
        .and_then(move |tree| {
            let stocks = Stocks::load_from_tree(&tree)?;
            if do_sanity_check {
                let insanities = stocks.sanity_check();
                for (symbol, vec) in insanities {
                    println!("{}", symbol);
                    for insanity in vec {
                        println!("  {}", insanity);
                    }
                }
            }
            Ok(stocks)
        })
        .and_then(|stocks| {
            let mut limiter = Limiter::new();
            limiter.add_task(File::open("/tmp/quux.tokio").map(|_| ()).map_err(|_| ()));
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
    //    let client = alphavantage::Client::new("CLIENT TOKEN");
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
