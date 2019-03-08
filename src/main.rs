#[macro_use]
extern crate clap;

use std::fmt::Debug;
use std::time::Duration;

use tokio::fs::File;
use tokio::prelude::*;

use args::Args;
use wfwalk::errors::*;
use wfwalk::ratelimiter::Limiter;
use wfwalk::stocks::Stocks;
use wfwalk::tokio_tools::erase_types;

mod args;

fn future_main() -> impl Future<Item = (), Error = Error> {
    setup().and_then(run).and_then(cleanup)
}

fn setup() -> impl Future<Item = Limiter, Error = Error> {
    start_rate_limiter()
}

fn start_rate_limiter() -> impl Future<Item = Limiter, Error = Error> {
    future::ok(Limiter::new())
}

fn run(mut limiter: Limiter) -> impl Future<Item = (), Error = Error> {
    future::result(limiter.add_task(future::ok(())))
}

fn cleanup(_: ()) -> impl Future<Item = (), Error = Error> {
    future::ok(())
}

fn real_main() -> Result<()> {
    let args = Args::parse()?;

    tokio::run(futures::lazy(|| erase_types(future_main())));

    //    let tree_future = wfwalk::tree::read_tree_async(args.file())
    //        .and_then(move |tree| {
    //            let stocks = Stocks::load_from_tree(&tree)?;
    //            if do_sanity_check {
    //                let insanities = stocks.sanity_check();
    //                for (symbol, vec) in insanities {
    //                    println!("{}", symbol);
    //                    for insanity in vec {
    //                        println!("  {}", insanity);
    //                    }
    //                }
    //            }
    //            Ok(stocks)
    //        })
    //        .and_then(|stocks| {
    //            let mut limiter = Limiter::new();
    //            limiter.add_task(File::open("/tmp/quux.tokio").map(|_| ()).map_err(|_| ()));
    //            Ok(())
    //        })
    //        .map_err(|e| eprintln!("{:?}", e));
    //
    //    tokio::run(tree_future);

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
    env_logger::init();

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
