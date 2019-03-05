use tokio::prelude::Future;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::timer::DelayQueue;
use tokio::prelude::Async;
use tokio::prelude::Stream;
use tokio::fs::File;
use tokio::prelude::future;
use std::time::Duration;

type Task = dyn Future<Item = (), Error = ()> + Send;

enum Request {
    AddTask(Box<Task>),
    Quit
}

pub struct Limiter {
    sender: UnboundedSender<Request>,
}

struct Runner  {
    queue: DelayQueue<Box<Task>>,
    receiver: UnboundedReceiver<Request>,
}

impl Limiter {
    pub fn new() -> Limiter {
        let (sender, receiver) = unbounded_channel();

        let mut runner = Runner {
            queue: DelayQueue::new(),
            receiver,
        };
        runner.queue.insert(Box::new(future::ok(())), Duration::new(60 * 50 * 24, 0));
        tokio::spawn(runner);

        Limiter { sender }
    }

    // TODO: error handling
    pub fn add_task<T>(&mut self, task: T) where T: Future<Item = (), Error = ()> + Send + 'static {
        println!("adding task");
        // TODO: get this unwrap out of here.
        self.sender.try_send(Request::AddTask(Box::new(task)));
    }

    // TODO: error handling
    pub fn quit(&mut self) {
        self.sender.try_send(Request::Quit);
    }
}

impl Future for Runner {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        loop {
            // TODO: deal with this error.
            println!("Polling");
            let request = try_ready!(self.receiver.poll().map_err(|_| ()));

            println!("got a thing");
            match request {
                Some(Request::AddTask(t)) => {
                    println!("adding a task");
                    self.queue.insert(t, Duration::new(5, 0));
                },
                Some(Request::Quit) => break,
                None => println!("Stream done"),
            };
        }
        Ok(Async::Ready(()))
    }
}