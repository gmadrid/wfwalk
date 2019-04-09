use itertools::Itertools;
use std::sync::Arc;
use std::sync::RwLock;
use tokio::prelude::{Async, Future, Stream};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::errors::*;
use crate::stocks::Stocks;
use crate::tokio_tools;

enum Request {
    Refresh,
}

pub struct Displayer {
    sender: UnboundedSender<Request>,
}

impl Displayer {
    pub fn new(stocks: Arc<RwLock<Stocks>>) -> Displayer {
        let (sender, receiver) = unbounded_channel();

        let runner = DisplayerRunner { receiver, stocks };
        tokio::spawn(tokio_tools::erase_types(runner));

        Displayer { sender }
    }

    pub fn refresh(&mut self) -> Result<()> {
        self.sender
            .try_send(Request::Refresh)
            // TODO: Why doesn't chain_err work?
            //.chain_err(|| "Error in refresh()")
            .map_err(|_| ErrorKind::WeirdError("Error in refresh").into())
    }
}

struct DisplayerRunner {
    receiver: UnboundedReceiver<Request>,
    stocks: Arc<RwLock<Stocks>>,
}

impl DisplayerRunner {
    fn spew(&self) {
        // Clear the screen.
        println!("\x1b[2J\x1b[1;1H");

        // unwrap: TODO deal with error here.
        let stocks = self.stocks.read().unwrap();
        for stock in stocks
            .stocks
            .values()
            .sorted_by(|l, r| Ord::cmp(&l.symbol, &r.symbol))
        {
            println!("{}: {:?}", stock.symbol, stock.last_price);
        }
    }
}

impl Future for DisplayerRunner {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Self::Item>> {
        loop {
            let request = try_ready!(self.receiver.poll());
            match request {
                None => {
                    // All done.
                    // TODO: is this really the right thing.
                    println!("Displayer quitting"); // TODO: remove this.
                    break;
                }
                _ => {
                    self.spew();
                }
            }
        }
        Ok(Async::Ready(()))
    }
}
