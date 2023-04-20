use serde::{Deserialize, Serialize};

pub mod api;
pub mod db;
pub mod global;
pub mod routes;
pub mod udp;

const FIELDS: &[&str; 4] = &["temperature", "pressure", "humidity", "brightness"];

#[derive(Serialize, Deserialize, Debug)]
pub struct Measurements {
    temperature: String,
    humidity: String,
    pressure: String,
    brightness: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NodeMeasurements {
    node_number: u8,
    humidity: u16,
    temperature: u16,
    crc: u8,
    is_valid: u8
}

