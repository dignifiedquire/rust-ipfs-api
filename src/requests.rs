
use hyper;
use hyper::client::response::Response;
use url::Url;

pub fn request(path: &str, args: &str) -> Response {
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
