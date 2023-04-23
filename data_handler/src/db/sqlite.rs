use rusqlite::{params, Connection, Result};

pub fn insert_in_db(table_name: &str, value: f64, origin: u8) {
    let conn = Connection::open("./data/measurements.db").unwrap_or_else(|error| {
        panic!("Could not open database, reason: '{}'", error);
    });

    let query: String = format!(
        "insert into {} (time, value, origin) values (datetime('now','localtime'), {}, {})",
        table_name, value, origin
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
