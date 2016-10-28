use tokio_core::reactor::Handle;
use tokio_curl::Session;
use fetch::Fetcher;
use multiaddr::MultiAddr;

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
        self.fetcher.fetch(&self.host, ("api", "v0", "version")).parse_json().into()
    }

    pub fn host_info(&self) -> future::PeerInfo {
        self.fetcher.fetch(&self.host, ("api", "v0", "id")).parse_json().into()
    }

    pub fn peer_info<S: AsRef<str>>(&self, peer: S) -> future::PeerInfo {
        self.fetcher.fetch(&self.host, ("api", "v0", "id", peer)).parse_json().into()
    }

    pub fn swarm(&self) -> SwarmClient {
        SwarmClient(self)
    }
}

impl<'a> SwarmClient<'a> {
    pub fn peers(&self) -> future::swarm::Peers {
        self.0.fetcher.fetch(&self.0.host, ("api", "v0", "swarm", "peers")).parse_json().into()
    }
}
