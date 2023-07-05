pub mod update_path_handler {
    use ::inet::protocoll::http::HttpResponse;

    pub fn insert(args: Vec<&str>) -> HttpResponse {
        for (index, arg) in args.iter().enumerate() {
            println!("arg no {} : {}", index, arg);
        }
        
        HttpResponse {
            status: String::from("HTTP/2 200 OK"),
            content_type: String::from("Content-Type: 'text/plain'"),
            content: format!("{:?}", "result"),
        }
    }

    pub fn register_node(args: Vec<&str>) -> HttpResponse {
        for (index, arg) in args.iter().enumerate() {
            // if node is already registered, return that information
            // else: register node

            println!("arg no {} : {}", index, arg);
        }
        
        HttpResponse {
            status: String::from("HTTP/2 200 OK"),
            content_type: String::from("Content-Type: 'text/plain'"),
            content: format!("{:?}", "result"),
        }
    }
}
