use std::fs::File;
use std::io::BufReader;

// Need this for bail!
//#[macro_use]
//extern crate error_chain;

use wfwalk::errors::*;
use wfwalk::tree::{ArenaIndex, NTree, read_tree};

fn find_stocks(tree: &NTree<String>) -> Result<ArenaIndex> {
    let finance_idx = tree.bf_iter().find_map(|(idx, val)| {
        if val.trim() == "Finance" {
            Some(idx)
        } else {
            None
        }
    }).ok_or_else(|| ErrorKind::Msg("Failed to find 'Finance' node.".into()))?;

    let stocks_idx = tree.bf_iter_from(finance_idx).find_map(|(idx, val)| {
        if val.trim() == "Stocks" {
            Some(idx)
        } else {
            None
        }
    }).ok_or_else(|| ErrorKind::Msg("Failed to find 'Stocks' node.".into()))?;

    Ok(stocks_idx)
}

fn main() -> Result<()> {
    let f = File::open("/Users/gmadrid/Dropbox/Apps/WorkFlowy/WorkFlowy (gmadrid@gmail.com).txt")?;

    let bufread = BufReader::new(f);
    let tree = read_tree(bufread, Some("-")).unwrap();

    println!("{:?}", find_stocks(&tree));
    println!("{}", tree);

    Ok(())
}
