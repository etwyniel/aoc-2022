use std::fmt::Debug;
use std::str::FromStr;

pub fn parse_tuple<L: FromStr, R: FromStr>(s: &str, pat: char) -> (L, R)
where
    L::Err: Debug,
    R::Err: Debug,
{
    let (l, r) = s.split_once(pat).unwrap();
    (l.parse().unwrap(), r.parse().unwrap())
}
