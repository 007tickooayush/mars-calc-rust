type BodyResult = String;

#[derive(Debug)]
/// Using a separate Struct for handling the request body to keep it as a separate entity <br/>
/// The reason is that the request Body can later on be used for REST/SOAP API calls
/// But the length of characters present in the Request body will be limited to 1024 characters
pub struct RequestBody {
    contents: BodyResult,
}

impl RequestBody {
    pub fn contents(&self) -> BodyResult {
        self.contents.clone()
    }
}

impl TryFrom<&str> for RequestBody {
    type Error = RequestBodyError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut res = String::new();;
        if value.len() < 1024 {
            for c in value.chars() {
                if c == '\r' {
                    continue;
                }
                if c == '\n' {
                    continue;
                }
                if c == '\t' {
                    continue;
                }
                if c == ' ' {
                    continue;
                }
                if c == '\0'
                {
                    continue;
                }
                if c == '\u{0}'
                {
                    continue;
                }
                res.push(c);
            }
        }

        todo!("\
        SUGGEST: Specify error for each scenario for Max allowed length exceeded, \
        Invalid encoding,\
        Invalid characters,\
        ");
        // match res {
        //     Some(r) => Ok(RequestBody { contents: r }),
        //     None => Err(RequestBodyError)
        // }

        unimplemented!()
    }
}

pub struct RequestBodyError;