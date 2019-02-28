use std::collections::HashSet;
use std::iter::FromIterator;

use super::Stock;

lazy_static! {
    static ref brokerage_tags: HashSet<String> = {
        HashSet::from_iter(as_strings(vec!["@etrade", "@ally"]))
    };
}

type Insanity = String;

fn sanity_check(stock: Stock) -> Vec<Insanity> {
    let mut insanities = vec![];

    let tests: Vec<fn(&Stock)->Option<Insanity>> = vec![
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

trait MapNone {
    fn map_none<U, F>(&self, f: F) -> Option<U> where F: FnOnce() -> U;
}

impl<T> MapNone for Option<T> {
    fn map_none<U, F>(&self, f: F) -> Option<U> where F: FnOnce() -> U {
        match self {
            Some(_) => None,
            None => Some(f())
        }
    }
}

fn map_true<T, F>(val: bool, f: F) -> Option<T> where F: FnOnce() -> T {
    if val { Some(f())} else { None }
}

fn as_strings(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

fn has_name(stock: &Stock) -> Option<Insanity> {
    stock.name.map_none(|| "missing name.".into())
}

fn has_brokerage_tag(stock: &Stock) -> Option<Insanity> {
    // TODO: Wow! This is inefficient.
    let tag_set = HashSet::from_iter(stock.tags.clone());

    map_true(tag_set.is_disjoint(&*brokerage_tags), || "has no brokerage tag".into())
}

fn has_short_tag_if_needed(stock: &Stock) -> Option<Insanity> {
    unimplemented!()
}

fn has_portfolio_tag(stock: &Stock) -> Option<Insanity> {
    unimplemented!()
}

fn has_lots(stock: &Stock) -> Option<Insanity> {
    unimplemented!()
}
