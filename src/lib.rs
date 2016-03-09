#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate hyper;
extern crate serde;
extern crate serde_json;

use std::io::Read;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Version {
    Version: String,
    Repo: usize,
    Commit: String,
}

pub fn version() -> String {
    let client = hyper::Client::new();
    let api_url = "http://localhost:5001/api/v0";

    let mut res = client
        .get(&format!("{}/version", api_url))
        .header(hyper::header::Connection::close())
        .send()
        .unwrap();

    assert_eq!(res.status, hyper::Ok);
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    println!("{:?}", body);
    let parsed : Version = serde_json::from_str(body.trim()).unwrap();
    println!("{:?}", parsed);
    body
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            version(),
            "0.4.0-dev"
        )
    }
}
