use std::borrow::Cow;

use tokio_core::reactor::Handle;
use tokio_curl::Session;

use version::VersionFuture;

pub struct Client {
    session: Session,
    base: Cow<'static, str>,
}

impl Client {
    pub fn new<S: Into<Cow<'static, str>>>(handle: Handle, base: S) -> Client {
        Client {
            session: Session::new(handle),
            base: base.into(),
        }
    }

    pub fn version(&self) -> VersionFuture {
        VersionFuture::new(&self.session, &self.base)
    }
}
