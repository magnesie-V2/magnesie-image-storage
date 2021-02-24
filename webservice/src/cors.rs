use rocket::fairing::{Fairing, Info, Kind};
use rocket::{http::Method, http::Status, Request, Response};

pub struct CorsFairing;

/// Cors fairing
impl Fairing for CorsFairing {
    fn on_response(&self, request: &Request, response: &mut Response) {
        // Add CORS headers to allow all origins to all outgoing requests
        response.set_header(rocket::http::Header::new(
            "Access-Control-Allow-Origin",
            "*",
        ));
        response.set_header(rocket::http::Header::new(
            "Access-Control-Allow-Methods", 
            "POST, GET, PUT, PATCH, OPTIONS"
        ));
        response.set_header(rocket::http::Header::new(
            "Access-Control-Allow-Headers", 
            "*"
        ));
        // Respond to all `OPTIONS` requests with a `200` (ok) status
        if response.status() == Status::NotFound && request.method() == Method::Options {
            response.set_status(Status::Ok);
        }
    }

    fn info(&self) -> Info {
        Info {
            name: "CORS Fairing",
            kind: Kind::Response,
        }
    }
}
