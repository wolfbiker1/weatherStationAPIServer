use connections::connection_handler;
use mt_handler::ThreadPool;
// use statistic::forecast;
use std::net::TcpListener;
use std::{thread, env};
use rusqlite::{Connection, params};
const FIELDS: &[&str; 4] = &[
    "temp",
    "pressure",
    "humidity",
    "brightness",
];

fn main() {
    let args: Vec<String> = env::args().collect();
    let ip: String = args[1].clone();
    let srv_port: String = args[2].clone();
    let udp_port: String = args[3].clone();
    
    let server_adress = format!("{}:{}", ip, srv_port);
    let listener = TcpListener::bind(&server_adress).unwrap();
    let pool = ThreadPool::new(4);
    thread::spawn(|| {
        // connection_handler::init_forecast_handler();
    });

    thread::spawn(|| {
        connection_handler::init_udp_connection(ip, udp_port);
    });
    
    // check db
    println!("Check for db...");
    if !std::path::Path::new("./database/measurements.db").exists() {
        println!("db does not exist, creating one...!");
        let conn = Connection::open("./database/measurements.db").unwrap();

        for field in FIELDS {
            let query: String = format!("CREATE TABLE {} (time DATE, value NUMBER)", *field);
            let res = conn.execute(&query, params![]);
            match res {
                Ok(_) => {
                    println!("Succesfully inserted new table {}", field);
                },
                Err(err) => {
                    println!("Could not insert new table! {}", err)
                }
            }
        }
    } else {
        println!("Successful!");
    }
    
    println!("Server listens on {}...", server_adress);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            connection_handler::handle_connection(stream);
        });
    }
}
