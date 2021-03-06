use connections::{connection_manager, udp};
use mt_handler::ThreadPool;
// use statistic::forecast;
use rusqlite::{params, Connection};
use std::net::TcpListener;
use std::process::Command;
use std::sync::mpsc::channel;
use std::{env, thread};

use data_handler::udp::update::listen_for_new_measurement;
// use data_handler::global::current::

const FIELDS: &[&str; 4] = &["temperature", "pressure", "humidity", "brightness"];

fn main() {
    let (udp_sender, udp_receiver) = channel();
    // let (current_sender, current_receiver) = channel();

    let args: Vec<String> = env::args().collect();
    let ip: String = args[1].clone();
    let srv_port: String = args[2].clone();
    let udp_port: String = args[3].clone();

    let server_adress = format!("{}:{}", ip, srv_port);
    let listener = TcpListener::bind(&server_adress).unwrap();
    let pool = ThreadPool::new(4);

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

    /********* PREPARE DATABASE *************/
    println!("Check for db...");
    if !std::path::Path::new("./data/measurements.db").exists() {
        println!("db does not exist, creating one...!");
        let conn = Connection::open("./data/measurements.db").unwrap();

        for field in FIELDS {
            let query: String = format!("CREATE TABLE {} (time DATE, value NUMBER)", *field);
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

    /********* RUN THREADS *************/
    thread::spawn(|| {
        udp::start_udp_listener(ip, udp_port, udp_sender);
    });
    thread::spawn(|| {
        listen_for_new_measurement(udp_receiver /* current_sender */);
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
