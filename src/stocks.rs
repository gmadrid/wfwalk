use crate::errors::*;
use crate::tree::read_tree;
use crate::tree::ArenaIndex;
use crate::tree::NTree;
use std::collections::hash_map::HashMap;
use std::fs::File;
use std::io::BufReader;

type StockTree = NTree<String>;

pub struct Stocks {
    tree: NTree<String>,
    stocks_index: ArenaIndex,
    stocks: HashMap<String, Stock>,
}

#[derive(Debug)]
pub struct Stock {
    symbol: String,
}

impl Stocks {
    pub fn load() -> Result<Stocks> {
        let f =
            File::open("/Users/gmadrid/Dropbox/Apps/WorkFlowy/WorkFlowy (gmadrid@gmail.com).txt")?;
        let bufread = BufReader::new(f);
        let tree = read_tree(bufread, Some("-"))?;

        let stocks_index = Stocks::find_stocks(&tree)?;

        let mut stocks = HashMap::new();
        for stock_index in tree.children(stocks_index)?.iter() {
            let stock = Stocks::load_stock(&tree, *stock_index)?;
            stocks.insert(stock.symbol.clone(), stock);
        }

        dbg!(&stocks);

        Ok(Stocks {
            tree,
            stocks_index,
            stocks,
        })
    }

    fn load_stock(tree: &StockTree, stock_index: ArenaIndex) -> Result<Stock> {
        let pieces: Vec<&str> = tree.value(stock_index)?.split("-").collect();
        let symbol = pieces[0].trim().to_string();

        Ok(Stock { symbol })
    }

    fn find_stocks(tree: &StockTree) -> Result<ArenaIndex> {
        let finance_idx = tree
            .bf_iter()
            .find_map(|(idx, val)| {
                if val.trim() == "Finance" {
                    Some(idx)
                } else {
                    None
                }
            })
            .ok_or_else(|| ErrorKind::Msg("Failed to find 'Finance' node.".into()))?;

        let stocks_idx = tree
            .bf_iter_from(finance_idx)
            .find_map(|(idx, val)| {
                if val.trim() == "Stocks" {
                    Some(idx)
                } else {
                    None
                }
            })
            .ok_or_else(|| ErrorKind::Msg("Failed to find 'Stocks' node.".into()))?;

        Ok(stocks_idx)
    }
}

mod parser {
    // Expected format of the tree:
    // 'Stocks'
    //   <Stock>*
    //     'Lots'
    //       <Lot>
    //
    // Stock   := Symbol '-' [ Name '-' ]? Num '-' Tags
    // Symbol  := /[A-Z.]*/
    // Name    := StringWithSpaces
    // Num     := A floating point number (no scientific notation)
    // Tags    := Tag*
    // Tag     := /[@#][A-Za-z0-9]*/
    //
    // Lot     := ???????
    //
    // All whitespace is ignored except in StringWithSpaces.

    use crate::errors::*;

    fn parse_tag(str: &str) -> Result<&str> {
        let result = str.trim();
        if !result.starts_with("@") && !result.starts_with("#") {
            bail!(ErrorKind::BadParse(
                "TAG".to_string(),
                "must begin with '@' or '#'.".to_string(),
                "".to_string()
            ));
        }

        // TODO: check for valid characters as described above in BNF
        if result.contains(char::is_whitespace) {
            bail!("Tag cannot contain white space.");
        }

        Ok(str.trim())
    }

    fn parse_tags(str: &str) -> Result<Vec<&str>> {
        let mut result = Vec::new();
        for tag in str.split_whitespace() {
            result.push(parse_tag(tag)?);
        }
        Ok(result)
    }

    fn parse_num(str: &str) -> Result<f32> {
        Ok(str.trim().parse()?)
    }

    fn parse_name(str: &str) -> Result<&str> {
        Ok(str.trim())
    }

    fn parse_symbol(str: &str) -> Result<&str> {
        let result = str.trim();

        // Test for _invalid_ characters.
        let test = result.contains(|c: char| {
            !c.is_ascii_uppercase() && c != '.'
        });

        if test {
            bail!(ErrorKind::BadParse(
            "SYMBOL".to_string(),
            "must contain only A-Z and '.'".to_string(),
            "".to_string()
            ));
        }

        Ok(result)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_symbol() {
            assert_eq!("AAPL", parse_symbol("AAPL").unwrap());
            assert_eq!("AAPL", parse_symbol("   AAPL").unwrap());
            assert_eq!("AAPL", parse_symbol("AAPL   ").unwrap());
            assert_eq!("AAPL", parse_symbol("  AAPL    ").unwrap());

            assert_eq!("BRK.B", parse_symbol("  BRK.B  ").unwrap());

            assert!(parse_symbol("NO SPACES").is_err());
            assert!(parse_symbol("nolower").is_err());
            assert!(parse_symbol("nopunct!").is_err());
        }

        #[test]
        fn test_name() {
            assert_eq!("quux", parse_name("quux").unwrap());
            assert_eq!("quux bar", parse_name("quux bar").unwrap());
            assert_eq!("quux bar", parse_name("   quux bar").unwrap());
            assert_eq!("quux bar", parse_name("quux bar   ").unwrap());
            assert_eq!("quux bar", parse_name("    quux bar   ").unwrap());

            assert_eq!("!#@# \t\n!!@@", parse_name("  !#@# \t\n!!@@     ").unwrap());
        }

        #[test]
        fn test_num() {
            assert_eq!(3.14, parse_num("3.14").unwrap());
            assert_eq!(-3.14, parse_num("-3.14").unwrap());
            assert_eq!(3.14, parse_num("  3.14").unwrap());
            assert_eq!(3.14, parse_num("3.14  ").unwrap());
            assert_eq!(3.14, parse_num("   3.14  ").unwrap());

            assert_eq!(3.0, parse_num("3").unwrap());
            assert_eq!(-3.0, parse_num("-3").unwrap());

            assert_eq!(0.0, parse_num("0").unwrap());
            assert_eq!(0.0, parse_num("000").unwrap());
            assert_eq!(0.0, parse_num("-000").unwrap());

            assert!(parse_num("xxx").is_err());
        }

        #[test]
        fn test_tags() {
            assert_eq!(Vec::<&'static str>::new(), parse_tags("").unwrap());
            assert_eq!(vec!["@one"], parse_tags("@one").unwrap());
            assert_eq!(
                vec!["@one", "#two", "@three"],
                parse_tags("@one #two @three").unwrap()
            );
            assert_eq!(
                vec!["@one", "#two", "@three"],
                parse_tags("  @one #two @three  ").unwrap()
            );
            assert_eq!(
                vec!["@one", "#two", "@three"],
                parse_tags("@one    #two   @three").unwrap()
            );
        }

        #[test]
        fn test_tag() {
            assert_eq!("@foo", parse_tag("@foo").unwrap());
            assert_eq!("@foo", parse_tag("   @foo").unwrap());
            assert_eq!("@foo", parse_tag("@foo   ").unwrap());
            assert_eq!("@foo", parse_tag("   @foo   ").unwrap());

            assert_eq!("#quux", parse_tag("  #quux  ").unwrap());

            // TODO: check the errors for what I expect.
            assert!(parse_tag("xxx").is_err());
            assert!(parse_tag("@ foo").is_err());
        }

    }

}
