#[macro_use]
extern crate clap;

use std::sync::Arc;
use std::time::Duration;

use futures::future::ok;
use tokio::prelude::*;

use wfwalk::errors::*;
use wfwalk::ratelimiter::Limiter;
use wfwalk::stocks::{sanity_check, Stocks};
use wfwalk::tokio_tools::erase_types;

mod args;

type Config = Arc<args::Config>;

fn future_main(args: Config) -> impl Future<Item = (), Error = Error> {
    setup(args.clone()).and_then(run).and_then(cleanup)
}

fn setup(config: Config) -> impl Future<Item = (Config, Limiter, Stocks), Error = Error> {
    let limiter = start_rate_limiter();
    let stocks = load_stock_info(config.clone());
    limiter
        .join(stocks)
        .map(|(limiter, stocks)| (config, limiter, stocks))
}

fn start_rate_limiter() -> impl Future<Item = Limiter, Error = Error> {
    future::ok(Limiter::new(3, Duration::from_secs(2)))
}

fn load_stock_info(config: Config) -> impl Future<Item = Stocks, Error = Error> {
    wfwalk::tree::read_tree_async(config.filepath.clone())
        .and_then(|tree| Stocks::load_from_tree(&tree))
}

//fn make_a_task(num: u8) -> impl Future<Item = (), Error = ()> {
//    ok(Instant::now()).map(move |i| println!("Task: {}/{}", num, i.elapsed().as_micros()))
//}

//fn maybe_sanity_check(config: &Config, stocks: &Stocks) -> Result<()> {
//    if config.do_sanity_check {
//        for (_, stock) in stocks.stocks.iter() {
//            let sanity = sanity_check(&stock);
//            if sanity.len() > 0 {
//                println!("{}", stock.symbol);
//                for warning in sanity {
//                    println!("  {}", warning);
//                }
//            }
//        }
//    }
//    Ok(())
//}

fn run_my_test_code<S>(config: &Config, stocks: S) -> impl Future<Item = (), Error = Error>
where
    S: AsRef<Stocks>,
{
    wfwalk::alphavantage::intraday("GOOG".to_string(), config.token.clone())
        .inspect(|v| println!("IN RUNNER: {:?}", v))
        .map(|_| ())
}

fn run(params: (Config, Limiter, Stocks)) -> impl Future<Item = (), Error = Error> {
    let (config, mut limiter, stocks) = params;

    let stocks_arc = Arc::new(stocks);

    //future::result(maybe_sanity_check(&config, stocks_arc))
    ok(()).and_then(move |_| run_my_test_code(&config, stocks_arc.clone()))

    //    let r = limiter
    //        .add_task(make_a_task(1))
    //        .and_then(|_| limiter.add_task(make_a_task(2)))
    //        .and_then(|_| limiter.add_task(make_a_task(3)))
    //        .and_then(|_| limiter.add_task(make_a_task(4)))
    //        .and_then(|_| limiter.add_task(make_a_task(5)))
    //        .and_then(|_| limiter.add_task(make_a_task(6)))
    //        .and_then(|_| limiter.add_task(make_a_task(7)))
    //        .and_then(|_| limiter.add_task(make_a_task(8)));
    //
    //    future::result(r)
}

fn cleanup(_: ()) -> impl Future<Item = (), Error = Error> {
    future::ok(())
}

fn real_main() -> Result<()> {
    let config = Arc::new(args::Config::new()?);

    tokio::run(futures::lazy(move || erase_types(future_main(config))));

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
