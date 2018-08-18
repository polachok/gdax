#![feature(futures_api, async_await, await_macro)]
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;
extern crate serde;
extern crate serde_json;

use std::error::Error;
use hyper::{Client, Request, Body};
use hyper::rt::Stream;
use hyper_tls::HttpsConnector;
use futures::{FutureExt, TryFutureExt};
use futures::compat::{Future01CompatExt, TokioDefaultSpawn};

async fn get_currencies(url: &str) -> Result<serde_json::Value, Box<Error>> {
    let https = HttpsConnector::new(4).unwrap();
    let client = Client::builder()
            .build::<_, Body>(https);
    let req = Request::get(url)
                .header("User-Agent", "hyper/0.12").body(Body::empty()).unwrap();
    let resp = await!(client.request(req).compat())?;
    let body = await!(resp.into_body().concat2().compat())?;
    let res: serde_json::Value = serde_json::from_slice(&body)?;
    Ok(res)
}

async fn get_and_print(url: &str) {
    match await!(get_currencies(url)) {
        Ok(cur) => println!("{}", cur),
        Err(err) => println!("{:?}", err),
    }
}

fn main() {
    let future = get_and_print("https://api.gdax.com/currencies");
    let compat_future = future
                .boxed()
                .unit_error()
                .compat(TokioDefaultSpawn);
    tokio::run(compat_future);
}
