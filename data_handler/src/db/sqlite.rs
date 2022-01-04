use rusqlite::{params, Connection, Result};

pub fn insert_values(table_name: &str, value: f64) {
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