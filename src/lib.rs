#![cfg_attr(feature = "serde_macros", feature(plugin, custom_derive))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

extern crate serde;
extern crate serde_json;

#[cfg(feature = "serde_macros")]
include!("serde_types.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

extern crate hyper;

use std::io::Read;
use std::collections::BTreeMap;


pub fn version() -> String {
    let client = hyper::Client::new();
    let api_url = "http://localhost:5001/api/v0";

    let mut res = client
        .get(&format!("{}/version", api_url))
        .send()
        .unwrap();

    assert_eq!(res.status, hyper::Ok);
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    println!("{:?}", body);
    let parsed : Version = serde_json::from_str(body.trim()).unwrap();
    println!("{:?}", parsed);
    parsed.Version
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            version(),
            "0.4.4-dev"
        )
    }
}
