use std::collections::HashMap;

use hyper::Uri;
use serde::{Deserialize, Deserializer};
use tokio::prelude::Future;

use crate::errors::*;

use super::Alphavantage;
use std::fmt::Display;
use std::str::FromStr;

// General ?

// TODO: verify that this is General
#[derive(Debug, Deserialize)]
struct Metadata {
    #[serde(rename = "2. Symbol")]
    symbol: String,
}

fn num_from_string<'de, D, T>(deserializer: D) -> std::result::Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: Display,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<T>().map_err(serde::de::Error::custom)
}

// TODO: verify that this is General
#[derive(Debug, Deserialize)]
struct Quote {
    #[serde(rename = "1. open", deserialize_with = "num_from_string")]
    open: f32,
    #[serde(rename = "2. high", deserialize_with = "num_from_string")]
    high: f32,
    #[serde(rename = "3. low", deserialize_with = "num_from_string")]
    low: f32,
    #[serde(rename = "4. close", deserialize_with = "num_from_string")]
    close: f32,
    #[serde(rename = "5. volume", deserialize_with = "num_from_string")]
    volume: u32,
}

// Intraday

struct IntraDay {
    symbol: String,
}

#[derive(Debug, Deserialize)]
pub struct IntraDayResponse {
    #[serde(rename = "Meta Data")]
    metadata: Metadata,
    #[serde(rename = "Time Series (60min)")]
    series: HashMap<String, Quote>,
}

impl IntraDayResponse {
    fn last_price(&self) -> Option<f32> {
        let last_date = self.series.keys().max().map(|s| s.clone());
        let quote = last_date.map(|d| &self.series[&d]);
        quote.map(|q| q.close)
    }
}

pub fn intraday(
    symbol: String,
    apikey: String,
) -> impl Future<Item = IntraDayResponse, Error = Error> {
    let client = Alphavantage::new(&apikey);
    client.query(IntraDay { symbol })
}

impl super::QuerySpec for IntraDay {
    type QueryResult = IntraDayResponse;

    fn url(&self, a: &Alphavantage) -> Uri {
        let mut url = a.base("TIME_SERIES_INTRADAY");
        url.query_pairs_mut()
            .append_pair("symbol", &self.symbol)
            .append_pair("interval", "60min")
            .append_pair("outputsize", "compact");
        super::convert_url(&url)
    }
}

// Daily

struct Daily {
    symbol: String,
}

#[derive(Debug, Deserialize)]
pub struct DailyResponse {
    #[serde(rename = "Meta Data")]
    metadata: Metadata,
    #[serde(rename = "Time Series (Daily)")]
    series: HashMap<String, Quote>,
}

impl DailyResponse {
    pub fn last_price(&self) -> Option<f32> {
        self.series
            .keys()
            .max()
            .map(|d| &self.series[d])
            .map(|q| q.close)
    }
}

pub fn daily(symbol: String, apikey: String) -> impl Future<Item = DailyResponse, Error = Error> {
    let client = Alphavantage::new(&apikey);
    client.query(Daily { symbol })
}

impl super::QuerySpec for Daily {
    type QueryResult = DailyResponse;

    fn url(&self, a: &Alphavantage) -> Uri {
        let mut url = a.base("TIME_SERIES_DAILY");
        url.query_pairs_mut()
            .append_pair("symbol", &self.symbol)
            .append_pair("outputsize", "compact");
        super::convert_url(&url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHORT_RESPONSE: &str = r#"{
    "Meta Data": {
        "1. Information": "Intraday (60min) open, high, low, close prices and volume",
        "2. Symbol": "GOOG",
        "3. Last Refreshed": "2019-03-27 15:30:00",
        "4. Interval": "60min",
        "5. Output Size": "Compact",
        "6. Time Zone": "US/Eastern"
    },
    "Time Series (60min)": {
        "2019-03-27 15:30:00": {
            "1. open": "1171.7600",
            "2. high": "1174.9399",
            "3. low": "1170.2000",
            "4. close": "1172.9800",
            "5. volume": "175329"
        },
        "2019-03-27 14:30:00": {
            "1. open": "1168.5950",
            "2. high": "1171.4900",
            "3. low": "1166.0179",
            "4. close": "1171.4900",
            "5. volume": "103325"
        },
        "2019-03-08 10:30:00": {
            "1. open": "1141.0699",
            "2. high": "1146.8500",
            "3. low": "1137.8900",
            "4. close": "1144.7000",
            "5. volume": "197772"
        },
        "2019-03-08 09:30:00": {
            "1. open": "1125.7950",
            "2. high": "1141.0900",
            "3. low": "1123.7700",
            "4. close": "1140.1200",
            "5. volume": "352188"
        },
        "2019-03-07 15:30:00": {
            "1. open": "1139.6488",
            "2. high": "1143.9399",
            "3. low": "1138.0601",
            "4. close": "1143.2200",
            "5. volume": "193325"
        },
        "2019-03-07 14:30:00": {
            "1. open": "1141.2100",
            "2. high": "1141.2488",
            "3. low": "1135.0000",
            "4. close": "1138.7400",
            "5. volume": "134785"
        }
    }
}    "#;

    #[test]
    fn test_deserialize() {
        let foo = serde_json::from_str::<IntraDayResponse>(SHORT_RESPONSE);
        assert!(foo.is_ok());

        let last_price = foo.unwrap().last_price().unwrap();
        assert_eq!(1172.9800, last_price);
    }
}
