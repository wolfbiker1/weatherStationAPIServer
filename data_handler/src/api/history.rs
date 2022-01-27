use ::inet::protocoll::http::HttpResponse;

pub mod history_path_handler {
    use super::HttpResponse;
    use rusqlite::Connection;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::str;

    #[derive(Debug, Serialize, Deserialize)]
    struct Point {
        x: String,
        y: f32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Dates {
        date: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct DatesCollection {
        temperature: serde_json::Value,
        pressure: serde_json::Value,
        humidity: serde_json::Value,
        brightness: serde_json::Value,
    }

    impl DatesCollection {
        pub fn new() -> DatesCollection {
            DatesCollection {
                temperature: json!({"foo": "bar"}),
                pressure: json!({"foo": "bar"}),
                humidity: json!({"foo": "bar"}),
                brightness: json!({"foo": "bar"}),
            }
        }
        pub fn change_value(mut self, field: &str, value: Vec<String>) -> DatesCollection {
            match field {
                "temp" => {
                    self.temperature = serde_json::Value::String(serde_json::to_string(&value).unwrap())
                }
                "pressure" => {
                    self.pressure =
                        serde_json::Value::String(serde_json::to_string(&value).unwrap())
                }
                "humidity" => {
                    self.humidity =
                        serde_json::Value::String(serde_json::to_string(&value).unwrap())
                }
                "brightness" => {
                    self.brightness =
                        serde_json::Value::String(serde_json::to_string(&value).unwrap())
                }
                _ => {}
            }
            self
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Peaks {
        max: f32,
        avg: f32,
        min: f32,
    }

    const FIELDS: &[&str; 4] = &["temperature", "pressure", "humidity", "brightness"];

    pub fn available_dates() -> HttpResponse {
        let conn = Connection::open("./data/measurements.db").unwrap_or_else(|error| {
            panic!("Could not open database, reason: '{}'", error);
        });

        let mut dt: DatesCollection = DatesCollection::new();
        for f in FIELDS {
            let mut result: Vec<String> = Vec::new();
            let query: String = format!("select distinct date(time) from {}", f);
            println!("{}", query);
            let mut stmt = conn.prepare(&query).unwrap();
            let date_iter = stmt.query_map([], |row| {
                let d = Dates {
                    date: row.get(0).unwrap(),
                };
                Ok(d)
            });
            for date in date_iter.unwrap() {
                let p = date.unwrap();
                result.push(p.date);
            }
            dt = dt.change_value(f, result);
        }
        HttpResponse {
            status: String::from("HTTP/2 200 OK"),
            content_type: String::from("Content-Type: 'text/plain'"),
            content: format!("{:?}", serde_json::to_string(&dt).unwrap()),
        }
    }

    ///
    /// 0 -> field (temp, ...)
    /// 1 -> type (min, max, ...)
    ///
    pub fn peaks() -> HttpResponse {
        let conn = Connection::open("./data/measurements.db").unwrap_or_else(|error| {
            panic!("Could not open database, reason: '{}'", error);
        });

        let mut result: Vec<String> = Vec::new();
        for field in FIELDS {
            let query: String = format!("select max(value), avg(value), min(value) from {}", field);
            let mut stmt = conn.prepare(&query).unwrap();
            let peak_iter = stmt.query_map([], |row| {
                let p = Peaks {
                    max: row.get(0).unwrap(),
                    avg: row.get(1).unwrap(),
                    min: row.get(2).unwrap(),
                };
                Ok(p)
            });
            for peak in peak_iter.unwrap() {
                let p = peak.unwrap();
                let peak_as_json = json!({
                    *field: {
                    "content": p
                    }
                });
                result.push(serde_json::to_string(&peak_as_json).unwrap());
            }
        }
        HttpResponse {
            status: String::from("HTTP/2 200 OK"),
            content_type: String::from("Content-Type: 'text/plain'"),
            content: format!("{:?}", result),
        }
    }

    // get all
    pub fn history_values(args: Vec<&str>) -> HttpResponse {
        let conn = Connection::open("./data/measurements.db").unwrap_or_else(|error| {
            panic!("Could not open database, reason: '{}'", error);
        });

        let query: String = format!("select * from {}", args[0]);
        let mut stmt = conn.prepare(&query).unwrap();
        let mut result: Vec<String> = Vec::new();
        let point_iter = stmt.query_map([], |row| {
            let p = Point {
                x: row.get(0).unwrap(),
                y: row.get(1).unwrap(),
            };
            Ok(p)
        });
        for point in point_iter.unwrap() {
            let p = point.unwrap();
            result.push(serde_json::to_string(&p).unwrap());
        }
        HttpResponse {
            status: String::from("HTTP/2 200 OK"),
            content_type: String::from("Content-Type: 'text/plain'"),
            content: format!("{:?}", result),
        }
    }

    // query: select time from pressure where time > '2021-09-19 17:14:57' and  time < '2021-09-19 17:17:57';
    pub fn history_range(args: Vec<&str>) -> HttpResponse {
        let conn = Connection::open("./data/measurements.db").unwrap_or_else(|error| {
            panic!("Could not open database, reason: '{}'", error);
        });

        let query: String = format!(
            "select * from {} where time > '{} {}' and time < '{} {}'",
            args[0], args[1], args[2], args[3], args[4]
        );
        println!("{}", query);
        let mut stmt = conn.prepare(&query).unwrap();
        let mut result: Vec<String> = Vec::new();
        let point_iter = stmt.query_map([], |row| {
            let p = Point {
                x: row.get(0).unwrap(),
                y: row.get(1).unwrap(),
            };
            Ok(p)
        });
        for point in point_iter.unwrap() {
            let p = point.unwrap();
            result.push(serde_json::to_string(&p).unwrap());
        }
        HttpResponse {
            status: String::from("HTTP/2 200 OK"),
            content_type: String::from("Content-Type: 'text/plain'"),
            content: format!("{:?}", result),
        }
    }
}
