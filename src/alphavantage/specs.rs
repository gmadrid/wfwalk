use hyper::Uri;

use super::Alphavantage;

// Intraday

struct IntraDay {
    symbol: String,
}

pub fn intraday(symbol: String, apikey: String) {
    let client = Alphavantage::new(&apikey);
    let response = client.query(IntraDay { symbol });
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