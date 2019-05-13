#![feature(async_await)]
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
use futures::compat::{Future01CompatExt};
use futures::channel::oneshot;
use futures::executor::block_on;

async fn get_currencies(url: &str) -> Result<serde_json::Value, Box<Error>> {
    let https = HttpsConnector::new(4).unwrap();
    let client = Client::builder()
            .build::<_, Body>(https);
    let req = Request::get(url)
                .header("User-Agent", "hyper/0.12").body(Body::empty()).unwrap();
    let resp = client.request(req).compat().await?;
    let body = resp.into_body().concat2().compat().await?;
    let res: serde_json::Value = serde_json::from_slice(&body)?;
    Ok(res)
}

async fn get_and_print(url: &str) {
    match get_currencies(url).await {
        Ok(cur) => println!("{}", cur),
        Err(err) => println!("{:?}", err),
    }
}

async fn get_and_send(url: &str, chan: oneshot::Sender<serde_json::Value>) {
    let res = get_currencies(url).await.unwrap();
    chan.send(res).unwrap();
}

fn main() {
    // do it async
    let future = get_and_print("https://api.gdax.com/currencies");
    let compat_future = future
                .boxed()
                .unit_error()
                .compat();
    tokio::run(compat_future);

    // do it sync
    let (tx, rx) = oneshot::channel();
    let future = get_and_send("https://api.gdax.com/currencies", tx);
    let compat_future = future
                .boxed()
                .unit_error()
                .compat();
    tokio::run(compat_future);
    let result = block_on(rx).unwrap();
    println!("{}", result);
}
