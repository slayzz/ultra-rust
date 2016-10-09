#[allow(dead_code)]
pub mod http_server {
    use request::*;
    use std::net::{TcpListener, TcpStream};
    use std::io::{Read, Write};


    pub trait Handler {
        fn handle(&mut self, &mut Request);
    }

    impl Handler for Box<Handler> {
        fn handle(&mut self, req: &mut Request) {
            (**self).handle(req)
        }
    }

    impl<F> Handler for F
    where F: Send + Sync + 'static + FnMut(&mut Request)  {
        fn handle(&mut self, req: &mut Request)  {
            (*self)(req)
        }
    }

    pub struct HttpServer<H> {
        handlers: H,
        addr: String,
        socket: Option<TcpListener>,
    }

    impl<H: Handler> HttpServer<H> {

        pub fn new(_addr: &str, _handler: H) -> Self {
            HttpServer {
                handlers: _handler,
                addr: _addr.to_owned(),
                socket: None
            }
        }

        pub fn start(&mut self) {
            let mut buffer = [0; 2048];
            self.socket = Some(TcpListener::bind(&self.addr as &str)
                .expect("Something happend wrong, when binding on addr"));
            let sock = self.socket.as_ref().unwrap();

            println!("Server running on address {}", &self.addr);

            for stream in sock.incoming() {
                let mut tcp_stream = stream.expect("Something wrong");
                if let Ok(_) = tcp_stream.read(&mut buffer) {
                    let mut req = Request::new(tcp_stream, & buffer);
                    self.handlers.handle(&mut req);
                }
            }
        }

    }
}

pub mod workspace {
    pub use request::Request;
    pub use router::Router;
}

pub mod request;
pub mod router;
