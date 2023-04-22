// use async_std::channel::Sender;

use super::super::db::sqlite::insert_in_db;
use super::super::global::current::update_static_values;
use super::super::{Measurements, NodeMeasurements, FIELDS};
use std::sync::mpsc::{Receiver, Sender};

pub fn listen_for_new_measurement(
    udp_receiver: Receiver<Vec<u8>>, /* sender_for_current: Sender<f64> */
) {
    for measure_data in udp_receiver {
        let data: Measurements =
            serde_json::from_str(std::str::from_utf8(&measure_data).unwrap()).unwrap();
        apply_current_measurements(data);
    }
}

pub fn listen_for_node_measurement(udp_receiver: Receiver<Vec<u8>>) {
    for measure_data in udp_receiver {
        unsafe {
            let data: *const NodeMeasurements = measure_data.as_ptr() as *const NodeMeasurements;
            apply_current_node_measurements(*data);
        }
    }
}

fn apply_current_node_measurements(measurements: NodeMeasurements) {
    // just a workaround until crc errors fixed
    let temperature = measurements.temperature as f64 / 10 as f64;
    let humidity = measurements.humidity as f64 / 10 as f64;

    if temperature > 40.0 || temperature < -10.0 {
        return;
    }

    if humidity > 95.0 || humidity < 5.0 {
        return;
    }

    for field in FIELDS {
        let value: f64 = match *field {
            "temperature" => temperature,
            "pressure" => 0.0,
            "humidity" => humidity,
            "brightness" => 0.0,
            _ => 0.0,
        };

        insert_in_db(*field, value);

        let res = update_static_values(*field, value, measurements.node_number);

        match res {
            Ok(_) => {}
            Err(e) => {
                println!("Something went wrong. {}", e)
            }
        }
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
        update_static_values(*field, value, 0);
    }
}
