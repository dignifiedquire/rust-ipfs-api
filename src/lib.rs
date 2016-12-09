#![cfg_attr(feature = "serde_derive", feature(proc_macro))]

#[cfg(feature = "serde_derive")]
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[cfg(feature = "serde_derive")]
include!("serde_types.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

extern crate hyper;
extern crate url;

use std::io::Read;
use hyper::client::response::Response;
use url::Url;

fn request(path: &str, args: &str) -> Response {
    let client = hyper::Client::new();
    let api_url = "http://localhost:5001/api/v0";

    let mut req_url = Url::parse(
        &format!("{}/{}", api_url, path)
    ).unwrap();

    req_url
        .query_pairs_mut()
        .append_pair("arg", args);

    let res = client
        .get(req_url)
        .send()
        .unwrap();

    assert_eq!(res.status, hyper::Ok);

    res
}

pub fn version() -> String {
    let mut res = request("version", "");
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    println!("{:?}", body);
    let parsed : Version = serde_json::from_str(body.trim()).unwrap();
    println!("{:?}", parsed);
    parsed.Version
}

pub fn cat(hash: &str) -> String {
    let mut res = request("cat", hash);
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    body
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(
            version(),
            "0.4.5-dev"
        )
    }

    #[test]
    fn test_cat() {
        assert_eq!(
            cat("/ipfs/QmVLDAhCY3X9P2uRudKAryuQFPM5zqA3Yij1dY8FpGbL7T/readme").lines().next().unwrap(),
            "Hello and Welcome to IPFS!"
        )
    }
}
