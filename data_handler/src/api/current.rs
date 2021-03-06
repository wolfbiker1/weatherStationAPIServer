use super::super::global::current::read_static_value;
use ::inet::protocoll::http::HttpResponse;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};

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

pub fn get_current_temp() -> HttpResponse {
    let data = Data {
        time: Local::now(),
        value: fetch_value("temperature"),
    };

    HttpResponse {
        status: String::from("HTTP/2 200 OK"),
        content_type: String::from("Content-Type: 'text/plain'"),
        content: serde_json::to_string(&data).unwrap(),
    }
}

pub fn get_current_pressure() -> HttpResponse {
    let data = Data {
        time: Local::now(),
        value: fetch_value("pressure"),
    };

    HttpResponse {
        status: String::from("HTTP/2 200 OK"),
        content_type: String::from("Content-Type: 'text/plain'"),
        content: serde_json::to_string(&data).unwrap(),
    }
}

pub fn get_current_humidty() -> HttpResponse {
    let data = Data {
        time: Local::now(),
        value: fetch_value("humidity"),
    };

    HttpResponse {
        status: String::from("HTTP/2 200 OK"),
        content_type: String::from("Content-Type: 'text/plain'"),
        content: serde_json::to_string(&data).unwrap(),
    }
}

pub fn get_current_brightness() -> HttpResponse {
    let data = Data {
        time: Local::now(),
        value: fetch_value("brightness"),
    };

    HttpResponse {
        status: String::from("HTTP/2 200 OK"),
        content_type: String::from("Content-Type: 'text/plain'"),
        content: serde_json::to_string(&data).unwrap(),
    }
}

fn fetch_value(field: &str) -> f64 {
    let val = read_static_value(field);
    match val {
        Ok(res) => res.strip_suffix("\n").unwrap().parse::<f64>().unwrap(),
        Err(_) => -1.0,
    }
}

pub fn get_all_current_fields() -> HttpResponse {
    let mut data: Vec<f64> = Vec::new();
    data.push(fetch_value("temperature"));
    data.push(fetch_value("pressure"));
    data.push(fetch_value("humidity"));
    data.push(fetch_value("brightness"));

    HttpResponse {
        status: String::from("HTTP/2 200 OK"),
        content_type: String::from("Content-Type: 'text/plain'"),
        content: format!("{:?}", data),
    }
}
