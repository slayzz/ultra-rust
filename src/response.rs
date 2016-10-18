
use std::cell::RefCell;
use std::rc::Rc;
use std::net::{TcpStream};
use std::io::{Write};

pub struct Response {
    protocol: String,
    all_ok_headers: String,
    stream: Rc<RefCell<TcpStream>>,
}

impl Response {
    pub fn new(the_stream: Rc<RefCell<TcpStream>>) -> Self {

        let std = vec![
            "Content-Type: text/html; charset=utf-8\n",
            "Connection: keep-alive\n",
        ];

        Response {
            protocol: "HTTP/1.1 {code} {codetxt}\n".to_owned(),
            all_ok_headers: std.iter().fold(String::new(), |ch, &s| ch + s),
            stream: the_stream
        }
    }

    pub fn write(&mut self, buffer: &str) {
        self.res_code_txt(200, "Ok");
        let pas_out = self.protocol.clone() + &self.all_ok_headers + "\n" + buffer;

        self.stream.borrow_mut().write_all(pas_out.as_bytes());
    }

    pub fn not_found(&mut self, buffer: &str) {
        self.res_code_txt(404, "Not Found");
        let mut out = (self.protocol.clone() + "\n") + buffer;
        self.stream.borrow_mut().write_all(out.as_bytes());
    }

    pub fn res_code(&mut self, code: u16) {
        self.protocol = self.protocol.replace("{code}", &code.to_string()).replace("{codetxt}","");
    }

    pub fn res_code_txt(&mut self, code: u16, txt: &str) {
        self.protocol = self.protocol.replace("{code}", &code.to_string()).replace("{codetxt}", txt);
    }


    // Если хотим отправлять chunked, пока неизвестно надо ли это
    // fn append_content(pas: &str, buf: &str) -> String {
    //     let mut pas_out = String::from(pas);
    //     pas_out.push_str(&format!("Content-Length: {}\r\n\r\n", buf.len()));
    //     pas_out.push_str(&format!("{}\r\n", [buf.len() as u8].to_hex()));
    //     pas_out.push_str(buf);
    //     pas_out.push_str(&format!("\r\n0\r\n\r\n"));
    //
    //     pas_out
    // }

}
