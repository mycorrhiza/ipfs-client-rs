#![feature(pub_restricted)]
#![feature(proc_macro)]

#![recursion_limit = "1024"]

extern crate curl;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;
extern crate tokio_curl;

mod errors;

#[macro_use]
mod wrap;

mod fetch;

pub mod client;
pub mod version;

pub use client::Client;
