use super::parser;
use crate::errors::*;
use crate::stocks::sanity::sanity_check;
use crate::tree::read_tree;
use crate::tree::ArenaIndex;
use crate::tree::NTree;
use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use std::fs::File;
use std::io::BufReader;

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
    pub lots: Vec<Lot>,
    pub last_price: Option<f32>,
}

#[derive(Debug, PartialEq)]
pub struct Lot {}

impl Stocks {
    pub fn load_from_file() -> Result<Stocks> {
        let f =
            File::open("/Users/gmadrid/Dropbox/Apps/WorkFlowy/WorkFlowy (gmadrid@gmail.com).txt")?;
        let bufread = BufReader::new(f);
        let tree = read_tree(bufread, Some("-"))?;

        Stocks::load_from_tree(&tree)
    }

    pub fn load_from_tree(tree: &NTree<String>) -> Result<Stocks> {
        let stocks_index = Stocks::find_stocks_node(&tree)?;
        let mut stocks = HashMap::new();
        for stock_index in tree.children(stocks_index)?.iter() {
            let str = tree.value(*stock_index)?;
            let stock = parser::parse_stock(str)?;
            stocks.insert(stock.symbol.clone(), stock);
        }
        Ok(Stocks { stocks })
    }

    pub fn sanity_check(&self) -> HashMap<String, Vec<String>> {
        self.stocks.values().fold(HashMap::new(), |mut acc, stock| {
            let sanity = sanity_check(stock);
            if !sanity.is_empty() {
                acc.insert(stock.symbol.clone(), sanity);
            }
            acc
        })
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
