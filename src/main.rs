use std::fs::File;
use std::io::Result;
use std::io::BufReader;

use wfwalk::tree::read_tree;

fn main() -> Result<()> {
    let f = File::open("/Users/gmadrid/Dropbox/Apps/WorkFlowy/WorkFlowy (gmadrid@gmail.com).txt")?;
    let bufread = BufReader::new(f);
    let tree = read_tree(bufread, Some("-")).unwrap();

    println!("{}", tree);

    Ok(())
}