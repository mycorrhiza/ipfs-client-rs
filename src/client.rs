use futures::Future;
use maddr::MultiAddr;
use mhash::MultiHash;
use tokio_core::reactor::Handle;
use tokio_curl::Session;

use fetch::Fetcher;
use data;
use future;

#[allow(missing_debug_implementations)] // TODO
pub struct Client {
    fetcher: Fetcher,
    host: MultiAddr,
}

#[allow(missing_debug_implementations)] // TODO
pub struct SwarmClient<'a>(&'a Client);

impl Client {
    pub fn new(handle: Handle, host: MultiAddr) -> Client {
        Client {
            fetcher: Fetcher::new(Session::new(handle)),
            host: host,
        }
    }

    pub fn version(&self) -> future::Version {
        self.fetcher.fetch(&self.host, ("api", "v0", "version"), ()).parse_json().into()
    }

    pub fn local_info(&self) -> future::PeerInfo {
        self.fetcher.fetch(&self.host, ("api", "v0", "id"), ()).parse_json().into()
    }

    pub fn peer_info(&self, peer: &MultiHash) -> future::PeerInfo {
        self.fetcher.fetch(&self.host, ("api", "v0", "id", peer.to_string()), ()).parse_json().into()
    }

    pub fn swarm(&self) -> SwarmClient {
        SwarmClient(self)
    }
}

impl<'a> SwarmClient<'a> {
    pub fn peers(&self) -> future::swarm::Addresses {
        self.0.fetcher.fetch(&self.0.host, ("api", "v0", "swarm", "peers"), ()).parse_json().into()
    }

    pub fn addresses(&self) -> future::swarm::PeerAddresses {
        self.0.fetcher.fetch(&self.0.host, ("api", "v0", "swarm", "addrs"), ()).parse_json().into()
    }

    pub fn local_addresses(&self, id: bool) -> future::swarm::Addresses {
        self.0.fetcher.fetch(&self.0.host, ("api", "v0", "swarm", "addrs", "local"), (("id", id))).parse_json().into()
    }

    pub fn connect(&self, addr: &MultiAddr) -> future::swarm::ConnectResult {
        let into: fn(data::swarm::ConnectResultData) -> Result<Vec<String>, String> = Into::into;
        self.0.fetcher
            .fetch(&self.0.host, ("api", "v0", "swarm", "connect"), ("arg", addr.to_string()))
            .parse_json()
            .map(into)
            .into()
    }

    pub fn disconnect(&self, addr: &MultiAddr) -> future::swarm::ConnectResult {
        let into: fn(data::swarm::ConnectResultData) -> Result<Vec<String>, String> = Into::into;
        self.0.fetcher
            .fetch(&self.0.host, ("api", "v0", "swarm", "disconnect"), ("arg", addr.to_string()))
            .parse_json()
            .map(into)
            .into()
    }
}
