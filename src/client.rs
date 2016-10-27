use tokio_core::reactor::Handle;
use tokio_curl::Session;
use fetch::Fetcher;
use multiaddr::MultiAddr;

use data::future;

pub struct Client {
    fetcher: Fetcher,
    host: MultiAddr,
}

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
}
