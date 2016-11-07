#![allow(redundant_closure)] // caused by error_chain
#![allow(trivial_casts)] // caused by error_chain

use curl;
use serde_json;
use tokio_curl;

error_chain! {
    foreign_links {
        curl::Error, Curl;
        serde_json::Error, SerdeJson;
        tokio_curl::PerformError, TokioCurl;
    }
}
