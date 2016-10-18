
#[allow(dead_code)]
pub mod http_server {
    use request::*;
    use response::*;
    use std::net::{TcpListener};
    use std::io::{Read};
    use std::cell::RefCell;
    use std::rc::Rc;


    pub trait Handler {
        fn handle(&mut self, &mut Request, &mut Response);
    }

    impl<F> Handler for F
    where F: Send + Sync + 'static + FnMut(&mut Request, &mut Response)  {
        fn handle(&mut self, req: &mut Request, res: &mut Response)  {
            (*self)(req, res)
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
                let rc_stream = Rc::new(RefCell::new(stream.expect("Something wrong with stream")));
                {
                    rc_stream.borrow_mut().read(&mut buffer);
                }
                self.handlers.handle(&mut Request::new(rc_stream.clone(), &buffer),
                    &mut Response::new(rc_stream.clone()));
            }
        }

    }
}

pub mod workspace {
    pub use request::Request;
    pub use router::Router;
    pub use response::Response;
}

pub mod request;
pub mod router;
pub mod headers;
pub mod response;
