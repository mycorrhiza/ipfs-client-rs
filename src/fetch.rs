use std::sync::{ Arc, Mutex };

use curl::easy::Easy;
use futures::{ self, Future, AndThen, Done, Map, Join, Flatten, MapErr, Finished };
use serde::Deserialize;
use serde_json::{ self };
use tokio_curl::{ Perform, Session, PerformError };
use multiaddr::{ MultiAddr, Segment };

use errors::*;

pub struct Fetcher {
    session: Session,
}

wrapped_future!(Vec<u8>, FetchFuture(Map<Join<Flatten<Done<MapErr<Perform,fn(PerformError) -> Error>,Error>>,Finished<Arc<Mutex<Vec<u8>>>, Error>>,fn((Easy, Arc<Mutex<Vec<u8>>>)) -> Vec<u8> >));
wrapped_future!(T, FetchJsonFuture<T: Deserialize>(AndThen<FetchFuture, Result<T>, fn(Vec<u8>) -> Result<T>>));

pub struct Path(String);

impl From<String> for Path {
    fn from(s: String) -> Path {
        Path(s)
    }
}

impl<'a> From<&'a str> for Path {
    fn from(s: &str) -> Path {
        Path(s.to_owned())
    }
}

macro_rules! path_from {
    ($($a:ident : $t:ident),+) => {
        impl<$($t),+> From<($($t),+,)> for Path where $($t: AsRef<str>),+ {
            fn from(($($a),+,): ($($t),+,)) -> Path {
                Path([$($a.as_ref()),+].join("/"))
            }
        }
    }
}

path_from!(a: A, b: B, c: C);
path_from!(a: A, b: B, c: C, d: D);
path_from!(a: A, b: B, c: C, d: D, e: E);

pub struct Param(Option<String>);
pub struct Params(Option<String>);

impl Params {
    fn to_string(self) -> String {
        match self.0 {
            Some(mut s) => {
                s.insert(0, '?');
                s
            }
            None => "".to_owned()
        }
    }
}

impl<A> From<(A, bool)> for Param where A: Into<String> {
    fn from((a, b): (A, bool)) -> Param {
        Param(if b { Some(a.into()) } else { None })
    }
}

impl From<(&'static str, String)> for Param {
    fn from((a, b): (&str, String)) -> Param {
        Param(Some(format!("{}={}", a, b)))
    }
}

impl From<()> for Params {
    fn from(_: ()) -> Params {
        Params(None)
    }
}

impl<A> From<A> for Params where A: Into<Param> {
    fn from(a: A) -> Params {
        Params(a.into().0)
    }
}

impl<A> From<(A,)> for Params where A: Into<Param> {
    fn from((a,): (A,)) -> Params {
        Params(a.into().0)
    }
}

impl Fetcher {
    pub fn new(session: Session) -> Fetcher {
        Fetcher {
            session: session,
        }
    }

    pub fn fetch<P: Into<Path>, A: Into<Params>>(&self, host: &MultiAddr, path: P, params: A) -> FetchFuture {
        fn construct_url(host: &MultiAddr, path: Path, params: Params) -> Result<String> {
            let base = match host.segments() {
                &[Segment::IP4(ref addr), Segment::Tcp(port), Segment::Http] =>
                    format!("http://{}:{}/", addr, port),
                &[Segment::IP4(ref addr), Segment::Tcp(port), Segment::Https] =>
                    format!("http://{}:{}/", addr, port),
                &[Segment::IP6(ref addr), Segment::Tcp(port), Segment::Http] =>
                    format!("http://{}:{}/", addr, port),
                &[Segment::IP6(ref addr), Segment::Tcp(port), Segment::Https] =>
                    format!("http://{}:{}/", addr, port),
                _ => {
                    return Err(Error::from(format!("Cannot fetch from host {}", host)));
                }
            };
            Ok(base + &path.0 + &*params.to_string())
        }

        // We have to use an Arc<Mutex<_>> here because of limitations in the
        // tokio-curl API, hopefully in the future this may be lifted (or
        // there'll be an alternative library to replace tokio-curl).
        //
        // We actually only access the buffer one place at a time, first in the
        // `write_function` callback while the transfer is going on, then in
        // `finish_fetch` once the transfer has finished.
        fn start_fetch(host: &MultiAddr, path: Path, params: Params, buffer: Arc<Mutex<Vec<u8>>>) -> Result<Easy> {
            let mut req = Easy::new();
            try!(req.get(true));
            try!(req.url(&try!(construct_url(host, path, params))));
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
                start_fetch(host, path.into(), params.into(), buffer.clone())
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
