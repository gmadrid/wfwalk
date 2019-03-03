use crate::tree::build_data::BuildData;
use futures::future::FutureResult;
use std::io::BufReader;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::lines;
use tokio::prelude::*;

pub fn read_tree_async<P>(path: P) -> impl Future<Item = BuildData, Error = std::io::Error>
where
    P: Into<PathBuf>,
{
    File::open(path.into()).and_then(|file| {
        let mut build_data = BuildData::new(Some("-".to_string()));

        let stream = BufReader::new(file);
        let ff = |mut bd: BuildData, line: String| -> FutureResult<BuildData, std::io::Error> {
            bd.add(&line);
            println!("LFD: {}", line);
            future::ok(bd)
        };
        lines(stream).fold(build_data, ff)
    })
}
