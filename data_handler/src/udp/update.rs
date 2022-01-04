use super::super::{Measurements, FIELDS};
use super::super::db::sqlite::insert_values;
use std::sync::mpsc::Receiver;


pub fn listen_for_new_measurement(receiver: Receiver<Vec<u8>>) {
    for measure_data in receiver {
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
        insert_values(*field, value);
        // update_static_values(*field, value);
    }
}
