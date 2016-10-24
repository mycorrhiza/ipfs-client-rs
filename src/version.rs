use futures::{ Future, Poll };
use fetch::{ Fetcher, FetchJsonFuture };

use errors::*;

#[derive(Debug, Deserialize)]
pub struct Version {
    #[serde(rename = "Version")]
    pub version: String,

    #[serde(rename = "Commit")]
    pub commit: String,

    #[serde(rename = "Repo")]
    pub repo: String,
}

pub struct VersionFuture(FetchJsonFuture<Version>);

impl VersionFuture {
    pub(crate) fn new(fetcher: &Fetcher, base: &str) -> VersionFuture {
        VersionFuture(fetcher.fetch(&(base.to_owned() + "version")).parse_json())
    }
}

impl Future for VersionFuture {
    type Item = Version;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll()
    }
}
