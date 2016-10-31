use base58::ToBase58;
use futures::Future;
use multiaddr::MultiAddr;
use multihash::MultiHash;
use tokio_core::reactor::Handle;
use tokio_curl::Session;

use fetch::Fetcher;
use future;

pub struct Client {
    fetcher: Fetcher,
    host: MultiAddr,
}

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
        self.fetcher.fetch(&self.host, ("api", "v0", "id", peer.to_bytes().to_base58()), ()).parse_json().into()
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
        self.0.fetcher
            .fetch(&self.0.host, ("api", "v0", "swarm", "connect"), ("arg", addr.to_string()))
            .parse_json()
            .map(Into::into as _)
            .into()
    }

    pub fn disconnect(&self, addr: &MultiAddr) -> future::swarm::ConnectResult {
        self.0.fetcher
            .fetch(&self.0.host, ("api", "v0", "swarm", "disconnect"), ("arg", addr.to_string()))
            .parse_json()
            .map(Into::into as _)
            .into()
    }
}
