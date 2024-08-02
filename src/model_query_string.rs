use std::collections::HashMap;

#[derive(Debug)]
/// Using A single Lifetime to manage both the keys and values of the HashMap
/// as the querystring only is utilized until the request buffer exists and then both are dropped at the same time
///
/// SUGGEST: a complex value =>
/// a=1&b=2&c&d=&e===&d=7&d=abc
///
pub struct QueryString<'map_buff> {
    // OLD Implementation:
    // SYNTAX: data: HashMap<&'map_buff str, &'map_buff str>

    // NEW Implementation:
    data: HashMap<&'map_buff str, Value<'map_buff>>,
}

#[derive(Debug)]
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

/// SUGGEST: a complex value =>
/// a=1&b=2&c&d=&e===&d=7&d=abc
impl<'m_buf> From<&'m_buf str> for QueryString<'m_buf> {
    fn from(str_val: &'m_buf str) -> Self {
        let mut data = HashMap::new();

        // SUGGEST: !!{NOT_USED}!! using the for_each() function to iterate over the substrings
        // string.split("&").for_each(|substr| {
        //     let mut k = substr;
        //     let mut v = "";
        //     if let Some(idx) = substr.find("=") {
        //         k = &substr[..idx];
        //         v = &substr[idx+1..];
        //     }
        // });

        for sub_str in str_val.split("&") {
            let mut k = sub_str;
            let mut v = "";

            if let Some(idx) = sub_str.find("=") {
                k = &sub_str[..idx];
                v = &sub_str[idx + 1..];
            }

            data
                .entry(k)
                .and_modify(|existing: &mut Value| {
                    match existing {
                        Value::Single(prev_val) => {
                            // SUGGEST: using the Vec::with_capacity() function to pre-allocate the memory
                            // let mut vec = vec![prev_val, v];

                            // Dereferencing the value and replacing the value in the memory address with the new value
                            *existing = Value::Multiple(vec![prev_val, v]);
                        },
                        Value::Multiple(vec) => vec.push(v)
                    }
                })
                .or_insert(Value::Single(v));
        }

        QueryString { data }
    }
}