extern crate serde_json;

use requests::request;
use std::io::Read;
use json::Version;

pub fn version() -> String {
    let mut res = request("version", "");
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
    fn test_version() {
        assert_eq!(
            version(),
            "0.4.5-dev"
        );
    }
}
