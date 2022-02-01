pub mod route_handler {
    use inet::protocoll::http::HttpResponse;
    use lang::types::*;

    use crate::api::{current, history, update};

    const GET_ROUTES_WITH_PARAM: [(RequestIdentifierWithParam, RequestHandlerWithParam); 4] = [
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
            (
                "GET",
                "/hist_range",
                "/:field/:date_start/:time_start/:date_end/:time_end",
            ),
            history::history_path_handler::history_range,
        ),
    ];

    const POST_ROUTES_WITH_PARAM: [(RequestIdentifier, RequestHandler); 1] =
        [(("POST", "/insert"), update::update_path_handler::insert)];
    const ROUTES: [(RequestIdentifier, RequestHandler); 9] = [
        (("GET", "/current"), current::get_all_current_fields),
        (("GET", "/temperature"), current::get_current_temp),
        (("GET", "/pressure"), current::get_current_pressure),
        (("GET", "/humidity"), current::get_current_humidty),
        (("GET", "/brightness"), current::get_current_brightness),
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
            if req.1.contains(raw_path) {
                let args = req.1.strip_prefix(raw_path).unwrap();
                let args: Vec<&str> = args.split('/').filter(|&x| !x.is_empty()).collect();
                return item.1();
            }
        }
        HttpResponse {
            status: String::from("HTTP/2 501 Not Implemented"),
            content_type: String::from("Content-Type: 'text/plain'"),
            content: String::from(""),
        }
    }
}
