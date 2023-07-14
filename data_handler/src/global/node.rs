use super::super::db::sqlite::database_module;
use super::super::global::types;
use super::current;
pub mod node_info {
    use super::current::{append_to_file, create_file, read_file};
    use super::database_module::DatabaseInfo;
    use super::types::DatesCollection;
    use ::inet::protocoll::http::HttpResponse;
    use chrono::{DateTime, Utc};
    use lazy_static::lazy_static;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex, MutexGuard};

    use std::time::SystemTime;

    const FILE_NAME: &str = "./registered_nodes";
    lazy_static! {
        static ref REGISTERED_NODES: Mutex<Vec<NodeInfo>> = { Mutex::new(Vec::new()) };
    }

    pub struct CurrentValues {
        pub temperature: f64,
        pub humidity: f64,
    }

    // #[derive(Debug)]
    pub struct NodeInfo {
        node_number: u8,
        registered: bool,
        // location: String,
        // check_registration: FunctionCallback,
        database_instance: DatabaseInfo,
        pub current_values: CurrentValues,
        fields: Vec<&'static str>,
        last_update: Option<DateTime<Utc>>,
    }

    impl NodeInfo {
        // pub fn node_new(node_number: u8, node_location: String) -> NodeInfo {
        //     NodeInfo {
        //         node_number: node_number,
        //         registered: true,
        //         location: node_location,
        //         check_registration: is_registered,
        //         database_instance: None,
        //         last_update: None,
        //     }
        // }
        pub fn node_get_number(&self) -> u8 {
            self.node_number
        }

        pub fn node_is_registered(&self) -> bool {
            self.registered
        }

        pub fn update_timestamp(&mut self) {
            let timestamp_as_utc: DateTime<Utc> = SystemTime::now().into();

            self.last_update = Some(timestamp_as_utc);
        }

        pub fn get_fields(&self) -> Vec<&'static str> {
            self.fields.clone()
        }

        pub fn node_init_db(&self) {
            for field in &self.fields {
                self.database_instance.db_init_table_default(field);
            }
        }

        pub fn node_get_value_last24hours(
            &self,
            table: &str,
            hours_back: chrono::DateTime<chrono::Local>,
            minute_offset: chrono::DateTime<chrono::Local>,
        ) -> Vec<String> {
            self.database_instance
                .db_query_last24hours(table, hours_back, minute_offset)
        }

        pub fn node_get_value_history_range(
            &self,
            field: &str,
            left_bound_h: &str,
            left_bound_min: &str,
            right_bound_h: &str,
            right_bound_min: &str,
        ) -> Vec<String> {
            self.database_instance.db_query_history_range(
                field,
                left_bound_h,
                left_bound_min,
                right_bound_h,
                right_bound_min,
            )
        }

        pub fn node_get_value_peaks(&self) -> Vec<String> {
            self.database_instance.db_query_peaks(self.fields.as_ref())
        }

        pub fn node_get_available_dates(&self) -> DatesCollection {
            self.database_instance
                .db_query_available_dates(self.fields.as_ref())
        }

        pub fn node_insert_measurement(&self, table: &str, value: f64, node_number: u8) {
            self.database_instance
                .db_insert_measurements(table, value, node_number);
        }

        pub fn node_update_current(&mut self, field: &str, value: f64) {
            match field {
                "temperature" => self.current_values.temperature = value,
                "humidity" => self.current_values.humidity = value,
                _ => {}
            };
        }
    }
    lazy_static! {
        static ref NODE_MAPPING: HashMap<u8, (&'static str, Vec<&'static str>)> = {
            let mut mapping = HashMap::new();
            mapping.insert(0xA0, ("Bad", vec!["temperature", "humidity"]));
            mapping.insert(0xA1, ("Wohnzimmer", vec!["temperature", "humidity"]));
            mapping.insert(0xA2, ("Schlafzimmer", vec!["temperature", "humidity"]));
            mapping.insert(0xA3, ("Balkon", vec!["temperature", "humidity"]));
            mapping
        };
    }

    fn get_vector_access<'a>() -> MutexGuard<'a, Vec<NodeInfo>> {
        unsafe { REGISTERED_NODES.lock().unwrap() }
    }

    pub fn get_node_container<'a>(
        node_number: u8,
    ) -> Option<(NodeInfo, MutexGuard<'a, Vec<NodeInfo>>)> {
        let mut node_vector_guard = get_vector_access();
        let vec_protected = &mut *node_vector_guard;
        let index = vec_protected
            .iter()
            .position(|x| x.node_number == node_number);
        match index {
            Some(i) => {
                let node: Option<(NodeInfo, MutexGuard<'a, Vec<NodeInfo>>)> =
                    Some((vec_protected.remove(i), node_vector_guard));
                node
            }
            None => None,
        }
    }

    pub fn insert_node_container<'a>(
        node: NodeInfo,
        mut node_guard: MutexGuard<'a, Vec<NodeInfo>>,
    ) {
        let vec_protected = &mut *node_guard;
        vec_protected.push(node);
        drop(node_guard);
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
                }
            }
            Err(_) => {
                create_file(FILE_NAME);
            }
        }
    }

    pub fn is_registered(number: u8) -> bool {
        unsafe {
            let mut node_vector_guard = REGISTERED_NODES.lock().unwrap();
            let vec_protected = &mut *node_vector_guard;
            let node_already_registered = vec_protected
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
    pub fn register_node_pub(args: Vec<&str>) -> HttpResponse {
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

        // let node_location: String = match NODE_MAPPING.get(&number) {
        //     Some(res) => {
        //         let f = &*res;
        //         String::from(f.0)
        //     }
        //     None => String::from("Unknown location"),
        // };

        let mut db_path: String = "./data/".to_owned();
        let db_name: &str = NODE_MAPPING.get(&number).unwrap().0;
        let suffix: &str = ".db";

        db_path.push_str(db_name);
        db_path.push_str(suffix);

        let node_info: NodeInfo = NodeInfo {
            node_number: number,
            registered: true,
            // location: node_location,
            // check_registration: is_registered,
            fields: NODE_MAPPING.get(&number).unwrap().1.clone(),
            database_instance: DatabaseInfo::new(&db_path),
            current_values: CurrentValues {
                temperature: 0.0,
                humidity: 0.0,
            },
            last_update: None,
        };

        node_info.node_init_db();

        unsafe {
            let mut node_vector_guard = REGISTERED_NODES.lock().unwrap();
            let vec_protected = &mut *node_vector_guard;
            vec_protected.push(node_info);
            drop(vec_protected);
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
            assert_eq!(NODE_MAPPING.get(&0xA0).unwrap().0, "Bad");
            assert_eq!(NODE_MAPPING.get(&0xA3).unwrap().0, "Balkon");
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
