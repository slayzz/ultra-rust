use std::collections::HashMap;

type HeaderMap = HashMap<String, String>;

pub struct Headers {
    headers: HeaderMap
}

impl Headers {

    pub fn new(the_headers: HeaderMap) -> Self {
        Headers { headers: the_headers }
    }


    pub fn get(&self, header_name: &str) -> Option<String> {
        self.headers.get(header_name).map(|st| st.clone())
    }

    pub fn get_as_hash(&self) -> &HeaderMap {
        (&self.headers)
    }
}
