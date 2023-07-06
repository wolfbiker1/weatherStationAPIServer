pub mod route_handler {
    use inet::protocoll::http::HttpResponse;
    use lang::types::*;

    use crate::api::{current, history, update};
    use crate::global::node;

    const GET_ROUTES_WITH_PARAM: [(RequestIdentifierWithParam, RequestHandlerWithParam); 9] = [
        (
            ("GET", "/trend/for", "/:field"),
            history::history_path_handler::trend_values,
        ),
        (
            ("GET", "/hist/for", "/:field"),
            history::history_path_handler::history_values,
        ),
        (
            ("GET", "/barchart/for", "/:field"),
            history::history_path_handler::barchart_values,
        ),
        (
            ("GET", "/past/for", "/:field/:hours"),
            history::history_path_handler::get_past_value,
        ),
        // (
        //     ("GET", "/current", "/:location"),
        //     current::get_all_current_fields,
        // ),
        (
            (
                "GET",
                "/hist_range",
                "/:field/:date_start/:time_start/:date_end/:time_end",
            ),
            history::history_path_handler::history_range,
        ),
        (
            ("GET", "/current", "/:temperature/:location"),
            current::get_current_value,
        ),
        (
            ("GET", "/pressure", "/:location"),
            current::get_current_pressure,
        ),
        (
            ("GET", "/humidity", "/:location"),
            current::get_current_humidty,
        ),
        (
            ("GET", "/brightness", "/:location"),
            current::get_current_brightness,
        ),
    ];

    const POST_ROUTES_WITH_PARAM: [(RequestIdentifierWithParam, RequestHandlerWithParam); 1] = [(
        ("POST", "/registernode", "/:node_number"),
        node::node_info::register_node_pub,
    )];
    const ROUTES: [(RequestIdentifier, RequestHandler); 4] = [
        (("GET", "/timestamps"), current::get_timestamps),
        (("GET", "/peaks"), history::history_path_handler::peaks),
        (("GET", "/trend/current"), current::get_trends),
        (
            ("GET", "/available_dates"),
            history::history_path_handler::available_dates,
        ),
    ];

    pub fn redirect_to_handler(req: (&str, &str)) -> HttpResponse {
        for (_i, &item) in ROUTES.iter().enumerate() {
            if item.0 == req {
                return item.1();
            }
        }
        for (_i, &item) in GET_ROUTES_WITH_PARAM.iter().enumerate() {
            // 0 -> first tuple, 1 -> path
            let raw_path = item.0 .1;
            // buggy!
            if req.1.contains(raw_path) {
                let args = req.1.strip_prefix(raw_path).unwrap();
                let args: Vec<&str> = args.split('/').filter(|&x| !x.is_empty()).collect();
                return item.1(args);
            }
        }
        for (_i, &item) in POST_ROUTES_WITH_PARAM.iter().enumerate() {
            // 0 -> first tuple, 1 -> path
            let raw_path = item.0 .1;
            println!(
                "match raw: {} , req: {:?} , {}",
                raw_path,
                req,
                req.1.contains(raw_path)
            );
            if req.1.contains(raw_path) {
                let args = req.1.strip_prefix(raw_path).unwrap();
                let args: Vec<&str> = args.split('/').filter(|&x| !x.is_empty()).collect();
                return item.1(args);
            }
        }
        HttpResponse {
            status: String::from("HTTP/2 501 Not Implemented"),
            content_type: String::from("Content-Type: 'text/plain'"),
            content: String::from("Not Implemented!"),
        }
    }
}
