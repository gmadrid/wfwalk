use std::fmt::Debug;
use std::fmt::Formatter;
use std::time::Duration;
use std::time::Instant;

use tokio::prelude::{future, Async, Future, Stream};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::timer::delay_queue::{DelayQueue, Expired};

use crate::errors::*;
use crate::tokio_tools;
use crate::ratelimiter::limiter::PollState::{PollingQueue, PollingReceiver};

type Task = dyn Future<Item = (), Error = ()> + Send;

enum Request {
    AddTask(Box<Task>),
    Quit,
}

// A wrapper for the different results we may receive while polling.
enum PollResult {
    Request(Request),
    Task(Expired<Box<Task>>),
}

impl From<Request> for PollResult {
    fn from(r: Request) -> Self {
        PollResult::Request(r)
    }
}

impl From<Expired<Box<Task>>> for PollResult {
    fn from(t: Expired<Box<Task>>) -> Self {
        PollResult::Task(t)
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
    receiver: Option<UnboundedReceiver<Request>>,
    queue: DelayQueue<Box<Task>>,

    times: Vec<Instant>,
    num_requests: u32,
    duration: Duration,
}

impl Limiter {
    pub fn new() -> Limiter {
        let (sender, receiver) = unbounded_channel();

        let runner = Runner {
            receiver,
            queue: DelayQueue::new(),
            times: Vec::new(),

            // 2 reqs/5 secs
            num_requests: 2,
            duration: Duration::from_secs(5),
        };
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

fn normalize<'a, S, T, E>(s: &'a mut S) -> impl Stream<Item = PollResult, Error=Error> + 'a
    where S: Stream<Item = T, Error = E>,
    E: Into<Error>,
    T: Into<PollResult>,
{
    s.map(|t| t.into()).map_err(|e| e.into())
}

enum PollState {
    PollingQueue,
    PollingReceiver,
}

impl Future for Runner {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Self::Item>> {
        use PollState::*;
        // TODO: maybe this state should be in the Runner.
        let mut state = PollingQueue;
        loop {
            match state {
                PollingQueue => {
                    if (self.receiver.is_some()) {
                        state = PollingReceiver;
                    }
                    match self.queue.poll() {
                        Err(e) => return Err(e.into()),
                        Ok(Async::NotReady) => {
                            // TODO: have to handle this NOW!
                        }
                        Ok(Async::Ready(None)) => { println!("GOTTA HANDLE THIS TOO"); }
                        Ok(Async::Ready(Some(task))) => {
                            println!("GOT A TASK!!!!!!!!!!");
                        }
                    }
                }

                PollingReceiver => {
                        state = PollingQueue;
                    match self.receiver.poll() {
                        Err(e) => return Err(e.into()),
                        Ok(Async::NotReady) => {
                            // TODO: Do something here to make the bouncing happen.
                            return Ok(Async::NotReady)
                        }
                        Ok(Async::Ready(req)) => {
                            // TODO: Do something with None value here.
                            match req {
                                None => { println!("GOTTA HANDLE THIS"); break; }
                                Some(Request::Quit) => {
                                    println!("QUITTING");
                                    break;
                                }
                                Some(Request::AddTask(task)) => {
                                    println!("ADDING TASK");
                                    self.queue.insert_at(task, Instant::now() + Duration::from_secs(1));
                                }
                            }
                        }
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
        }
        Ok(Async::Ready(()))
    }
}
