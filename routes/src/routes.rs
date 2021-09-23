pub mod route_handler {
    type RequestIdentifier = (&'static str, &'static str);
    type RequestHandler = fn() -> HttpResponse;
    type RequestIdentifierWithParam = (&'static str, &'static str, &'static str);
    type RequestHandlerWithParam = fn(Vec<&str>) -> HttpResponse;
    
    use rusqlite::{Connection};
    use crate::http::HttpResponse;
    use std::net::UdpSocket;

    // JUST A WORKAROUND
    // #[path = "/home/pi/rustserver/routes/src/api/current.rs"]
    // pub mod current;
    // #[path = "/home/pi/rustserver/routes/src/api/history.rs"]
    // pub mod history;
    // #[path = "/home/pi/rustserver/routes/src/api/forecast.rs"]
    // pub mod forecast;
    // #[path = "/home/pi/rustserver/routes/src/api/update.rs"]
    // pub mod update;
    // #[path = "/home/pi/rustserver/routes/src/api/trend.rs"]
    // pub mod trend;
    #[path = "/home/benni/development/backend/weatherStationAPIServer/routes/src/api/current.rs"]
    pub mod current;
    #[path = "/home/benni/development/backend/weatherStationAPIServer/routes/src/api/forecast.rs"]
    pub mod forecast;
    #[path = "/home/benni/development/backend/weatherStationAPIServer/routes/src/api/history.rs"]
    pub mod history;
    #[path = "/home/benni/development/backend/weatherStationAPIServer/routes/src/api/trend.rs"]
    pub mod trend;
    #[path = "/home/benni/development/backend/weatherStationAPIServer/routes/src/api/update.rs"]
    pub mod update;
    const GET_ROUTES_WITH_PARAM: [(RequestIdentifierWithParam, RequestHandlerWithParam); 2] = [
        // (
        //     ("GET", "/history", "/:field/:type"),
        //     history::history_path_handler::peaks,
        // ),
        (
            ("GET", "/hist/for", "/:field"),
            history::history_path_handler::history_values,
        ),
        (
            // left: start value
            ("GET", "/hist_range", "/:field/:date_start/:time_start/:date_end/:time_end"),
            history::history_path_handler::history_range,
        ),
    ];

    const POST_ROUTES_WITH_PARAM: [(RequestIdentifier, RequestHandler); 1] = [(
        ("POST", "/insert"),
        update::update_path_handler::insert,
    )];
    const ROUTES: [(RequestIdentifier, RequestHandler); 9] = [
        (("GET", "/current"), current::public_api),
        (("GET", "/outdoor_temp"), current::get_current_temp),
        (("GET", "/pressure"), current::get_current_pressure),
        (("GET", "/humidity"), current::get_current_humidty),
        (("GET", "/brightness"), current::get_current_brightness),
        (("GET", "/timestamps"), current::get_timestamps),
        (("GET", "/peaks"), history::history_path_handler::peaks),
        (("GET", "/trend/current"), current::get_trends),
        // (("GET", "/foo"), history::history_path_handler::history_values),
        (
            ("GET", "/forecast"),
            forecast::forecast_handler::calc_forecast,
        ),
    ];

    ///
    ///
    /// ```
    /// let x = 2
    /// ```
    /// foobarbaz
    ///
    ///
    pub fn receive_measure_data(socket: &UdpSocket) -> update::update_path_handler::Measurements {
        let mut buf = [0; 4];
        socket.recv_from(&mut buf).unwrap();
        let mut measure_data: Vec<u8> = vec![0; buf[0] as usize];
        socket.recv_from(&mut measure_data).unwrap();
        serde_json::from_str(std::str::from_utf8(&measure_data).unwrap()).unwrap()
        // update::update_path_handler::update(measure_struct, &DB_ADRESS);
    }

    pub fn udp_listener(ip: String, port: String) -> ! {
        let udp_adress = format!("{}:{}", ip, port);
        println!("Udp socket listens on {}...", &udp_adress);
        let socket = UdpSocket::bind(udp_adress).unwrap();
        loop {
            let measure_data = receive_measure_data(&socket);
            update::update_path_handler::update(measure_data);
            // timestamps
            let mut buf = [0; 4];
            socket.recv_from(&mut buf).unwrap();
            let mut timestamp_data: Vec<u8> = vec![0; buf[0] as usize];
            socket.recv_from(&mut timestamp_data).unwrap();
            let timestamp_struct: update::update_path_handler::Timestamps =
                serde_json::from_str(std::str::from_utf8(&timestamp_data).unwrap()).unwrap();
            update::update_path_handler::update_timestamps(timestamp_struct);
        }
    }

    pub fn forecast_calculator() {
        forecast::forecast_handler::main_worker()
    }

    pub fn redirect_to_handler(req: (&str, &str)) -> HttpResponse {
        for (_i, &item) in ROUTES.iter().enumerate() {
            // println!("{}", format!("{:?} {:?}", item.0, req));
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
                // println!("args: {:?}", args);
                return item.1(args);
            }
        }
        for (_i, &item) in POST_ROUTES_WITH_PARAM.iter().enumerate() {
            // 0 -> first tuple, 1 -> path
            let raw_path = item.0 .1;
            if req.1.contains(raw_path) {
                let args = req.1.strip_prefix(raw_path).unwrap();
                let args: Vec<&str> = args.split('/').filter(|&x| !x.is_empty()).collect();
                // println!("args: {:?}", args);
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
