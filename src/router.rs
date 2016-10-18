use std::collections::HashMap;
use http_server::{Handler};
use ::workspace::Request;
use ::workspace::Response;

#[allow(dead_code)]
pub struct Router {
    routes: HashMap<String, Box<Handler>>
}

#[allow(dead_code)]
impl Router {
    pub fn new() -> Self {
        Router { routes: HashMap::new() }
    }
    pub fn add_route<H>(&mut self, path: String, handler: H) where H: Handler + 'static {
        self.routes.insert(path, Box::new(handler));
    }
}
impl Handler for Router {
    fn handle(&mut self, req: &mut Request, res: &mut Response) {
        match self.routes.get_mut(&req.url) {
            Some(handler) => handler.handle(req, res),
            None => res.not_found("<div><p>NOT FOUND</p></div>")
        }
    }
}
