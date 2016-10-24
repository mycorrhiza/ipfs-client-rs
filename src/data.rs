#[derive(Debug, Deserialize)]
pub struct Version {
    #[serde(rename = "Version")]
    pub version: String,

    #[serde(rename = "Commit")]
    pub commit: String,

    #[serde(rename = "Repo")]
    pub repo: String,
}

pub mod future {
    use fetch::FetchJsonFuture;

    macro_rules! future {
        ($t:ident) => {
            wrapped_future!(super::$t, $t(FetchJsonFuture<super::$t>));
        };
    }

    future!(Version);
}
