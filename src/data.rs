#[derive(Debug, Deserialize)]
pub struct PeerInfo {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "PublicKey")]
    pub public_key: String,

    #[serde(rename = "Addresses")]
    pub addresses: Vec<String>,

    #[serde(rename = "AgentVersion")]
    pub agent_version: String,

    #[serde(rename = "ProtocolVersion")]
    pub protocol_version: String,
}

#[derive(Debug, Deserialize)]
pub struct Version {
    #[serde(rename = "Version")]
    pub version: String,

    #[serde(rename = "Commit")]
    pub commit: String,

    #[serde(rename = "Repo")]
    pub repo: String,
}

pub mod swarm {
    #[derive(Debug, Deserialize)]
    pub struct Peers {
        #[serde(rename = "Strings")]
        pub addresses: Vec<String>,
    }
}
