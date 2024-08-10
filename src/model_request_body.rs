#[derive(Debug)]
/// Using a separate Struct for handling the request body to keep it as a separate entity <br/>
/// The reason is that the request Body can later on be used for REST/SOAP API calls
pub struct RequestBody<'buf> {
    contents: &'buf str
}

impl<'buf> RequestBody<'buf> {
    pub fn contents(&self) -> &str {
        self.contents
    }
}

impl<'hb> From<&'hb str> for RequestBody<'hb> {
    fn from(value: &'hb str) -> Self {
        Self {
            contents: value.trim()
        }
    }
}