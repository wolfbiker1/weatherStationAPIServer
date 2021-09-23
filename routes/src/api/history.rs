use super::trend::trend_handler::calc_trend;
pub mod history_path_handler {
    use super::calc_trend;
    use crate::http::HttpResponse;
    use async_std::task;
    use rusqlite::Connection;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use std::str;
    use serde_json::json;

    #[derive(Debug, Serialize, Deserialize)]
    struct Point {
        x: String,
        y: f32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Peaks {
        max: f32,
        avg: f32,
        min: f32,
    }

    const FIELDS: &[&str; 5] = &[
        "indoor_temp",
        "outdoor_temp",
        "pressure",
        "humidity",
        "brightness",
    ];

    ///
    /// 0 -> field (temp, ...)
    /// 1 -> type (min, max, ...)
    ///
    pub fn peaks() -> HttpResponse {
        let conn = Connection::open("./database/measurements.db").unwrap_or_else(|error| {
            panic!("Could not open database, reason: '{}'", error);
        });

        let mut result: Vec<String> = Vec::new();
        for field in FIELDS {

            let query: String = format!("select max(value), avg(value), min(value) from {}", field);
            let mut stmt = conn.prepare(&query).unwrap();
            let peak_iter = stmt.query_map([], |row| {
                // println!("{:?}", row.get(0));
                let p = Peaks {
                    max: row.get(0).unwrap(),
                    avg: row.get(1).unwrap(),
                    min: row.get(2).unwrap(),
                };
                Ok(p)
            });
            for peak in peak_iter.unwrap() {
                let p = peak.unwrap();
                let foo = json!({
                    *field: {
                    "content": p
                    }
                });
                result.push(serde_json::to_string(&foo).unwrap());
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
        let conn = Connection::open("./database/measurements.db").unwrap_or_else(|error| {
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
        let conn = Connection::open("./database/measurements.db").unwrap_or_else(|error| {
            panic!("Could not open database, reason: '{}'", error);
        });

        let query: String = format!("select * from {} where time > '{} {}' and time < '{} {}'", args[0], args[1], args[2], args[3], args[4]);
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
