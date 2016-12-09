use requests::request;
use std::io::Read;

pub fn cat(hash: &str) -> String {
    let mut res = request("cat", hash);
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    println!("{}", body);
    body
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cat() {
        assert_eq!(
            cat("/ipfs/QmVLDAhCY3X9P2uRudKAryuQFPM5zqA3Yij1dY8FpGbL7T/readme").lines().next().unwrap(),
            "Hello and Welcome to IPFS!"
        );
    }
}
