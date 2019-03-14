use std::fmt::Debug;
use tokio::prelude::Future;

pub fn erase_types<F, I, E>(f: F) -> impl Future<Item = (), Error = ()>
where
    F: Future<Item = I, Error = E>,
    E: Debug,
{
    f.map(|_| ()).map_err(|e| eprintln!("Error: {:?}", e))
}
