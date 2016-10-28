use std::error::Error as StdError;
use std::str::FromStr;

use serde;
use serde::de::{ self, Deserializer };
use multiaddr::MultiAddr;

#[derive(Debug, Deserialize)]
pub struct PeerInfo {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "PublicKey")]
    pub public_key: String,

    #[serde(rename = "Addresses")]
    #[serde(deserialize_with = "from_strs")]
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
    use multiaddr::MultiAddr;
    use super::from_strs;

    #[derive(Debug, Deserialize)]
    pub struct Peers {
        #[serde(rename = "Strings")]
        #[serde(deserialize_with = "from_strs")]
        pub addresses: Vec<MultiAddr>,
    }
}

fn from_strs<T, D>(d: &mut D) -> Result<Vec<T>, D::Error>
    where T: FromStr,
          <T as FromStr>::Err: StdError,
          D: Deserializer
{
    let strs = try!(d.deserialize_seq(de::impls::VecVisitor::<String>::new()));
    let mut result = Vec::with_capacity(strs.len());
    for s in strs {
        result.push(try!(s.parse().map_err(|e| <D::Error as serde::Error>::custom(<<T as FromStr>::Err as StdError>::description(&e)))));
    }
    Ok(result)
}

// fn from_str<T, D>(d: &mut D) -> Result<T, D::Error>
//     where T: FromStr,
//           <T as FromStr>::Err: StdError,
//           D: Deserializer
// {
//     struct Visitor;
// 
//     impl de::Visitor for Visitor {
//         type Value = String;
// 
//         fn visit_str<E: de::Error>(&mut self, value: &str) -> Result<String, E> {
//             Ok(value.to_owned())
//         }
//     }
// 
//     Ok(try!(try!(d.deserialize_str(Visitor)).parse().map_err(|e| <D::Error as serde::Error>::custom(<<T as FromStr>::Err as StdError>::description(&e)))))
// }
