extern crate tokio_core;
extern crate ipfs_client;

use tokio_core::reactor::Core;
use ipfs_client::Client;

fn main() {
    let mut event_loop = Core::new().unwrap();
    let client = Client::new(event_loop.handle(), "/ip4/127.0.0.1/tcp/5001/https".parse().unwrap());

    println!("{:?}", event_loop.run(client.version()));
}
