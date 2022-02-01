use super::trend::trend_handler;
use ::inet::protocoll::http::HttpResponse;

pub mod history_path_handler {
    use super::trend_handler;
    use super::HttpResponse;
    use chrono::Duration;
    use chrono::Local;
    use rusqlite::Connection;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::str;

    #[derive(Debug, Serialize, Deserialize)]
    struct QueryResult {
        date_of_record: String,
        value: f32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct QueryValueOnly {
        value: f64,
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
                "temperature" => {
                    self.temperature =
                        serde_json::Value::String(serde_json::to_string(&value).unwrap())
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

    ///
    /// Returns every available day where entries exists.
    ///
    /// # Arguments
    ///
    /// * `None`
    ///
    /// # Returns
    /// * `HttpResponse` - A HttpResponse object containing the result.
    ///
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
    /// Returns the trend the given field by analyzing the last 6 hours.
    ///
    /// # Arguments
    ///
    /// * `args` - A &str vector containing the following parameter: [0] - field,
    ///
    /// # Returns
    /// * `HttpResponse` - A HttpResponse object containing the result [m = gradient, b = intercept].
    ///
    pub fn trend_values(args: Vec<&str>) -> HttpResponse {
        let conn = Connection::open("./data/measurements.db").unwrap_or_else(|error| {
            panic!("Could not open database, reason: '{}'", error);
        });

        let now = Local::now();
        let six_hours_back = now - Duration::hours(6);

        let query: String = format!(
            "select value from {} where time < '{}' and time > '{}'",
            args[0], now, six_hours_back
        );

        let mut stmt = conn.prepare(&query).unwrap();
        let mut result: Vec<f64> = Vec::new();
        let res_iter = stmt.query_map([], |row| {
            let p = QueryValueOnly {
                value: row.get(0).unwrap(),
            };
            Ok(p)
        });

        for res in res_iter.unwrap() {
            let p = res.unwrap();
            result.push(p.value);
        }

        let trend_value = trend_handler::calc_trend(&result);

        HttpResponse {
            status: String::from("HTTP/2 200 OK"),
            content_type: String::from("Content-Type: 'text/plain'"),
            content: format!("{:?}", trend_value),
        }
    }

    ///
    /// Returns *min*, *max* and *avg* on the given field.
    ///
    /// # Arguments
    ///
    /// * `None`
    ///
    /// # Returns
    /// * `HttpResponse` - A HttpResponse object containing the result.
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

    ///
    /// Returns a bar chart for the last 12 hours with the values on average.
    ///
    /// # Arguments
    ///
    /// * `args` - A &str vector containing the following parameter: [0] - field,
    ///
    /// # Returns
    /// * `HttpResponse` - A HttpResponse object containing the result.
    ///
    pub fn barchart_values(args: Vec<&str>) -> HttpResponse {
        let conn = Connection::open("./data/measurements.db").unwrap_or_else(|error| {
            panic!("Could not open database, reason: '{}'", error);
        });

        let now = Local::now();
        let six_hours_back = now - Duration::hours(12);

        let query: String = format!("select strftime('%H:%M', time), avg(value) from {} where time < '{}' and time > '{}'  group by strftime ('%H:%M',time) order by strftime ('%H:%M',time)", args[0], now, six_hours_back);

        let mut stmt = conn.prepare(&query).unwrap();
        let mut result: Vec<String> = Vec::new();
        let res_iter = stmt.query_map([], |row| {
            let p = QueryResult {
                date_of_record: row.get(0).unwrap(),
                value: row.get(1).unwrap(),
            };
            Ok(p)
        });
        for res in res_iter.unwrap() {
            let p = res.unwrap();
            result.push(serde_json::to_string(&p).unwrap());
        }
        HttpResponse {
            status: String::from("HTTP/2 200 OK"),
            content_type: String::from("Content-Type: 'text/plain'"),
            content: format!("{:?}", result),
        }
    }

    ///
    /// Returns all values for the given field.
    ///
    /// # Arguments
    ///
    /// * `args` - A &str vector containing the following parameter: [0] - field,
    ///
    /// # Returns
    /// * `HttpResponse` - A HttpResponse object containing the result.
    ///
    pub fn history_values(args: Vec<&str>) -> HttpResponse {
        let conn = Connection::open("./data/measurements.db").unwrap_or_else(|error| {
            panic!("Could not open database, reason: '{}'", error);
        });

        let query: String = format!("select * from {}", args[0]);
        let mut stmt = conn.prepare(&query).unwrap();
        let mut result: Vec<String> = Vec::new();
        let res_iter = stmt.query_map([], |row| {
            let p = QueryResult {
                date_of_record: row.get(0).unwrap(),
                value: row.get(1).unwrap(),
            };
            Ok(p)
        });
        for res in res_iter.unwrap() {
            let p = res.unwrap();
            result.push(serde_json::to_string(&p).unwrap());
        }
        HttpResponse {
            status: String::from("HTTP/2 200 OK"),
            content_type: String::from("Content-Type: 'text/plain'"),
            content: format!("{:?}", result),
        }
    }

    ///
    /// Returns all values for the given field within the given time range.
    ///
    /// # Arguments
    ///
    /// * `args` - A &str vector containing the following parameters:
    ///            [0] - field, [1] - left bound date, [2] - left bound time, [3] - right bound date, [4] - right bound time.
    ///
    /// # Returns
    /// * `HttpResponse` - A HttpResponse object containing the result.
    ///
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
        let res_iter = stmt.query_map([], |row| {
            let p = QueryResult {
                date_of_record: row.get(0).unwrap(),
                value: row.get(1).unwrap(),
            };
            Ok(p)
        });
        for res in res_iter.unwrap() {
            let p = res.unwrap();
            result.push(serde_json::to_string(&p).unwrap());
        }
        HttpResponse {
            status: String::from("HTTP/2 200 OK"),
            content_type: String::from("Content-Type: 'text/plain'"),
            content: format!("{:?}", result),
        }
    }
}
