use std::sync::{ Arc, Mutex };

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
        fn start_fetch(url: &str, buffer: Arc<Mutex<Vec<u8>>>) -> Result<Easy> {
            let mut req = Easy::new();
            try!(req.get(true));
            try!(req.url(url));
            try!(req.write_function(move |data| {
                let mut buffer = buffer.lock().expect("We're the only thread accessing this mutex now, so we shouldn't be able to poison it");
                buffer.extend_from_slice(data);
                Ok(data.len())
            }));
            Ok(req)
        }

        fn finish_fetch((easy, buffer): (Easy, Arc<Mutex<Vec<u8>>>)) -> Vec<u8> {
            drop(easy);
            // Dropping the Easy instance here will drop the `write_function`
            // callback above containing the only other strong reference to the
            // buffer, allowing this try_unwrap to succeed.
            Arc::try_unwrap(buffer)
                .expect("We should be the only strong reference to the data")
                .into_inner()
                .expect("If the transferring thread panicked we should not have made it here in the first place")
        }

        let buffer = Arc::new(Mutex::new(Vec::with_capacity(128)));
        FetchFuture(
            futures::done(
                start_fetch(url, buffer.clone())
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
