use std::net::UdpSocket;
use std::sync::mpsc::Sender;

pub fn start_udp_listener(ip: String, port: String, sender: Sender<u32>) -> ! {
    let udp_adress = format!("{}:{}", ip, port);
    println!("Udp socket listens on {}...", &udp_adress);
    let socket = UdpSocket::bind(udp_adress).unwrap();
    loop {
        let measure_data = receive_measure_data(&socket);
        update::update_path_handler::update(measure_data);
    }
}


pub fn receive_measure_data(socket: &UdpSocket) -> update::update_path_handler::Measurements {
    let mut buf = [0; 4];
    socket.recv_from(&mut buf).unwrap();
    let mut measure_data: Vec<u8> = vec![0; buf[0] as usize];
    socket.recv_from(&mut measure_data).unwrap();
    serde_json::from_str(std::str::from_utf8(&measure_data).unwrap()).unwrap()
}