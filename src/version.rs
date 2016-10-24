use fetch::{ Fetcher, FetchJsonFuture };

#[derive(Debug, Deserialize)]
pub struct Version {
    #[serde(rename = "Version")]
    pub version: String,

    #[serde(rename = "Commit")]
    pub commit: String,

    #[serde(rename = "Repo")]
    pub repo: String,
}

wrapped_future!(VersionFuture(FetchJsonFuture<Version>));

impl VersionFuture {
    pub(crate) fn new(fetcher: &Fetcher, base: &str) -> VersionFuture {
        VersionFuture(fetcher.fetch(&(base.to_owned() + "version")).parse_json())
    }
}
