use crate::errors::*;
use crate::tree::build_data::BuildData;
use crate::tree::NTree;
use std::io::BufReader;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::lines;
use tokio::prelude::*;

pub fn read_tree_async<P>(path: P) -> impl Future<Item = NTree<String>, Error = Error>
where
    P: Into<PathBuf>,
{
    File::open(path.into())
        .map_err(|e| Error::with_chain(e, "failed to open file"))
        .and_then(|file| {
            let build_data = BuildData::new(Some("-".to_string()));
            let stream = BufReader::new(file);
            lines(stream)
                .map_err(|e| Error::with_chain(e, "failed while reading lines"))
                .fold(build_data, |mut bd, line| -> Result<BuildData> {
                    bd.add(&line)?;
                    Ok(bd)
                })
        })
        .map(|bd| bd.tree)
}
