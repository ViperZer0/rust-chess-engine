//! This is a private module that returns the differences between hashmaps.
//! This is useful to see how two piece mailboxes differ for testing.

use std::{collections::HashMap, hash::Hash};
use std::fmt::Debug;

pub fn compare_hashmaps<'a, 'b, K, V>(hashmap_1: &'a HashMap<K, V>, hashmap_2: &'b HashMap<K, V>) -> (Vec<(&'a K, &'a V)>, Vec<(&'b K, &'b V)>)
where K: Eq + Hash, V: PartialEq
{
    let hashmap_1_only = hashmap_1.iter().filter(|(key, value)| hashmap_2.get(key).map_or(true, |v| **value != *v));
    let hashmap_2_only = hashmap_2.iter().filter(|(key, value)| hashmap_1.get(key).map_or(true, |v| **value != *v));

    (hashmap_1_only.collect(), hashmap_2_only.collect())
}

pub fn print_hashmap_differences<K, V>(hashmap_1: &HashMap<K, V>, hashmap_2: &HashMap<K, V>)
where K: Eq + Hash + Debug, V: PartialEq + Debug
{
    let (hashmap_1_only, hashmap_2_only) = compare_hashmaps(hashmap_1, hashmap_2);
    println!("---Hashmap 1 Only---");
    for different in hashmap_1_only
    {
        println!("{:?}", different);
    }
    println!("---Hashmap 2 Only---");
    for different in hashmap_2_only
    {
        println!("{:?}", different);
    }
}
