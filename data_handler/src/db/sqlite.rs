use super::super::global::types;
pub mod database_module {
    use super::types::{Dates, DatesCollection, Peaks, QueryResult};
    use rusqlite::{params, Connection, Result};
    use serde_json::json;

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
        // @todo: split content to db handling - data handling.
        // @todo: remove rendunant code stuff!!!!!
        pub fn db_query_last24hours(
            &self,
            table: &str,
            hours_back: chrono::DateTime<chrono::Local>,
            minute_offset: chrono::DateTime<chrono::Local>,
        ) -> Vec<String> {
            let query: String = format!(
                "select * from {} where time < '{}' and time > '{}'",
                table, hours_back, minute_offset
            );
            let mut result: Vec<String> = Vec::new();

            let db_ref = self.database_instance.as_ref();

            let prepared_stmt = match db_ref {
                Ok(self_ref) => Ok(self_ref.prepare(&query)),
                Err(e) => {
                    println!("{}", e);
                    Err(e)
                }
            };

            match prepared_stmt.unwrap() {
                Ok(mut stmt) => {
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
                }
                Err(e) => {
                    println!("{} , origin is 'db_query_last24hours`", e);
                }
            }

            result
        }

        pub fn db_query_history_range(
            &self,
            field: &str,
            left_bound_h: &str,
            left_bound_min: &str,
            right_bound_h: &str,
            right_bound_min: &str,
        ) -> Vec<String> {
            let query: String = format!(
                "select * from {} where time > '{} {}' and time < '{} {}'",
                field, left_bound_h, left_bound_min, right_bound_h, right_bound_min
            );
            let mut result: Vec<String> = Vec::new();

            let db_ref = self.database_instance.as_ref();

            let prepared_stmt = match db_ref {
                Ok(self_ref) => Ok(self_ref.prepare(&query)),
                Err(e) => {
                    println!("{}", e);
                    Err(e)
                }
            };

            match prepared_stmt.unwrap() {
                Ok(mut stmt) => {
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
                }
                Err(e) => {
                    println!("{}, origin is `db_query_history_range` ", e);
                }
            }

            result
        }

        pub fn db_query_available_dates(
            &self,
            available_fields: &Vec<&'static str>,
        ) -> DatesCollection {
            let mut dt: DatesCollection = DatesCollection::new();

            for f in available_fields {
                let mut result: Vec<String> = Vec::new();
                let query: String = format!("select distinct date(time) from {}", f);

                let db_ref = self.database_instance.as_ref();

                let prepared_stmt = match db_ref {
                    Ok(self_ref) => Ok(self_ref.prepare(&query)),
                    Err(e) => {
                        println!("{}", e);
                        Err(e)
                    }
                };

                match prepared_stmt.unwrap() {
                    Ok(mut stmt) => {
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
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }

            dt
        }

        pub fn db_query_peaks(&self, available_fields: &Vec<&'static str>) -> Vec<String> {
            let mut result: Vec<String> = Vec::new();
            let t: Vec<&str> = vec!["min", "avg", "max"];

            for field in available_fields {
                let query: String = format!("select *, max(value) from {} union select *, min(value) from {} union select *,avg(value) from {}  order by value", field, field, field);
                let db_ref = self.database_instance.as_ref();

                let prepared_stmt = match db_ref {
                    Ok(self_ref) => Ok(self_ref.prepare(&query)),
                    Err(e) => {
                        println!("{} , origin is 'db_query_peaks`", e);
                        Err(e)
                    }
                };

                match prepared_stmt.unwrap() {
                    Ok(mut stmt) => {
                        let peak_iter = stmt.query_map([], |row| {
                            let p = Peaks {
                                date: row.get(0).unwrap(),
                                val: row.get(2).unwrap(),
                            };
                            Ok(p)
                        });
                        for peak in peak_iter.unwrap().enumerate() {
                            let p = peak.1.unwrap();
                            let peak_as_json = json!({
                                *field: {
                                    "ident": t[peak.0],
                                "content": p
                                }
                            });
                            result.push(serde_json::to_string(&peak_as_json).unwrap());
                        }
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
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
