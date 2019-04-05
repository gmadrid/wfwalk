use hyper::client::HttpConnector;
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use tokio::prelude::stream::Stream;
use tokio::prelude::Future;
use url::Url;

pub use specs::daily;
pub use specs::intraday;

use crate::errors::*;

mod specs;

type ClientType = Client<HttpsConnector<HttpConnector>>;

pub struct Alphavantage {
    client: ClientType,
    apikey: String,
}

impl Alphavantage {
    pub fn new(apikey: &str) -> Alphavantage {
        let https = HttpsConnector::new(4).expect("TLS initialization failed");
        let client = Client::builder().build::<_, hyper::Body>(https);

        Alphavantage {
            client,
            apikey: apikey.to_string(),
        }
    }

    fn base(&self, function: &str) -> Url {
        let mut url = Url::parse("https://www.alphavantage.co/").unwrap();
        url.set_path("/query");

        url.query_pairs_mut()
            .append_pair("function", function)
            .append_pair("datatype", "json")
            .append_pair("apikey", &self.apikey);
        url
    }

    fn query<S>(&self, spec: S) -> impl Future<Item = S::QueryResult, Error = Error>
    where
        S: QuerySpec,
    {
        let uri = spec.url(&self);
        self.client
            .get(uri)
            .map_err(Error::from)
            .and_then(|r| r.into_body().concat2().map_err(Error::from))
            .and_then(|chunk| serde_json::from_slice::<S::QueryResult>(&chunk).map_err(Error::from))
    }
}

fn convert_url(url: &Url) -> Uri {
    Uri::builder()
        .scheme(url.scheme())
        .authority(url.host_str().unwrap())
        .path_and_query(format!("{}?{}", url.path(), url.query().unwrap()).as_str())
        .build()
        .unwrap()
}

trait QuerySpec {
    type QueryResult: DeserializeOwned;

    fn url(&self, a: &Alphavantage) -> Uri;
}
