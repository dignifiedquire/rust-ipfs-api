//! Structs related to JSON serialization/deserialization
#![allow(missing_docs)]
#[allow(non_snake_case)]

extern crate serde;
extern crate serde_json;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Version {
    pub Version: String,
    pub Repo: usize,
    pub Commit: String,
}
