use std::mem;
use std::sync::{ Arc, Mutex };

use curl::easy::Easy;
use futures::{ Future, Async, Poll };
use serde_json::{ self, Map };
use tokio_curl::{ Perform, Session };

use errors::*;

#[derive(Debug)]
pub struct Version {
    pub version: String,
    pub commit: String,
    pub repo: String,
}

type Buffer = Arc<Mutex<Vec<u8>>>;

enum State {
    Errored(Error),
    Performing(Buffer, Perform),
    Empty,
}

pub struct VersionFuture(State);

fn version_request(base: &str, buffer: Buffer) -> Result<Easy> {
    let mut req = Easy::new();
    try!(req.get(true));
    try!(req.url(&(base.to_owned() + "version")));
    try!(req.write_function(move |data| {
        let mut buf = buffer.lock().expect("If some thread panicked we're SOL");
        buf.extend_from_slice(data);
        Ok(data.len())
    }));
    Ok(req)
}

fn parse_version(buffer: &[u8]) -> Result<Version> {
    let mut map: Map<String, String> = try!(serde_json::from_slice(buffer));
    Ok(Version {
        version: try!(map.remove("Version").ok_or("Missing version")),
        commit: try!(map.remove("Commit").ok_or("Missing commit")),
        repo: try!(map.remove("Repo").ok_or("Missing repo")),
    })
}

impl VersionFuture {
    pub(crate) fn new(session: &Session, base: &str) -> VersionFuture {
        let buffer = Arc::new(Mutex::new(Vec::with_capacity(128)));
        match version_request(base, buffer.clone()) {
            Ok(req) => VersionFuture(State::Performing(buffer, session.perform(req))),
            Err(err) => VersionFuture(State::Errored(err)),
        }
    }
}

impl Future for VersionFuture {
    type Item = Version;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match mem::replace(&mut self.0, State::Empty) {
            State::Performing(buffer, mut perform) => {
                match perform.poll() {
                    Ok(Async::Ready(_)) => {
                        let buf = buffer.lock().expect("If some thread panicked we're SOL");
                        Ok(Async::Ready(try!(parse_version(&*buf))))
                    }
                    Ok(Async::NotReady) => {
                        self.0 = State::Performing(buffer, perform);
                        Ok(Async::NotReady)
                    }
                    Err(error) => Err(error.into()),
                }
            },
            State::Errored(error) => Err(error),
            State::Empty => panic!("poll a VersionFuture after it's done"),
        }
    }
}
