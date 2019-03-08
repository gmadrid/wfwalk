use tokio::prelude::future;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
//use tokio::timer::DelayQueue;
use crate::errors::*;
use crate::tokio_tools;
use std::fmt::Debug;
use std::fmt::Formatter;
use tokio::prelude::future::Future;
use tokio::prelude::Async;
use tokio::prelude::Stream;

type Task = dyn future::Future<Item = (), Error = ()> + Send;

enum Request {
    AddTask(Box<Task>),
    Quit,
}

impl Debug for Request {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Request::AddTask(_) => write!(f, "AddTask"),
            Request::Quit => write!(f, "Quit"),
        }
    }
}

pub struct Limiter {
    sender: UnboundedSender<Request>,
}

struct Runner {
    receiver: UnboundedReceiver<Request>,
}

//fn map_send_error<T>(result: std::err::Result<T,UnboundTrySendError>) -> Result<T> {
//    let e: Option<::std::io::Error> = None;
//}
//
impl Limiter {
    pub fn new() -> Limiter {
        let (sender, receiver) = unbounded_channel();

        let runner = Runner { receiver };
        tokio::spawn(tokio_tools::erase_types(runner));

        Limiter {
            sender: sender.clone(),
        }
    }

    // TODO: error handling
    pub fn add_task<T>(&mut self, task: T) -> Result<()>
    where
        T: Future<Item = (), Error = ()> + Send + 'static,
    {
        println!("adding task");
        let req = Request::AddTask(Box::new(task));
        self.sender.try_send(req).chain_err(|| "Error in add_task")
    }

    // TODO: error handling
    pub fn quit(&mut self) -> Result<()> {
        self.sender
            .try_send(Request::Quit)
            .chain_err(|| "Error in quit()")
    }
}

impl Future for Runner {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Self::Item>> {
        loop {
            // TODO: deal with this error.
            //println!("Polling");
            let request = try_ready!(self.receiver.poll()); //.map_err(|_| "TODO: FIX THIS".into()));

            //println!("got a thing: {:?}", request);
            match request {
                Some(Request::AddTask(_t)) => {
                    info!("Adding a task");
                }
                Some(Request::Quit) => break,
                None => break,
            };
        }
        Ok(Async::Ready(()))
    }
}
