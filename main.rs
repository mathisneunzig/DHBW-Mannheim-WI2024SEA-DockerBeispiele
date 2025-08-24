use tiny_http::{Server, Response};

fn main() {
    let server = Server::http("0.0.0.0:3000").unwrap();
    for request in server.incoming_requests() {
        if request.url() == "/hello" {
            let response = Response::from_string("Hello World");
            request.respond(response).unwrap();
        }
    }
}