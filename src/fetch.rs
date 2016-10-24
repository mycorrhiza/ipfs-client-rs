use std::mem;
use std::sync::{ Arc, Weak, Mutex };
use std::marker::PhantomData;

use curl::easy::Easy;
use futures::{ Future, Async, Poll };
use serde::Deserialize;
use serde_json::{ self };
use tokio_curl::{ Perform, Session };

use errors::*;

enum State {
    Performing(Arc<Mutex<Vec<u8>>>, Perform),
    Errored(Error),
    Empty,
}

pub struct Fetcher {
    session: Session,
}

pub struct FetchFuture(State);
pub struct FetchJsonFuture<T: Deserialize>(FetchFuture, PhantomData<T>);

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

impl FetchFuture {
    fn new(session: &Session, url: &str) -> FetchFuture {
        let buffer = Arc::new(Mutex::new(Vec::with_capacity(128)));
        match start_fetch(url, Arc::downgrade(&buffer)) {
            Ok(req) => FetchFuture(State::Performing(buffer, session.perform(req))),
            Err(err) => FetchFuture(State::Errored(err)),
        }
    }

    pub fn parse_json<T: Deserialize>(self) -> FetchJsonFuture<T> {
        FetchJsonFuture(self, PhantomData)
    }
}

impl Future for FetchFuture {
    type Item = Vec<u8>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match mem::replace(&mut self.0, State::Empty) {
            State::Performing(buffer, mut perform) => {
                Ok(match try!(perform.poll()) {
                    Async::Ready(_) => {
                        let buffer = Arc::try_unwrap(buffer).expect("We should be the only strong reference to the data");
                        let buffer = buffer.into_inner().expect("If the transferring thread panicked we should not have made it here in the first place");
                        Async::Ready(buffer)
                    }
                    Async::NotReady => {
                        self.0 = State::Performing(buffer, perform);
                        Async::NotReady
                    }
                })
            },
            State::Errored(error) => Err(error),
            State::Empty => panic!("poll a FetchFuture after it's done"),
        }
    }
}

impl<T: Deserialize> Future for FetchJsonFuture<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.0.poll() {
            Ok(Async::Ready(buffer)) => {
                serde_json::from_slice(&*buffer).map_err(Error::from).map(Async::Ready)
            }
            Ok(Async::NotReady) => {
                Ok(Async::NotReady)
            }
            Err(error) => Err(error),
        }
    }
}
