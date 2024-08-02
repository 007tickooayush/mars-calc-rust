use std::collections::HashMap;

/// Using A single Lifetime to manage both the keys and values of the HashMap
/// as the querystring only is utilized until the request buffer exists and then both are dropped at the same time
/// // SUGGEST: a complex value =>
/// a=1&b=2&c&d=&e===&d=7&d=abc
///
pub struct QueryString<'map_buff> {
    // OLD Implementation:
    // SYNTAX: data: HashMap<&'map_buff str, &'map_buff str>

    // NEW Implementation:
    data: HashMap<&'map_buff str, Value<'map_buff>>,
}

pub enum Value<'map_buff> {
    Single(&'map_buff str),
    Multiple(Vec<&'map_buff str>), // using heap allocated array (Vector) as the length is unknown at compile time
}

/// Implementation for the hashmap to store the Query String
impl<'map_buff> QueryString<'map_buff> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}