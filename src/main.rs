#[macro_use]
extern crate clap;

use std::sync::{Arc, RwLock};
use std::time::Duration;

use futures::future::{err, ok};
use tokio::prelude::*;

use args::Config;
use std::path::PathBuf;
use wfwalk::display::Displayer;
use wfwalk::errors::*;
use wfwalk::ratelimiter::Limiter;
use wfwalk::stocks::{sanity_check, Stocks};
use wfwalk::tokio_tools::erase_types;

mod args;

fn future_main(args: Config) -> impl Future<Item = (), Error = Error> {
    setup(args).and_then(run).and_then(cleanup)
}

fn setup(
    config: Config,
) -> impl Future<Item = (Config, Displayer, Limiter, Arc<RwLock<Stocks>>), Error = Error> {
    let limiter = start_rate_limiter();
    let stocks = load_stock_info(config.filepath.clone());
    let display = stocks.and_then(|s| start_display(s));

    limiter
        .join(display)
        .map(|(limiter, (displayer, stocks))| (config, displayer, limiter, stocks))
}

fn start_rate_limiter() -> impl Future<Item = Limiter, Error = Error> {
    future::ok(Limiter::new(5, Duration::from_secs(62)))
}

fn start_display(
    stocks: Stocks,
) -> impl Future<Item = (Displayer, Arc<RwLock<Stocks>>), Error = Error> {
    let stocks_arc = Arc::new(RwLock::new(stocks));
    future::ok((Displayer::new(stocks_arc.clone()), stocks_arc))
}

fn load_stock_info(filepath: PathBuf) -> impl Future<Item = Stocks, Error = Error> {
    wfwalk::tree::read_tree_async(filepath).and_then(|tree| Stocks::load_from_tree(&tree))
}

////fn make_a_task(num: u8) -> impl Future<Item = (), Error = ()> {
////    ok(Instant::now()).map(move |i| println!("Task: {}/{}", num, i.elapsed().as_micros()))
////}
//
fn maybe_sanity_check(config: &Config, stocks: &Stocks) -> bool {
    if !config.do_sanity_check {
        return false;
    }

    for (_, stock) in stocks.stocks.iter() {
        let sanity = sanity_check(&stock);
        if sanity.len() > 0 {
            println!("{}", stock.symbol);
            for warning in sanity {
                println!("  {}", warning);
            }
        }
    }
    true
}

fn query_task(
    symbol: String,
    token: String,
    stocks: Arc<RwLock<Stocks>>,
    displayer: Arc<RwLock<Displayer>>,
) -> impl Future<Item = (), Error = ()> {
    println!("QUERY TASK: {}", symbol);
    wfwalk::alphavantage::daily(symbol.clone(), token)
        .map(move |daily_result| {
            // unwrap: TODO: really bad.
            let mut stocks = stocks.write().unwrap();
            daily_result.last_price().map(move |lp| {
                stocks.stocks.entry(symbol).and_modify(|s| {
                    s.last_price.replace(lp);
                });
            });
            //            let mut stock = stocks.stocks[symbol];
            //            stock.last_price = daily_result.last_price();
            // unwrap: TODO: really bad.
            displayer.write().unwrap().refresh();
        })
        //        .inspect(move |_| println!("IN RUNNER: {}", symbol))
        .map_err(|_| ())
}

fn run(
    params: (Config, Displayer, Limiter, Arc<RwLock<Stocks>>),
) -> impl Future<Item = (), Error = Error> {
    let (config, displayer, mut limiter, stocks) = params;

    // unwrap: TODO No good.
    if maybe_sanity_check(&config, &stocks.read().unwrap()) {
        ok(())
    } else {
        let mut err = None;
        let stocks_clone = stocks.clone();
        let displayer_arc = Arc::new(RwLock::new(displayer));
        for stock in stocks.read().unwrap().stocks.values() {
            let token = config.token.clone();
            let symbol = stock.symbol.clone();
            let stocks_for_task = stocks_clone.clone();
            let displayer_for_task = displayer_arc.clone();
            if let Err(e) = limiter.add_task(future::lazy(move || {
                query_task(symbol, token.clone(), stocks_for_task, displayer_for_task)
            })) {
                err = Some(e);
                break;
            }
        }

        if let Some(e) = err {
            limiter.quit();
            return future::err(e);
        }

        ok(())
    }
}

fn cleanup(_: ()) -> impl Future<Item = (), Error = Error> {
    future::ok(())
}

fn real_main() -> Result<()> {
    let config = args::Config::new()?;
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
