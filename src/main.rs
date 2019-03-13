#[macro_use]
extern crate clap;

use std::sync::Arc;

use tokio::prelude::*;

use futures::future::ok;
use std::time::Duration;
use std::time::Instant;
use wfwalk::errors::*;
use wfwalk::ratelimiter::Limiter;
use wfwalk::stocks::Stocks;
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
    future::ok(Limiter::new(5, Duration::from_secs(60)))
}

fn load_stock_info(config: Config) -> impl Future<Item = Stocks, Error = Error> {
    wfwalk::tree::read_tree_async(config.filepath.clone())
        .and_then(|tree| Stocks::load_from_tree(&tree))
}

fn make_a_task(num: u8) -> impl Future<Item = (), Error = ()> {
    ok(Instant::now()).map(move |i| println!("Task: {}/{}", num, i.elapsed().as_micros()))
}

fn run(params: (Config, Limiter, Stocks)) -> impl Future<Item = (), Error = Error> {
    let (config, mut limiter, stocks) = params;

    let r = limiter
        .add_task(make_a_task(1))
        .and_then(|_| limiter.add_task(make_a_task(2)))
        .and_then(|_| limiter.add_task(make_a_task(3)))
        .and_then(|_| limiter.add_task(make_a_task(4)))
        .and_then(|_| limiter.add_task(make_a_task(5)))
        .and_then(|_| limiter.add_task(make_a_task(6)))
        .and_then(|_| limiter.add_task(make_a_task(7)))
        .and_then(|_| limiter.add_task(make_a_task(8)));

    future::result(r)
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
