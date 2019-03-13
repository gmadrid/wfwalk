use std::fmt::Debug;
use std::fmt::Formatter;
use std::time::Duration;
use std::time::Instant;

use tokio::prelude::{future, Async, Future, Stream};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::timer::delay_queue::{DelayQueue, Expired};

use crate::errors::*;
use crate::tokio_tools;
use std::collections::{HashMap, VecDeque};

type Task = dyn Future<Item = (), Error = ()> + Send;

enum Request {
    AddTask(Box<Task>),
    Quit,
}

// A wrapper for the different results we may receive while polling.
enum Polled {
    Request(Request),
    Timer,
}

impl From<Request> for Polled {
    fn from(r: Request) -> Self {
        Polled::Request(r)
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
    // Timer to the next task.
    timer: Option<u32>, // TODO: pick a type for this

    // Tasks waiting to fire.
    tasks: VecDeque<Box<Task>>,

    // Times of the last tasks that run.
    run_instants: VecDeque<Instant>, // TODO: use Instant here?

    // Limit tasks to num_requests/duration.
    num_requests: usize,
    duration: Duration,
}

impl Limiter {
    pub fn new() -> Limiter {
        let (sender, receiver) = unbounded_channel();

        let runner = Runner {
            receiver,
            timer: None,
            tasks: VecDeque::default(),
            run_instants: VecDeque::default(),

            // 2 reqs/5 secs
            num_requests: 2,
            duration: Duration::from_secs(5),
        };
        tokio::spawn(tokio_tools::erase_types(runner));

        Limiter { sender }
    }

    pub fn add_task<T>(&mut self, task: T) -> Result<()>
    where
        T: Future<Item = (), Error = ()> + Send + 'static,
    {
        println!("adding task");
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
    fn try_to_run_task(&mut self) {
        println!("TRYING TO RUN TASK");
            if let Some(task) = self.tasks.pop_front() {
                println!("SPAWNING TASK");
                tokio::spawn(task);
            }
    }
}

impl Future for Runner {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Self::Item>> {
        loop {
            let mut polled: Option<Polled> = None;
            {
                // stream must be scoped so that it drops the mut ref to self.
                // TODO: add timer logic here.
                let mut stream = normalize(self.receiver.by_ref());
                polled = try_ready!(stream.poll());
            }
            match polled {
                None => {
                    // TODO: what should I really do here?
                    println!("GOT NONE");
                    break;
                }
                Some(Polled::Timer) => unimplemented!(),
                Some(Polled::Request(Request::Quit)) => break,
                Some(Polled::Request(Request::AddTask(t))) => {
                    println!("GOT A TASK");
                    self.tasks.push_back(t);
                    self.try_to_run_task();
                }
            }
        }

        //            let q = normalize(self.queue.by_ref());
        //            let r = normalize(self.receiver.by_ref());
        //            let mut sel = q.select(r);
        //
        //            let polled = try_ready!(sel.poll());
        //            match polled {
        //                None => {
        //                    println!("End of streams");
        //                    break;
        //                },
        //                Some(PollResult::Request(Request::Quit)) => {
        //                    println!("Quitting");
        //                    break;
        //                },
        //                Some(PollResult::Request(Request::AddTask(t))) => {
        //                    println!("Adding Task");
        //                    self.queue.insert_at(t, Instant::now() + Duration::from_secs(1));
        //                },
        //                Some(PollResult::Task(expired)) => {
        //                    println!("EXPIRED");
        //                }
        //            }
        Ok(Async::Ready(()))
    }
}
