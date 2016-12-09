//! Library to interact with ipfs
#![feature(custom_derive, proc_macro)]
#![cfg_attr(feature = "dev", plugin(clippy))]
// This allows for better enforc
// unstable_features add one day when custom derive is stableed style and project features
#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts, unsafe_code, unused_import_braces,
        unused_qualifications)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate url;

mod version;
mod cat;
mod requests;
mod json;
