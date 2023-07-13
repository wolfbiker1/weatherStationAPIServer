use super::super::global::current::update_static_values;
use super::super::global::node::node_info::*;
use super::super::NodeMeasurements;
use std::sync::mpsc::Receiver;
use std::sync::MutexGuard;

pub fn listen_for_node_measurement(udp_receiver: Receiver<Vec<u8>>) {
    for measure_data in udp_receiver {
        unsafe {
            let data: *const NodeMeasurements = measure_data.as_ptr() as *const NodeMeasurements;
            apply_current_node_measurements(*data);
        }
    }
}

fn apply_current_node_measurements<'a>(measurements: NodeMeasurements) {
    let node_box: Option<(NodeInfo, MutexGuard<'a, Vec<NodeInfo>>)> = get_node_container(measurements.node_number);
    match node_box {
        Some(mut node) => {
            node.0.update_timestamp();
            for field in node.0.get_fields() {
                let value: f64 = match field {
                    "temperature" => (measurements.temperature as f64 / 10_f64) as f64,
                    "pressure" => 0.0,
                    "humidity" => (measurements.humidity as f64 / 10_f64) as f64,
                    "brightness" => 0.0,
                    _ => 0.0,
                };
                node.0.node_update_current(field, value);
                node.0.node_insert_measurement(&field, value, measurements.node_number);

                let res = update_static_values(&field, value, measurements.node_number);

                match res {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Something went wrong. {}", e)
                    }
                }
            }
            // @todo!
            insert_node_container(node.0, node.1);
        }
        None => {}
    }
}

// fn apply_current_measurements(measurements: Measurements) {
//     for field in FIELDS {
//         let value: f64 = match *field {
//             "temperature" => measurements.temperature.parse::<f64>().unwrap(),
//             "pressure" => measurements.pressure.parse::<f64>().unwrap(),
//             "humidity" => measurements.humidity.parse::<f64>().unwrap(),
//             "brightness" => measurements.brightness.parse::<f64>().unwrap(),
//             _ => 0.0,
//         };
//         insert_in_db(*field, value, 0);
//         update_static_values(*field, value, 0);
//     }
// }
