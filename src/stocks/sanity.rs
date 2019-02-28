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

fn sanity_check(stock: Stock) -> Vec<Insanity> {
    let mut insanities = vec![];

    let tests: Vec<fn(&Stock) -> Option<Insanity>> = vec![
        has_name,
        has_brokerage_tag,
        has_short_tag_if_needed,
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
        .then(|| "has no short tag".into())
}

fn has_portfolio_tag(stock: &Stock) -> Option<Insanity> {
    None
}

fn has_lots(stock: &Stock) -> Option<Insanity> {
    None
}
