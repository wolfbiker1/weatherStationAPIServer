use connections::{connection_manager, udp};
use data_handler::udp::update::listen_for_node_measurement;
use mt_handler::ThreadPool;
use std::net::TcpListener;
use std::process::Command;
use std::sync::mpsc::channel;
use std::{env, thread};
//@todo: unclean
use data_handler::global::node;
use std::fs;

fn main() {
    let (udp_sender, udp_receiver) = channel();

    let args: Vec<String> = env::args().collect();
    let ip: String = args[1].clone();
    let srv_port: String = args[2].clone();
    let udp_port: String = args[3].clone();

    let server_adress = format!("{}:{}", ip, srv_port);
    let listener = TcpListener::bind(&server_adress).unwrap();
    let pool = ThreadPool::new(4);

    /********* PREPARE RAMDISK *************/
    let df_out = Command::new("sh")
        .arg("-c")
        .arg("df ./data")
        .output()
        .unwrap();

    println!("Check for ramdisk...");
    if !(String::from_utf8_lossy(&df_out.stdout).contains("ramfs")
        || String::from_utf8_lossy(&df_out.stdout).contains("none"))
    {
        println!("ramdisk does not exist, creating one...!");

        /********* FS PREPARATION *************/
        if let Err(e) = fs::create_dir_all("./data") {
            eprintln!("Failed to create directory: {}", e);
            return;
        }

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
