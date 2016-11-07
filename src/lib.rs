#![feature(pub_restricted)]
#![feature(proc_macro)]
#![feature(slice_patterns)]

#![recursion_limit = "1024"]

#![allow(unknown_lints)] // for clippy
#![warn(fat_ptr_transmutes)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
// TODO #![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_import_braces)]
#![warn(unused_results)]
#![warn(variant_size_differences)]

extern crate curl;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate maddr;
extern crate mhash;
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
