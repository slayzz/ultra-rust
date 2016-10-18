
extern crate http_muncher;
extern crate rustc_serialize;
// extern crate chrono;

use std::collections::HashMap;
use std::net::{TcpStream};
use self::http_muncher::{Parser, ParserHandler};
// use self::rustc_serialize::hex::to_hex;
use std::cell::RefCell;
use std::rc::Rc;
use headers::Headers;


//Нужно для парсинга headers
pub struct HeaderHelper(HashMap<String, String>, String, String);

impl ParserHandler for HeaderHelper{

    fn on_url(&mut self, _: &mut Parser, url: &[u8]) -> bool {
        self.2 = String::from_utf8_lossy(url).to_string();
        true
    }

    fn on_header_field(&mut self, _: &mut Parser, header: &[u8]) -> bool {
       self.1 = String::from_utf8_lossy(header).to_string();
       true
    }
    fn on_header_value(&mut self, _: &mut Parser, value: &[u8]) -> bool {
        self.0.insert(self.1.clone(), String::from_utf8_lossy(value).to_string());
        true
   }
}

pub struct Request {
    pub headers: Headers,
    pub stream: Rc<RefCell<TcpStream>>,
    pub url: String,
    pub method: String,
}

impl Request {

    pub fn new(stream: Rc<RefCell<TcpStream>>, bytes: &[u8]) -> Self {
        let mut hdr_helper = HeaderHelper(HashMap::new(), String::new(), String::new());
        let mut prs = Parser::request();

        prs.parse(&mut hdr_helper, bytes);
        Request {
            headers: Headers::new(hdr_helper.0),
            stream: stream,
            url: hdr_helper.2 ,
            method: prs.http_method().to_owned(),
        }
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

    pub fn get_method(&self) -> &str {
        &self.method
    }


}
