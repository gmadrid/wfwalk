use crate::errors::*;
use crate::tree::read_tree;
use crate::tree::ArenaIndex;
use crate::tree::NTree;

use std::collections::hash_set::HashSet;
use std::collections::hash_map::HashMap;
use std::fs::File;
use std::io::BufReader;

use super::parser;

type StockTree = NTree<String>;

#[derive(Debug)]
pub struct Stocks {
    pub stocks: HashMap<String, Stock>,
}

#[derive(Debug, PartialEq)]
pub struct Stock {
    pub symbol: String,
    pub name: Option<String>,
    pub num: f32,
    pub tags: HashSet<String>,
}

impl Stocks {
    pub fn load() -> Result<Stocks> {
        let f =
            File::open("/Users/gmadrid/Dropbox/Apps/WorkFlowy/WorkFlowy (gmadrid@gmail.com).txt")?;
        let bufread = BufReader::new(f);
        let tree = read_tree(bufread, Some("-"))?;

        let stocks_index = Stocks::find_stocks_node(&tree)?;

        let mut stocks = HashMap::new();
        for stock_index in tree.children(stocks_index)?.iter() {
            let str = tree.value(*stock_index)?;
            let stock = parser::parse_stock(str)?;
            stocks.insert(stock.symbol.clone(), stock);
        }

        Ok(Stocks { stocks })
    }

    fn find_stocks_node(tree: &StockTree) -> Result<ArenaIndex> {
        let finance_idx = tree
            .find_node(tree.root_index(), |(_, val)| val.trim() == "Finance")
            .ok_or_else(|| ErrorKind::Msg("Failed to find 'Finance' node".into()))?;
        let stocks_idx = tree
            .find_node(finance_idx, |(_, val)| val.trim() == "Stocks")
            .ok_or_else(|| ErrorKind::Msg("Failed to find 'Stocks' node.".into()))?;

        Ok(stocks_idx)
    }
}
