use std::collections::HashSet;
use std::iter::FromIterator;

use super::Stock;
use crate::type_tools::{BoolTools, OptionTools, VecTools};

lazy_static! {
    static ref BROKERAGE_TAGS: HashSet<String> =
        { HashSet::from_iter(vec!["@etrade", "@ally"].to_strings()) };
    static ref PORTFOLIO_TAGS: HashSet<String> =
        { HashSet::from_iter(vec!["@ally", "@longshort", "@marijuana", "@misc"].to_strings()) };
}

type Insanity = String;

pub fn sanity_check(stock: &Stock) -> Vec<Insanity> {
    let mut insanities = vec![];

    let tests: Vec<fn(&Stock) -> Option<Insanity>> = vec![
        has_name,
        has_brokerage_tag,
        has_short_tag_if_needed,
        no_short_if_not_needed,
        has_portfolio_tag,
        has_lots,
    ];

    for test in tests {
        test(&stock).map(|i| insanities.push(i));
    }

    insanities
}

fn has_name(stock: &Stock) -> Option<Insanity> {
    stock.name.not(|| "missing name.".into())
}

fn has_brokerage_tag(stock: &Stock) -> Option<Insanity> {
    stock
        .tags
        .is_disjoint(&BROKERAGE_TAGS)
        .then(|| "has no brokerage tag".into())
}

fn has_short_tag_if_needed(stock: &Stock) -> Option<Insanity> {
    (stock.num < 0.0 && !stock.tags.contains(&"@short".to_string()))
        .then(|| "has no @short tag".into())
}

fn no_short_if_not_needed(stock: &Stock) -> Option<Insanity> {
    (stock.num > 0.0 && stock.tags.contains(&"@short".to_string()))
        .then(|| "shouldn't have @short tag".into())
}

fn has_portfolio_tag(stock: &Stock) -> Option<Insanity> {
    None
}

fn has_lots(stock: &Stock) -> Option<Insanity> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stocks::stocks::Stock;

    fn make_stock(symbol: &str, name: Option<&str>, num: f32, tags: Vec<&str>) -> Stock {
        Stock {
            symbol: symbol.to_string(),
            name: name.map(|s| s.to_string()),
            num: num,
            tags: HashSet::from_iter(tags.to_strings().into_iter()),
        }
    }

    fn make_apple() -> Stock {
        make_stock(
            "AAPL",
            Some("Apple Computer"),
            100.0,
            vec!["@etrade", "@longshort"],
        )
    }

    #[test]
    fn test_has_name() {
        assert!(has_name(&make_apple()).is_none());
        assert!(has_name(&Stock {
            name: None,
            ..make_apple()
        })
        .is_some());
    }

    #[test]
    fn test_has_brokerage_tag() {
        assert!(has_brokerage_tag(&make_apple()).is_none());
        assert!(has_brokerage_tag(&Stock {
            tags: HashSet::new(),
            ..make_apple()
        })
        .is_some());
    }

    #[test]
    fn test_has_short_tag() {
        let short = Stock {
            num: -100.0,
            ..make_apple()
        };

        assert!(has_short_tag_if_needed(&make_apple()).is_none());
        assert!(has_short_tag_if_needed(&short).is_some());
        assert!(has_short_tag_if_needed(&Stock {
            tags: HashSet::from_iter(vec!["@short".to_string()].into_iter()),
            ..short
        })
        .is_none());
    }

}
