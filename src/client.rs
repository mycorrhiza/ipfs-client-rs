use std::borrow::Cow;

use tokio_core::reactor::Handle;
use tokio_curl::Session;
use fetch::Fetcher;

use data::future;

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

    pub fn version(&self) -> future::Version {
        let url = self.base.to_owned() + "version";
        future::Version(self.fetcher.fetch(&url).parse_json())
    }
}
