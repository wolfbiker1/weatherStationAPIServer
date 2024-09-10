use serde::{Deserialize, Serialize};

pub mod api;
pub mod db;
pub mod global;
pub mod routes;
pub mod udp;

#[derive(Serialize, Deserialize, Debug)]
pub struct Measurements {
    temperature: String,
    humidity: String,
    pressure: String,
    brightness: String,
}

// @deprecated
#[derive(Serialize, Deserialize, Debug)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NodeMeasurements {
    node_number: u8,
    humidity: u16,
    temperature: u16,
    crc: u8,
    is_valid: u8,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[repr(C)]
pub struct StructHeader {
    pub version: u16,
    pub size: u16,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[repr(C)]
pub struct PayloadMetaData {
    pub node_number: u8,
    pub reserved: [u8; 3],
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[repr(C)]
pub struct DeviceEnvironmentData {
    pub total_lost_packages: u8,
    pub retransmits_for_this_package: u8,
    pub signal_strength: u8,
    pub reserved: u8,
    pub battery_health: u16,
    pub current_channel: u8,
    pub reserved1: [u8; 3],
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[repr(C)]
pub struct Am2302Readings {
    pub humidity: u16,
    pub temperature: u16,
    pub crc: u8,
    pub is_valid: u8,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[repr(C)]
pub struct NodePackage {
    pub header: StructHeader,
    pub meta: PayloadMetaData,
    pub environment: DeviceEnvironmentData,
    pub readings: Am2302Readings, // This struct needs to be defined
}
