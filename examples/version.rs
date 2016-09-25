extern crate tokio_core;
extern crate ipfs_client;

use tokio_core::reactor::Core;
use ipfs_client::Client;

fn main() {
    let mut event_loop = Core::new().unwrap();
    let client = Client::new(event_loop.handle(), "http://localhost:5001/api/v0/");

    println!("{:?}", event_loop.run(client.version()));
}
