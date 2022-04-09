use std::str;
use std::time::{Duration, SystemTime};
pub struct HttpReq {
    req_type: String,
    route: String,
}

pub struct HttpResponse {
    pub status: String,
    pub content_type: String,
    pub content: String,
}

impl HttpReq {
    pub fn get_type(&self) -> String {
        self.req_type.clone()
    }
    pub fn get_route(&self) -> String {
        self.route.clone()
    }
}

pub fn wrap_requests(buffer: &[u8]) -> HttpReq {
    let req = str::from_utf8(buffer).unwrap();
    println!("Request: {}", req);
    println!("@ {:?}", SystemTime::now());
    let req = req.split(' ').collect::<Vec<&str>>();
    HttpReq {
        req_type: String::from(req[0]),
        route: String::from(req[1]),
    }
}
