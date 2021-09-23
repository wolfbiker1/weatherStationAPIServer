pub mod connection_handler {
    use ::routes::{http, routes};
    use std::io::prelude::*;
    use std::net::TcpStream;
    use std::str;
    const CORS_HEADER: &str = "Access-Control-Allow-Origin: *";

    pub fn init_udp_connection(ip: String, port: String) {
        routes::route_handler::udp_listener(ip, port);
    }
    pub fn init_forecast_handler() {
        routes::route_handler::forecast_calculator();
    }
    pub fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let req = http::wrap_requests(&buffer);
        // println!("{} {}", req.get_type(), req.get_route());
        let response: http::HttpResponse =
            routes::route_handler::redirect_to_handler((&req.get_type(), &req.get_route()));

        let response = format!(
            "{}\r\n{}\r\n{}\r\nContent-Length: {}\r\n\r\n{}",
            response.status,
            CORS_HEADER,
            response.content_type,
            response.content.len(),
            response.content
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
