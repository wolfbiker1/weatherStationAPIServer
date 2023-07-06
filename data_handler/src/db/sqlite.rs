use rusqlite::{params, Connection, Result};

pub mod database_module {
    use rusqlite::{params, Connection, Result};

    pub struct DatabaseInfo {
        database_instance: Result<rusqlite::Connection, rusqlite::Error>,
    }

    impl DatabaseInfo {
        pub fn new(db_path: &str) -> DatabaseInfo {
            DatabaseInfo {
                database_instance: Connection::open(db_path),
            }
        }
        pub fn db_init_table_default(&self, field: &str) {
            let query: String = format!(
                "CREATE TABLE {} (time DATE, value NUMBER, origin NUMBER)",
                field
            );
            match &self.database_instance {
                Ok(db) => {
                    let res = db.execute(&query, params![]);
                    match res {
                        Ok(_) => {}
                        Err(msg) => {
                            println!("Could not exec db_init_table_default - query : {}, err: {}", query, msg)
                        }
                    }
                }
                Err(_) => {
                    panic!("No Database available!");
                }
            }
        }
        pub fn db_insert_measurements(&self, table: &str, value: f64, origin: u8) {
            let query: String = format!(
                "insert into {} (time, value, origin) values (datetime('now','localtime'), {}, {})",
                table, value, origin
            );
            match &self.database_instance {
                Ok(db) => {
                    let res = db.execute(&query, params![]);
                    match res {
                        Ok(_) => {}
                        Err(msg) => {
                            println!("Could not insert value from {}, reason: '{}'", table, msg)
                        }
                    }
                }
                Err(_) => {
                    panic!("No Database available!");
                }
            }
        }
    }
}

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
