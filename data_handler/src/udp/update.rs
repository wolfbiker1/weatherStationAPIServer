// use async_std::channel::Sender;

use super::super::{Measurements, FIELDS};
use super::super::db::sqlite::insert_in_db;
use super::super::global::current::update_static_values;
use std::sync::mpsc::{Receiver, Sender};


pub fn listen_for_new_measurement(udp_receiver: Receiver<Vec<u8>>, /* sender_for_current: Sender<f64> */) {
    for measure_data in udp_receiver {
        let data: Measurements = serde_json::from_str(std::str::from_utf8(&measure_data).unwrap()).unwrap();
        apply_current_measurements(data);
    }
}

fn apply_current_measurements(measurements: Measurements) {
    for field in FIELDS {
        let value: f64 = match *field {
            "temperature" => measurements.temperature.parse::<f64>().unwrap(),
            "pressure" => measurements.pressure.parse::<f64>().unwrap(),
            "humidity" => measurements.humidity.parse::<f64>().unwrap(),
            "brightness" => measurements.brightness.parse::<f64>().unwrap(),
            _ => 0.0,
        };
        insert_in_db(*field, value);
        update_static_values(*field, value);
    }
}
