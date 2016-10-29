use multiaddr::MultiAddr;

use deserialize_helpers::vec_from_strs;

#[derive(Debug, Deserialize)]
pub struct PeerInfo {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "PublicKey")]
    pub public_key: String,

    #[serde(rename = "Addresses")]
    #[serde(deserialize_with = "vec_from_strs")]
    pub addresses: Vec<MultiAddr>,

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
    use std::collections::HashMap;

    use multiaddr::MultiAddr;

    use deserialize_helpers::{ vec_from_strs, map_of_vec_from_strs };

    #[derive(Debug, Deserialize)]
    pub struct Addresses {
        #[serde(rename = "Strings")]
        #[serde(deserialize_with = "vec_from_strs")]
        pub addresses: Vec<MultiAddr>,
    }

    #[derive(Debug, Deserialize)]
    pub struct PeerAddresses {
        #[serde(rename = "Addrs")]
        #[serde(deserialize_with = "map_of_vec_from_strs")]
        pub peers: HashMap<String, Vec<MultiAddr>>,
    }
}
