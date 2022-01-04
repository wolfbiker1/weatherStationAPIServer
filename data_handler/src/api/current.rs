use super::forecast::forecast_handler::load_trend_values;
use super::update::update_path_handler::load_current_measurements;
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
    data.push(load_trend_values("temp"));
    data.push(load_trend_values("pressure"));
    data.push(load_trend_values("humidity"));
    data.push(load_trend_values("brightness"));

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
        value: load_current_measurements("temp"),
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
        value: load_current_measurements("pressure"),
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
        value: load_current_measurements("humidity"),
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
        value: load_current_measurements("brightness"),
    };

    HttpResponse {
        status: String::from("HTTP/2 200 OK"),
        content_type: String::from("Content-Type: 'text/plain'"),
        content: serde_json::to_string(&data).unwrap(),
    }
}

pub fn public_api() -> HttpResponse {
    let mut data: Vec<f64> = Vec::new();
    // data.push(load_current_measurements("indoor_temp"));
    data.push(load_current_measurements("temp"));
    data.push(load_current_measurements("pressure"));
    data.push(load_current_measurements("humidity"));
    data.push(load_current_measurements("brightness"));

    HttpResponse {
        status: String::from("HTTP/2 200 OK"),
        content_type: String::from("Content-Type: 'text/plain'"),
        content: format!("{:?}", data),
    }
}
