extern crate ultra;
use ultra::http_server::*;
use ultra::workspace::*;
// use std::collections::HashMap;

fn main() {


    let mut router = Router::new();
    router.add_route("/hello".to_string(), |_: &mut Request, res: &mut Response| {
        res.write("<div>Привет</div>");
    });
    router.add_route("/".to_string(), |_: &mut Request, res: &mut Response| {
        res.write("<div>Main</div>");
    });

    let mut server: HttpServer<Router> = HttpServer::new("localhost:1337", router);

    server.start();


}
