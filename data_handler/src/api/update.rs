pub mod update_path_handler {
    use atomic_float::AtomicF64;
    use rusqlite::{params, Connection, Result};
    use serde::{Deserialize, Serialize};
    use std::str;
    use std::sync::atomic::Ordering;

    use ::inet::protocoll::http::HttpResponse;

    fn update_static_values(field: &str, value: f64) {
        match field {
            // "temp" => OUTDOOR_TEMP.store(value, Ordering::SeqCst),
            // // "indoor_temp" => INDOOR_TEMP.store(value, Ordering::SeqCst),
            // "pressure" => PRESSURE.store(value, Ordering::SeqCst),
            // "humidity" => HUMIDITY.store(value, Ordering::SeqCst),
            // "brightness" => BRIGHTNESS.store(value, Ordering::SeqCst),
            _ => {
                println!("sry no match.. {}", field);
            }
        }
    }

    pub fn load_current_measurements(field: &str) -> f64 {
        match field {
            // "temp" => OUTDOOR_TEMP.load(Ordering::SeqCst),
            // // "indoor_temp" => INDOOR_TEMP.load(Ordering::SeqCst),
            // "pressure" => PRESSURE.load(Ordering::SeqCst),
            // "humidity" => HUMIDITY.load(Ordering::SeqCst),
            // "brightness" => BRIGHTNESS.load(Ordering::SeqCst),
            _ => -1.0,
        }
    }

    pub fn insert() -> HttpResponse {
    //     let conn = Connection::open("./database/measurements.db").unwrap_or_else(|error| {
    //         panic!("Could not open database, reason: '{}'", error);
    //     });
    //     // let mut rng = rand::task_rng();
    //     for i in 1..2880 {
    //         println!("{}", i);
    //         let n: u16 = rand::thread_rng().gen_range(1011..1021);
    //         let query: String = format!(
    //             "insert into pressure (time, value) values (datetime('now','localtime', '-{} minutes'), {})",
    //             i, n
    //         );
    //         let res = conn.execute(&query, params![]);

    //         let t: u16 = rand::thread_rng().gen_range(13..24);
    //         let query0: String = format!(
    //             "insert into temp (time, value) values (datetime('now','localtime', '-{} minutes'), {})",
    //             i, t
    //         );
    //         // let query: String = format!(
    //         //     "insert into indoor_temp (time, value) values (datetime('now','localtime', '-{} minutes'), {})",
    //         //     i, t
    //         // );
    //         let res = conn.execute(&query0, params![]);
    //         // let res = conn.execute(&query, params![]);

    //         let h: u16 = rand::thread_rng().gen_range(50..90);
    //         let query: String = format!(
    //             "insert into humidity (time, value) values (datetime('now','localtime', '-{} minutes'), {})",
    //             i, h
    //         );
    //         let res = conn.execute(&query, params![]);

    //         let b: u16 = rand::thread_rng().gen_range(11..11123);
    //         let query: String = format!(
    //             "insert into brightness (time, value) values (datetime('now','localtime', '-{} minutes'), {})",
    //             i, b
    //         );
    //         let res = conn.execute(&query, params![]);
    //         match res {
    //             Ok(_) => {}
    //             Err(msg) => {
    //                 println!("Could not insert value from {}, reason: '{}'", n, msg)
    //             }
    //         }
    //     }
    //     println!("bar!");
        HttpResponse {
            status: String::from("HTTP/2 200 OK"),
            content_type: String::from("Content-Type: 'text/plain'"),
            content: format!("{:?}", "result"),
        }
    }

}
