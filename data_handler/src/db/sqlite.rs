use rusqlite::{params, Connection, Result};

pub mod database_module {
    use chrono::Duration;
    use chrono::Local;
    use rusqlite::{params, Connection, Result};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct QueryResult {
        date_of_record: String,
        value: f32,
    }

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
                            println!(
                                "Could not exec db_init_table_default - query : {}, err: {}",
                                query, msg
                            )
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
        // @todo: split content to db handling - data handling
        pub fn db_query_map(
            &self,
            table: &str,
            hours_back: chrono::DateTime<chrono::Local>,
            minute_offset: chrono::DateTime<chrono::Local>,
        ) -> Vec<String> {
        let query: String = format!(
            "select * from {} where time < '{}' and time > '{}'",
            table, hours_back, minute_offset
        );

        let mut stmt = self.database_instance.as_ref().unwrap().prepare(&query).unwrap();
        let res_iter = stmt.query_map([], |row| {
            let p = QueryResult {
                date_of_record: row.get(0).unwrap(),
                value: row.get(1).unwrap(),
            };
            Ok(p)
        });
        let mut result: Vec<String> = Vec::new();
        for res in res_iter.unwrap() {
            let p = res.unwrap();
            result.push(serde_json::to_string(&p).unwrap());
        }
        result
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

}