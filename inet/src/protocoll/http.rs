use chrono::{DateTime, Utc};
use std::str;
use std::time::SystemTime;
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
    // println!("-- custom logging start --");
    // let timestamp_as_utc: DateTime<Utc> = SystemTime::now().into();
    // println!(
    //     "Request:\n {} \n Time: {:?}",
    //     req,
    //     timestamp_as_utc.to_rfc3339()
    // );
    // println!("-- custom logging end --");

    let req = req.split(' ').collect::<Vec<&str>>();
    if req.len() >= 2 {
        return HttpReq {
            req_type: String::from(req[0]),
            route: String::from(req[1]),
        };
    } else {
        println!("FAIL: {:?}\n", req);
        return HttpReq {
            req_type: String::from(""),
            route: String::from(""),
        };
    }
}
