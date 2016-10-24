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

wrapped_future!(Vec<u8>, FetchFuture(Map<Join<Flatten<Done<MapErr<Perform,fn(PerformError) -> Error>,Error>>,Finished<Arc<Mutex<Vec<u8>>>, Error>>,fn((Easy, Arc<Mutex<Vec<u8>>>)) -> Vec<u8> >));
wrapped_future!(T, FetchJsonFuture<T: Deserialize>(AndThen<FetchFuture, Result<T>, fn(Vec<u8>) -> Result<T>>));

impl Fetcher {
    pub fn new(session: Session) -> Fetcher {
        Fetcher {
            session: session,
        }
    }

    pub fn fetch(&self, url: &str) -> FetchFuture {
        // We have to use an Arc<Mutex<_>> here because of limitations in the
        // tokio-curl API, hopefully in the future this may be lifted (or
        // there'll be an alternative library to replace tokio-curl).
        //
        // We actually only access the buffer one place at a time, first in the
        // `write_function` callback while the transfer is going on, then in
        // `finish_fetch` once the transfer has finished.
        //
        // The callback needs to use a Weak instead of a clone of the Arc as
        // the Easy instance keeps the callback even after the transfer has
        // completed, if the callback had a clone of the Arc then this would
        // block unwrapping it to take exclusive ownership of the buffer in
        // `finish_fetch`.
        //
        // (It should be possible to use an Arc here and simply drop the Easy
        // instance in `finish_fetch` before unwrapping it, for some reason
        // that doesn't work though, potential leak in curl-rs?)
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

        fn finish_fetch((_, buffer): (Easy, Arc<Mutex<Vec<u8>>>)) -> Vec<u8> {
            Arc::try_unwrap(buffer)
                .expect("We should be the only strong reference to the data")
                .into_inner()
                .expect("If the transferring thread panicked we should not have made it here in the first place")
        }

        let buffer = Arc::new(Mutex::new(Vec::with_capacity(128)));
        FetchFuture(
            futures::done(
                start_fetch(url, Arc::downgrade(&buffer))
                    .map(|req| self.session.perform(req).map_err(Error::from as _)))
            .flatten()
            .join(futures::finished(buffer))
            .map(finish_fetch))
    }
}

impl FetchFuture {
    pub fn parse_json<T: Deserialize>(self) -> FetchJsonFuture<T> {
        fn json_from_vec<T: Deserialize>(vec: Vec<u8>) -> Result<T> {
            serde_json::from_slice(&vec).map_err(Error::from)
        }

        FetchJsonFuture(self.and_then(json_from_vec))
    }
}
