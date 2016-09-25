#![feature(pub_restricted)]

#![recursion_limit = "1024"]

extern crate curl;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate serde_json;
extern crate tokio_core;
extern crate tokio_curl;

mod errors;

pub mod client;
pub mod version;

pub use client::Client;
