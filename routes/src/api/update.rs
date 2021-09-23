pub mod update_path_handler {
    use async_std::task;
    use atomic_float::AtomicF64;
    use chrono::{DateTime, Utc};
    use rusqlite::{params, Connection, Result};
    use serde::{Deserialize, Serialize};
    use std::str;
    use std::sync::atomic::Ordering;
    use std::sync::RwLock;
    pub static OUTDOOR_TEMP: AtomicF64 = AtomicF64::new(0.0);
    pub static INDOOR_TEMP: AtomicF64 = AtomicF64::new(0.0);
    pub static PRESSURE: AtomicF64 = AtomicF64::new(0.0);
    pub static HUMIDITY: AtomicF64 = AtomicF64::new(0.0);
    pub static BRIGHTNESS: AtomicF64 = AtomicF64::new(0.0);
    use crate::http::HttpResponse;
    use chrono::{Duration, Local};
    use rand::Rng; // 0.8.0
    lazy_static! {
        pub static ref OUTDOOR_VALUES_STAMP: RwLock<String> = RwLock::new(String::from("n/a"));
    }

    const FIELDS: &[&str; 5] = &[
        "indoor_temp",
        "outdoor_temp",
        "pressure",
        "humidity",
        "brightness",
    ];

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Measurements {
        indoor_temp: String,
        outdoor_temp: String,
        humidity: String,
        pressure: String,
        brightness: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Timestamps {
        // indoor_temp: String,
        // brightness: Instant,
        // pressure: Instant,
        outdoor_values: String,
    }

    fn update_static_values(field: &str, value: f64) {
        match field {
            "outdoor_temp" => OUTDOOR_TEMP.store(value, Ordering::SeqCst),
            "indoor_temp" => INDOOR_TEMP.store(value, Ordering::SeqCst),
            "pressure" => PRESSURE.store(value, Ordering::SeqCst),
            "humidity" => HUMIDITY.store(value, Ordering::SeqCst),
            "brightness" => BRIGHTNESS.store(value, Ordering::SeqCst),
            _ => {
                println!("sry no match.. {}", field);
            }
        }
    }

    pub fn load_current_measurements(field: &str) -> f64 {
        match field {
            "outdoor_temp" => OUTDOOR_TEMP.load(Ordering::SeqCst),
            "indoor_temp" => INDOOR_TEMP.load(Ordering::SeqCst),
            "pressure" => PRESSURE.load(Ordering::SeqCst),
            "humidity" => HUMIDITY.load(Ordering::SeqCst),
            "brightness" => BRIGHTNESS.load(Ordering::SeqCst),
            _ => -1.0,
        }
    }

    pub fn load_current_timestamps(field: &str) -> String {
        match field {
            "outdoor_values" => OUTDOOR_VALUES_STAMP.write().unwrap().to_string(),
            _ => String::from("n/a"),
        }
    }

    pub fn update_timestamps(timestamps: Timestamps) {
        let mut stamp = OUTDOOR_VALUES_STAMP.write().unwrap();
        *stamp = timestamps.outdoor_values;
    }

    pub fn insert() -> HttpResponse {
        let conn = Connection::open("./database/measurements.db").unwrap_or_else(|error| {
            panic!("Could not open database, reason: '{}'", error);
        });
        // let mut rng = rand::task_rng();
        for i in 1..2880 {
            println!("{}", i);
            let n: u16 = rand::thread_rng().gen_range(1011..1021);
            let query: String = format!(
                "insert into pressure (time, value) values (datetime('now','localtime', '-{} minutes'), {})",
                i, n
            );
            let res = conn.execute(&query, params![]);

            let t: u16 = rand::thread_rng().gen_range(13..24);
            let query0: String = format!(
                "insert into outdoor_temp (time, value) values (datetime('now','localtime', '-{} minutes'), {})",
                i, t
            );
            let query: String = format!(
                "insert into indoor_temp (time, value) values (datetime('now','localtime', '-{} minutes'), {})",
                i, t
            );
            let res = conn.execute(&query0, params![]);
            let res = conn.execute(&query, params![]);

            let h: u16 = rand::thread_rng().gen_range(50..90);
            let query: String = format!(
                "insert into humidity (time, value) values (datetime('now','localtime', '-{} minutes'), {})",
                i, h
            );
            let res = conn.execute(&query, params![]);

            let b: u16 = rand::thread_rng().gen_range(11..11123);
            let query: String = format!(
                "insert into brightness (time, value) values (datetime('now','localtime', '-{} minutes'), {})",
                i, b
            );
            let res = conn.execute(&query, params![]);
            match res {
                Ok(_) => {}
                Err(msg) => {
                    println!(
                        "Could not insert value from {}, reason: '{}'",
                        n, msg
                    )
                }
            }
        }
        println!("bar!");
        HttpResponse {
            status: String::from("HTTP/2 200 OK"),
            content_type: String::from("Content-Type: 'text/plain'"),
            content: format!("{:?}", "result"),
        }
    }

    fn store_data_in_db(table_name: &str, value: f64) {
        let conn = Connection::open("./database/measurements.db").unwrap_or_else(|error| {
            panic!("Could not open database, reason: '{}'", error);
        });

        let query: String = format!(
            "insert into {} (time, value) values (datetime('now','localtime'), {})",
            table_name, value
        );
        let res = conn.execute(&query, params![]);
        match res {
            Ok(_) => {}
            Err(msg) => {
                println!(
                    "Could not insert value from {}, reason: '{}'",
                    table_name, msg
                )
            }
        }
    }

    pub fn update(measurements: Measurements) {
        for field in FIELDS {
            let value: f64 = match *field {
                "outdoor_temp" => measurements.outdoor_temp.parse::<f64>().unwrap(),
                "indoor_temp" => measurements.indoor_temp.parse::<f64>().unwrap(),
                "pressure" => measurements.pressure.parse::<f64>().unwrap(),
                "humidity" => measurements.humidity.parse::<f64>().unwrap(),
                "brightness" => measurements.brightness.parse::<f64>().unwrap(),
                _ => 0.0,
            };
            store_data_in_db(*field, value);
            update_static_values(*field, value);
        }
    }
}
