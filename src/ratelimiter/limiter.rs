use std::collections::VecDeque;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::time::Duration;
use std::time::Instant;

use tokio::prelude::{Async, Future, Stream};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::timer::delay_queue::{DelayQueue, Expired};

use crate::errors::*;
use crate::tokio_tools;

type Task = dyn Future<Item = (), Error = ()> + Send;
type BoxedTask = Box<Task>;

enum Request {
    AddTask(Box<Task>),
    Quit,
}

// A wrapper for the different results we may receive while polling.
enum Polled {
    Request(Request),
    Task(Expired<BoxedTask>),
}

impl From<Request> for Polled {
    fn from(r: Request) -> Self {
        Polled::Request(r)
    }
}

impl From<Expired<BoxedTask>> for Polled {
    fn from(expired: Expired<BoxedTask>) -> Self {
        Polled::Task(expired)
    }
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
    // Channel receiving Requests to alter the queue.
    receiver: UnboundedReceiver<Request>,

    // DelayQueue with Tasks which are waiting to run.
    queue: DelayQueue<BoxedTask>,

    // Times of the last tasks run since Instant::now() - self.duration.
    run_instants: VecDeque<Instant>,

    // Limit tasks to num_requests/duration.
    num_requests: usize,
    duration: Duration,
}

impl Limiter {
    pub fn new(num_requests: usize, duration: Duration) -> Limiter {
        let (sender, receiver) = unbounded_channel();

        let runner = Runner {
            receiver,
            queue: DelayQueue::new(),
            run_instants: VecDeque::new(),

            // 2 reqs/5 secs
            num_requests,
            duration,
        };
        tokio::spawn(tokio_tools::erase_types(runner));

        Limiter { sender }
    }

    pub fn add_task<T>(&mut self, task: T) -> Result<()>
    where
        T: Future<Item = (), Error = ()> + Send + 'static,
    {
        let req = Request::AddTask(Box::new(task));
        self.sender
            .try_send(req)
            .chain_err(|| "Error in add_task()")
    }

    pub fn quit(&mut self) -> Result<()> {
        self.sender
            .try_send(Request::Quit)
            .chain_err(|| "Error in quit()")
    }
}

fn normalize<'a, S, T, E>(s: &'a mut S) -> impl Stream<Item = Polled, Error = Error> + 'a
where
    S: Stream<Item = T, Error = E>,
    E: Into<Error>,
    T: Into<Polled>,
{
    s.map(|t| t.into()).map_err(|e| e.into())
}

impl Runner {
    fn compute_next_task_time(&mut self) -> Instant {
        let cutoff = Instant::now() - self.duration;
        self.run_instants = self
            .run_instants
            .iter()
            .map(|i| *i)
            .filter(|i| *i > cutoff)
            .collect();
        if self.run_instants.len() < self.num_requests {
            Instant::now()
        } else {
            let index = self.run_instants.len() - self.num_requests;
            self.run_instants[index] + self.duration
        }
    }
}

impl Future for Runner {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Self::Item>> {
        loop {
            let polled = {
                let receiver_future = normalize(self.receiver.by_ref());
                let queue_future = normalize(self.queue.by_ref());
                try_ready!(queue_future.select(receiver_future).poll())
            };

            match polled {
                None => {
                    // TODO: what should I really do here?
                    trace!("Limiter is done. Quitting.");
                    break;
                }
                Some(Polled::Task(expired)) => {
                    tokio::spawn(expired.into_inner());
                }
                Some(Polled::Request(Request::Quit)) => {
                    trace!("Limiter received shutdown. Clearing waiting tasks and quitting.");
                    break;
                }
                Some(Polled::Request(Request::AddTask(t))) => {
                    let new_instant = self.compute_next_task_time();
                    self.run_instants.push_back(new_instant);
                    self.queue.insert_at(t, new_instant);
                    trace!(
                        "New task scheduled at: {}",
                        new_instant.elapsed().as_millis()
                    );
                }
            }
        }
        Ok(Async::Ready(()))
    }
}
