use connections::{connection_manager, udp};
use mt_handler::ThreadPool;
// use statistic::forecast;
use data_handler::udp::update::{listen_for_new_measurement, listen_for_node_measurement};
use rusqlite::{params, Connection};
use std::net::TcpListener;
use std::process::Command;
use std::sync::mpsc::channel;
use std::{env, thread};
//@todo: unclean
use data_handler::global::node;

const FIELDS: &[&str; 4] = &["temperature", "pressure", "humidity", "brightness"];

fn main() {
    let (udp_sender, udp_receiver) = channel();

    let args: Vec<String> = env::args().collect();
    let ip: String = args[1].clone();
    let srv_port: String = args[2].clone();
    let udp_port: String = args[3].clone();

    let server_adress = format!("{}:{}", ip, srv_port);
    let listener = TcpListener::bind(&server_adress).unwrap();
    let pool = ThreadPool::new(4);

    /********* PREPARE DATABASE *************/
    println!("Check for db...");
    if !std::path::Path::new("./data/measurements.db").exists() {
        println!("db does not exist, creating one...!");

        /********* FS PREPARATION *************/
        // mount ramfs
        Command::new("sh")
            .arg("-c")
            .arg("sudo mount -t ramfs ramfs ./data")
            .output()
            .expect("Failed");
        // owned by current user
        Command::new("sh")
            .arg("-c")
            .arg("sudo chown -R $USER:users ./data")
            .output()
            .expect("Failed");

        let conn = Connection::open("./data/measurements.db").unwrap();

        for field in FIELDS {
            let query: String = format!(
                "CREATE TABLE {} (time DATE, value NUMBER, origin NUMBER)",
                *field
            );
            let res = conn.execute(&query, params![]);
            match res {
                Ok(_) => {
                    println!("Succesfully inserted new table {}", field);
                }
                Err(err) => {
                    println!("Could not insert new table! {}", err)
                }
            }
        }
    } else {
        println!("Successful!");
    }
    node::node_info::init_map();
    
    /********* RUN THREADS *************/
    thread::spawn(|| {
        udp::start_udp_listener(ip, udp_port, udp_sender);
    });
    thread::spawn(|| {
        listen_for_node_measurement(udp_receiver);
        // listen_for_new_measurement(udp_receiver /* current_sender */);
    });

    /********* START SERVER *************/
    println!("Server listens on {}...", server_adress);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            connection_manager::handle_connection(stream);
        });
    }
}
