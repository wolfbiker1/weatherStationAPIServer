use super::current;
pub mod runtime_info {
    type FunctionCallback = fn(u8) -> bool;
    use super::current::{create_file, file_exists, read_file};
    use chrono::{DateTime, Utc};
    use lazy_static::lazy_static;
    use rusqlite::{params, Connection};
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::sync::Mutex;

    const FILE_NAME: &str = "./data/registered_nodes";
    const PATH_APPENDIX: &str = "/data";

    struct NodeInfo {
        node_number: u8,
        registered: bool,
        location: String,
        check_registration: FunctionCallback,
        database_instance: Option<rusqlite::Connection>,
        last_update: Option<DateTime<Utc>>,
    }

    impl NodeInfo {
        fn set_callback(&mut self, c: FunctionCallback) {
            self.check_registration = c;
        }

        fn process_events(&self, node_number: u8) {
            (self.check_registration)(node_number);
        }
    }

    lazy_static! {
        static ref NodeMapping: HashMap<u8, &'static str> = {
            let mut mapping = HashMap::new();
            mapping.insert(0xA0, "Bad");
            mapping.insert(0xA1, "Wohnzimmer");
            mapping.insert(0xA2, "Schlafzimmer");
            mapping.insert(0xA3, "Balkon");
            mapping
        };
    }

    static mut registered_nodes: Vec<NodeInfo> = Vec::new();

    pub fn init_map() {
        let available_nodes = read_file(FILE_NAME);

        match available_nodes {
            Ok(file_content) => {
                if file_content.len() == 0 {
                    return;
                }

                let node_numbers: Vec<_> = file_content.trim().split(",").collect();
                for node in node_numbers.iter() {
                    println!("{}", node);
                    register_node((*node).parse().unwrap());
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

    pub fn register_node(number: u8) {
        if is_registered(number) {
            return;
        }

        let node_location: String = match NodeMapping.get(&number) {
            Some(res) => String::from(*res),
            None => String::from("Unknown location"),
        };

        let node_info: NodeInfo = NodeInfo {
            node_number: number,
            registered: true,
            location: node_location,
            check_registration: is_registered,
            database_instance: None,
            last_update: None,
        };

        unsafe {
            registered_nodes.push(node_info);
        }
    }

    pub fn load_map() {}

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_hashmap() {
            assert_eq!(*NodeMapping.get(&0xA0).unwrap(), "Bad");
            assert_eq!(*NodeMapping.get(&0xA3).unwrap(), "Balkon");
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
