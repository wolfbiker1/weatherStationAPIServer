use super::super::global::current::update_static_values;
use super::super::global::node::node_info::*;
use super::super::global::types::FieldValue;
use super::super::NodePackage;
use std::sync::mpsc::Receiver;
use std::sync::MutexGuard;

pub fn listen_for_node_measurement(udp_receiver: Receiver<Vec<u8>>) {
    for measure_data in udp_receiver {
        unsafe {
            println!("got it");
            let data: *const NodePackage = measure_data.as_ptr() as *const NodePackage;
            apply_current_node_measurements(*data);
        }
    }
}

fn apply_current_node_measurements<'a>(measurements: NodePackage) {
    let node_box: Option<(NodeInfo, MutexGuard<'a, Vec<NodeInfo>>)> =
        get_node_container(measurements.meta.node_number);
    match node_box {
        Some(mut node) => {
            node.0.update_timestamp();
            for field in node.0.get_fields() {
                let value = match field {
                    "temperature" => {
                        FieldValue::Float(measurements.readings.temperature as f64 / 10_f64)
                    }
                    "pressure" => FieldValue::Float(0.0), // Beispiel für eine nicht implementierte Messung
                    "humidity" => FieldValue::Float(measurements.readings.humidity as f64 / 10_f64),
                    "brightness" => FieldValue::Float(0.0), // Beispiel für eine nicht implementierte Messung
                    "signalStrength" => FieldValue::U8(measurements.environment.signal_strength),
                    "maxRetries" => {
                        FieldValue::U8(measurements.environment.retransmits_for_this_package)
                    }
                    "totalLostPackages" => {
                        FieldValue::U8(measurements.environment.total_lost_packages)
                    }
                    "batteryHealth" => FieldValue::U16(measurements.environment.battery_health),
                    _ => FieldValue::Float(-255.0), // Default Wert für nicht erkannte Felder
                };

                // Dann die Funktion aufrufen
                node.0.node_update_current(field, &value);
                node.0.node_insert_measurement(&field, value, measurements.meta.node_number);

                // let value: f64 = match field {
                //     "temperature" => (measurements.readings.temperature as f64 / 10_f64) as f64,
                //     "pressure" => 0.0,
                //     "humidity" => (measurements.readings.humidity as f64 / 10_f64) as f64,
                //     "brightness" => 0.0,
                //     _ => -255.0 as f64,
                // };

                // let value: u8 = match field {
                //     "signalStrength" => measurements.environment.signal_strength,
                //     "maxRetries" => measurements.environment.retransmits_for_this_package,
                //     "totalLostPackages" => measurements.environment.total_lost_packages,
                //     _ => 255,
                // };

                // let value: u16 = match field {
                //     "batteryHealth" => measurements.environment.battery_health,
                //     _ => 255,
                // };

                // node.0.node_update_current(field, value);

                // let res = update_static_values(&field, value, measurements.meta.node_number);

                // match res {
                //     Ok(_) => {}
                //     Err(e) => {
                //         println!("Something went wrong. {}", e)
                //     }
                // }
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
