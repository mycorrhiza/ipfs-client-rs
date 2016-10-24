use std::borrow::Cow;

use tokio_core::reactor::Handle;
use tokio_curl::Session;
use fetch::Fetcher;

use version::VersionFuture;

pub struct Client {
    fetcher: Fetcher,
    base: Cow<'static, str>,
}

impl Client {
    pub fn new<S: Into<Cow<'static, str>>>(handle: Handle, base: S) -> Client {
        Client {
            fetcher: Fetcher::new(Session::new(handle)),
            base: base.into(),
        }
    }

    pub fn version(&self) -> VersionFuture {
        VersionFuture::new(&self.fetcher, &self.base)
    }
}
