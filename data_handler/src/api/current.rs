use super::super::global::current::read_static_value;
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
    let val = read_static_value(field, node_number);
    match val {
        Ok(res) => res.strip_suffix("\n").unwrap().parse::<f64>().unwrap(),
        Err(_) => -1.0,
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
