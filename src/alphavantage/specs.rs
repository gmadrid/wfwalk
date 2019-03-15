use hyper::Uri;

use super::Alphavantage;
use crate::errors::*;
use tokio::prelude::Future;

// Intraday

struct IntraDay {
    symbol: String,
}

pub fn intraday(symbol: String, apikey: String) -> impl Future<Item = (), Error = Error> {
    let client = Alphavantage::new(&apikey);
    client.query(IntraDay { symbol })
}

impl super::QuerySpec for IntraDay {
    fn url(&self, a: &Alphavantage) -> Uri {
        let mut url = a.base("TIME_SERIES_INTRADAY");
        url.query_pairs_mut()
            .append_pair("symbol", &self.symbol)
            .append_pair("interval", "60min")
            .append_pair("outputsize", "compact");
        super::convert_url(&url)
    }
}
