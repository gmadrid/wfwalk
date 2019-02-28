use super::Stock;

type Insanity = String;

fn sanity_check(stock: Stock) -> Vec<Insanity> {
    let mut insanities = vec![];

    let tests = vec![
        has_name,
        has_brokerage_tag,
        has_short_tag_if_needed,
        has_portfolio_tag,
        has_lots,
    ];

    for test in tests {
        test(stock).map(|i| insanities.push(i));
    }

    insanities
}

//trait MapNone {
//    fn map_none<U, F>(&self, f: F) -> Option<U> where F: FnOnce() -> U;
//}
//
//impl<T> MapNone for Option<T> {
//    fn map_none<U, F>(&self, f: F) -> Option<U> where F: FnOnce() -> U {
//        match self {
//            Some(_) => None,
//            None => Some(f())
//        }
//    }
//}
//
fn has_name(stock: Stock) -> Option<Insanity> {
    unimplemented!()
}

fn has_brokerage_tag(stock: Stock) -> Option<Insanity> {
    unimplemented!()
}

fn has_short_tag_if_needed(stock: Stock) -> Option<Insanity> {
    unimplemented!()
}

fn has_portfolio_tag(stock: Stock) -> Option<Insanity> {
    unimplemented!()
}

fn has_lots(stock: Stock) -> Option<Insanity> {
    unimplemented!()
}
