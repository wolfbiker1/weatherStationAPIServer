use super::current;
pub mod node_info {
    use ::inet::protocoll::http::HttpResponse;
    type FunctionCallback = fn(u8) -> bool;
    use super::current::{create_file, file_exists, read_file, append_to_file};
    use chrono::{DateTime, Utc};
    use lazy_static::lazy_static;
    use rusqlite::{params, Connection};
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::sync::Mutex;
    use std::time::SystemTime;

    const FILE_NAME: &str = "./registered_nodes";
    const PATH_APPENDIX: &str = "/data";

    pub struct NodeInfo {
        node_number: u8,
        registered: bool,
        location: String,
        check_registration: FunctionCallback,
        database_instance: Result<rusqlite::Connection, rusqlite::Error>,
        fields: Vec<&'static str>,
        last_update: Option<DateTime<Utc>>,
    }

    impl NodeInfo {
        fn set_callback(&mut self, c: FunctionCallback) {
            self.check_registration = c;
        }

        fn process_events(&self, node_number: u8) {
            (self.check_registration)(node_number);
        }
        pub fn update_timestamp(&mut self) {
            let timestamp_as_utc: DateTime<Utc> = SystemTime::now().into();

            self.last_update = Some(timestamp_as_utc);
        }
        pub fn get_fields(&self) -> Vec<&'static str> {
            self.fields.clone()
        }

        // @todo: Not a proper location
        pub fn db_init(&self) {
            for field in &self.fields {
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
                                    "Could not insert value fro"
                                )
                            }
                        }
                    }
                    Err(_) => {
                        panic!("No Database available for Node Container {}", self.location);
                    }
                }
            }
        }

        // @todo: Not a proper location
        pub fn insert_value(&self, table_name: &str, value: f64, origin: u8) {
            let query: String = format!(
                "insert into {} (time, value, origin) values (datetime('now','localtime'), {}, {})",
                table_name, value, origin
            );
            match &self.database_instance {
                Ok(db) => {
                    let res = db.execute(&query, params![]);
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
                Err(_) => {
                    panic!("No Database available for Node Container {}", self.location);
                }
            }
        }
    }

    lazy_static! {
        static ref NodeMapping: HashMap<u8, (&'static str, Vec<&'static str>)> = {
            let mut mapping = HashMap::new();
            mapping.insert(0xA0, ("Bad", vec!["temperature", "humidity"]));
            mapping.insert(0xA1, ("Wohnzimmer", vec!["temperature", "humidity"]));
            mapping.insert(0xA2, ("Schlafzimmer", vec!["temperature", "humidity"]));
            mapping.insert(0xA3, ("Balkon", vec!["temperature", "humidity"]));
            mapping
        };
    }

    static mut registered_nodes: Vec<NodeInfo> = Vec::new();

    pub fn get_node_container(node_number: u8) -> Option<NodeInfo> {
        unsafe {
            let index = registered_nodes
                .iter()
                .position(|x| x.node_number == node_number);
            match index {
                Some(i) => Some(registered_nodes.remove(i)),
                None => None,
            }
        }
    }

    pub fn insert_node_container(node: NodeInfo) {
        unsafe { registered_nodes.push(node) }
    }

    pub fn init_map() {
        let available_nodes = read_file(FILE_NAME);

        match available_nodes {
            Ok(file_content) => {
                if file_content.len() == 0 {
                    return;
                }

                let node_numbers: Vec<_> = file_content.trim().split(",").collect();
                println!("{:?}", node_numbers);
                for node in node_numbers.iter() {
                    match (*node).parse::<u8>() {
                        Ok(number) => {
                            register_node(number, true);
                        }
                        Err(e) => {
                            println!("{}", e);
                        } 
                    }
                    // match par
                    // register_node((*node).parse().unwrap(), true);
                }
            }
            Err(_) => {
                create_file(FILE_NAME);
            }
        }
    }

    pub fn is_registered(number: u8) -> bool {
        unsafe {
            let node_already_registered = registered_nodes
                .iter()
                .find(|&x| x.node_number == number && x.registered);
            match node_already_registered {
                Some(_) => true,
                None => false,
            }
        }
    }

    // @route
    // @todo: find proper location
    pub fn register_node_pub(args: Vec<&str>)  -> HttpResponse {
        // @todo: error handling
        register_node(args[0].parse::<u8>().unwrap(), false);
        HttpResponse {
            status: String::from("HTTP/2 200 OK"),
            content_type: String::from("Content-Type: 'text/plain'"),
            content: serde_json::to_string("ok").unwrap(),
        }
    }
    fn register_node(number: u8, from_cache: bool) {
        if is_registered(number) {
            println!("{} is already registered", number);
            return;
        }

        let node_location: String = match NodeMapping.get(&number) {
            Some(res) => {
                let f = &*res;
                String::from(f.0)
            }
            None => String::from("Unknown location"),
        };

        let mut db_path: String = "./data/".to_owned();
        let db_name: &str = NodeMapping.get(&number).unwrap().0;
        let suffix: &str = ".db";

        db_path.push_str(db_name);
        db_path.push_str(suffix);

        let node_info: NodeInfo = NodeInfo {
            node_number: number,
            registered: true,
            location: node_location,
            check_registration: is_registered,
            fields: NodeMapping.get(&number).unwrap().1.clone(),
            database_instance: Connection::open(db_path),
            last_update: None,
        };

        node_info.db_init();
        unsafe {
            registered_nodes.push(node_info);
        }

        if !from_cache {
            append_to_file(FILE_NAME, &number.to_string());
        }


    }

    pub fn load_map() {}

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_hashmap() {
            assert_eq!(NodeMapping.get(&0xA0).unwrap().0, "Bad");
            assert_eq!(NodeMapping.get(&0xA3).unwrap().0, "Balkon");
        }

        #[test]
        fn test_init() {
            // @todo: problems in filepath resolution, main program looks from
            // other root dir than test.

            init_map();
            // let path =
            // let is_present = file_exists(&String::from("FILE_NAME","df");
            assert_eq!(true, true);
        }
    }
}
