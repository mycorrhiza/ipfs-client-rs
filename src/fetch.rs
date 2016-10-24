use std::sync::{ Arc, Weak, Mutex };

use curl::easy::Easy;
use futures::{ self, Future, AndThen, Done, Map, Join, Flatten, MapErr, Finished };
use serde::Deserialize;
use serde_json::{ self };
use tokio_curl::{ Perform, Session, PerformError };

use errors::*;

pub struct Fetcher {
    session: Session,
}

wrapped_future!(FetchFuture(Map<Join<Flatten<Done<MapErr<Perform,fn(PerformError) -> Error>,Error>>,Finished<Arc<Mutex<Vec<u8>>>, Error>>,fn((Easy, Arc<Mutex<Vec<u8>>>)) -> Vec<u8> >));

wrapped_future!(FetchJsonFuture<T: Deserialize>(AndThen<FetchFuture, Result<T>, fn(Vec<u8>) -> Result<T>>));

fn start_fetch(url: &str, buffer: Weak<Mutex<Vec<u8>>>) -> Result<Easy> {
    let mut req = Easy::new();
    try!(req.get(true));
    try!(req.url(url));
    try!(req.write_function(move |data| {
        let buffer = buffer.upgrade().expect("The Arc should be alive till the transfer is finished");
        let mut buffer = buffer.lock().expect("We're the only thread accessing this mutex now, so we shouldn't be able to poison it");
        buffer.extend_from_slice(data);
        Ok(data.len())
    }));
    Ok(req)
}

impl Fetcher {
    pub fn new(session: Session) -> Fetcher {
        Fetcher {
            session: session,
        }
    }

    pub fn fetch(&self, url: &str) -> FetchFuture {
        FetchFuture::new(&self.session, url)
    }
}

fn json_from_vec<T: Deserialize>(vec: Vec<u8>) -> Result<T> {
    serde_json::from_slice(&vec).map_err(Error::from)
}

fn finish_fetch((_, buffer): (Easy, Arc<Mutex<Vec<u8>>>)) -> Vec<u8> {
    Arc::try_unwrap(buffer)
        .expect("We should be the only strong reference to the data")
        .into_inner()
        .expect("If the transferring thread panicked we should not have made it here in the first place")
}

impl FetchFuture {
    fn new(session: &Session, url: &str) -> FetchFuture {
        let buffer = Arc::new(Mutex::new(Vec::with_capacity(128)));
        FetchFuture(
            futures::done(
                start_fetch(url, Arc::downgrade(&buffer))
                    .map(|req| session.perform(req).map_err(Error::from as _)))
            .flatten()
            .join(futures::finished(buffer))
            .map(finish_fetch))
    }

    pub fn parse_json<T: Deserialize>(self) -> FetchJsonFuture<T> {
        FetchJsonFuture(self.and_then(json_from_vec))
    }
}
