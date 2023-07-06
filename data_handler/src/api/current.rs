use super::super::global::current::read_static_value;
use super::super::global::node::node_info::{get_node_container, insert_node_container, NodeInfo};
use ::inet::protocoll::http::HttpResponse;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    time: DateTime<Local>,
    value: f64,
}

pub fn get_trends() -> HttpResponse {
    let mut data: Vec<f64> = Vec::new();

    HttpResponse {
        status: String::from("HTTP/2 200 OK"),
        content_type: String::from("Content-Type: 'text/plain'"),
        content: format!("{:?}", data),
    }
}

pub fn get_timestamps() -> HttpResponse {
    let mut data: Vec<String> = Vec::new();
    HttpResponse {
        status: String::from("HTTP/2 200 OK"),
        content_type: String::from("Content-Type: 'text/plain'"),
        content: format!("{:?}", data),
    }
}

pub fn get_current_temp(args: Vec<&str>) -> HttpResponse {
    let data = Data {
        time: Local::now(),
        value: fetch_value("temperature", args[0]),
    };

    HttpResponse {
        status: String::from("HTTP/2 200 OK"),
        content_type: String::from("Content-Type: 'text/plain'"),
        content: serde_json::to_string(&data).unwrap(),
    }
}

pub fn get_current_pressure(args: Vec<&str>) -> HttpResponse {
    let data = Data {
        time: Local::now(),
        value: fetch_value("pressure", args[0]),
    };

    HttpResponse {
        status: String::from("HTTP/2 200 OK"),
        content_type: String::from("Content-Type: 'text/plain'"),
        content: serde_json::to_string(&data).unwrap(),
    }
}

pub fn get_current_humidty(args: Vec<&str>) -> HttpResponse {
    let data = Data {
        time: Local::now(),
        value: fetch_value("humidity", args[0]),
    };

    HttpResponse {
        status: String::from("HTTP/2 200 OK"),
        content_type: String::from("Content-Type: 'text/plain'"),
        content: serde_json::to_string(&data).unwrap(),
    }
}

pub fn get_current_brightness(args: Vec<&str>) -> HttpResponse {
    let data = Data {
        time: Local::now(),
        value: fetch_value("brightness", args[0]),
    };

    HttpResponse {
        status: String::from("HTTP/2 200 OK"),
        content_type: String::from("Content-Type: 'text/plain'"),
        content: serde_json::to_string(&data).unwrap(),
    }
}

fn fetch_value(field: &str, node_number: &str) -> f64 {
    let node_number = match node_number.parse::<u8>() {
        Ok(n) => n,
        Err(_) => 255,
    };
    let node_option: Option<NodeInfo> = get_node_container(node_number);

    match node_option {
        Some(node) => {
            let val = match field {
                "temperature" => node.current_values.temperature,
                "pressure" => 255_f64,
                "humidity" => node.current_values.humidity,
                "brightness" => 255_f64,
                &_ => 255_f64,
            };
            insert_node_container(node);
            val
        }
        None => {
            println!("Node not found");
            255_f64
        }
    }
}

pub fn get_all_current_fields(args: Vec<&str>) -> HttpResponse {
    let mut data: Vec<f64> = Vec::new();
    data.push(fetch_value("temperature", args[0]));
    data.push(fetch_value("pressure", args[0]));
    data.push(fetch_value("humidity", args[0]));
    data.push(fetch_value("brightness", args[0]));

    HttpResponse {
        status: String::from("HTTP/2 200 OK"),
        content_type: String::from("Content-Type: 'text/plain'"),
        content: format!("{:?}", data),
    }
}
