use std::collections::HashMap;
#[derive(Debug)]
pub struct Headers<'h_buf> {
    data: HashMap<&'h_buf str, &'h_buf str>
}

impl<'hb> Headers<'hb> {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).copied()
    }
}

impl<'buf> From<&'buf str> for Headers<'buf> {
    fn from(str_val: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in str_val.split("\r\n") {
            let mut k = sub_str;
            let mut v = "";

            if let Some(idx) = sub_str.find(":") {
                k = &sub_str[..idx];
                v = &sub_str[idx + 1..];
            }

            data.insert(k, v);
        }

        Self { data }
    }
}