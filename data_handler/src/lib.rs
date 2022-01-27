use serde::{Deserialize, Serialize};

pub mod api;
pub mod db;
pub mod routes;
pub mod udp;
pub mod global;

const FIELDS: &[&str; 4] = &["temperature", "pressure", "humidity", "brightness"];

#[derive(Serialize, Deserialize, Debug)]
pub struct Measurements {
    temperature: String,
    humidity: String,
    pressure: String,
    brightness: String,
}
