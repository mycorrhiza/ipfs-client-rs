use std::collections::{ hash_map, HashMap };
use std::error::Error as StdError;
use std::str::FromStr;

use serde;
use serde::de::{ self, Deserializer };

pub fn map_of_vec_from_strs<T, D>(d: &mut D) -> Result<HashMap<String, Vec<T>>, D::Error>
    where T: FromStr,
          <T as FromStr>::Err: StdError,
          D: Deserializer
{
    let str_map = try!(d.deserialize_map(de::impls::HashMapVisitor::<String, Vec<String>, hash_map::RandomState>::new()));
    let mut results = HashMap::with_capacity(str_map.len());
    for (k, strs) in str_map {
        let mut result = Vec::with_capacity(strs.len());
        for s in strs {
            result.push(try!(s.parse().map_err(|e| <D::Error as serde::Error>::custom(<<T as FromStr>::Err as StdError>::description(&e)))));
        }
        results.insert(k, result);
    }
    Ok(results)
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
