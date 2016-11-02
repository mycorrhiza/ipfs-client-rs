#![feature(pub_restricted)]
#![feature(proc_macro)]
#![feature(slice_patterns)]

#![recursion_limit = "1024"]

extern crate curl;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate multiaddr;
extern crate multihash;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;
extern crate tokio_curl;

mod errors;

#[macro_use]
mod wrap;

mod deserialize_helpers;
mod fetch;

pub mod client;
pub mod data;
pub mod future;

pub use client::Client;
