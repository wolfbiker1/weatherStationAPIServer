use std::net::UdpSocket;
use std::sync::mpsc::Sender;

pub fn start_udp_listener(ip: String, port: String, sender: Sender<Vec<u8>>) -> ! {
    let udp_adress = format!("{}:{}", ip, port);
    println!("Udp socket listens on {}...", &udp_adress);
    let socket = UdpSocket::bind(udp_adress).unwrap();
    loop {
        let mut buf = [0; 4];
        socket.recv_from(&mut buf).unwrap();

        // buf[0] : Num of upcoming bytes (quantity)
        let mut measure_data: Vec<u8> = vec![0; buf[0] as usize];
        socket.recv_from(&mut measure_data).unwrap();
        sender.send(measure_data);
    }
}