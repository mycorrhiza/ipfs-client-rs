use std::hash::Hash;
use std::collections::{ hash_map, HashMap };
use std::error::Error as StdError;
use std::str::FromStr;

use serde;
use serde::de::{ self, Deserializer };

pub fn map_of_vec_from_strs<K, V, D>(d: &mut D) -> Result<HashMap<K, Vec<V>>, D::Error>
    where K: FromStr + Eq + Hash,
          V: FromStr,
          <K as FromStr>::Err: StdError,
          <V as FromStr>::Err: StdError,
          D: Deserializer
{
    let str_map = try!(d.deserialize_map(de::impls::HashMapVisitor::<String, Vec<String>, hash_map::RandomState>::new()));
    let mut map = HashMap::with_capacity(str_map.len());
    for (k, strs) in str_map {
        let key = try!(k.parse().map_err(|e| <D::Error as serde::Error>::custom(<<K as FromStr>::Err as StdError>::description(&e))));
        let mut values = Vec::with_capacity(strs.len());
        for v in strs {
            values.push(try!(v.parse().map_err(|e| <D::Error as serde::Error>::custom(<<V as FromStr>::Err as StdError>::description(&e)))));
        }
        if map.insert(key, values).is_some() {
            panic!("Can't have duplicate keys");
        }
    }
    Ok(map)
}

pub fn vec_from_strs<T, D>(d: &mut D) -> Result<Vec<T>, D::Error>
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

pub fn from_str<T, D>(d: &mut D) -> Result<T, D::Error>
    where T: FromStr,
          <T as FromStr>::Err: StdError,
          D: Deserializer
{
    struct Visitor;

    impl de::Visitor for Visitor {
        type Value = String;

        fn visit_str<E: de::Error>(&mut self, value: &str) -> Result<String, E> {
            Ok(value.to_owned())
        }
    }

    Ok(try!(try!(d.deserialize_str(Visitor)).parse().map_err(|e| <D::Error as serde::Error>::custom(<<T as FromStr>::Err as StdError>::description(&e)))))
}
