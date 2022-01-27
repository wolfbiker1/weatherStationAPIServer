pub mod update_path_handler {
    use ::inet::protocoll::http::HttpResponse;

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
