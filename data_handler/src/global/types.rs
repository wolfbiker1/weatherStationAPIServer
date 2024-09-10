use serde::{Deserialize, Serialize};

use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResult {
    pub date_of_record: String,
    pub value: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Peaks {
    pub date: String,
    pub val: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dates {
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatesCollection {
    temperature: serde_json::Value,
    pressure: serde_json::Value,
    humidity: serde_json::Value,
    brightness: serde_json::Value,
}

impl DatesCollection {
    pub fn new() -> DatesCollection {
        DatesCollection {
            temperature: json!({"foo": "bar"}),
            pressure: json!({"foo": "bar"}),
            humidity: json!({"foo": "bar"}),
            brightness: json!({"foo": "bar"}),
        }
    }
    pub fn change_value(mut self, field: &str, value: Vec<String>) -> DatesCollection {
        match field {
            "temperature" => {
                self.temperature = serde_json::Value::String(serde_json::to_string(&value).unwrap())
            }
            "pressure" => {
                self.pressure = serde_json::Value::String(serde_json::to_string(&value).unwrap())
            }
            "humidity" => {
                self.humidity = serde_json::Value::String(serde_json::to_string(&value).unwrap())
            }
            "brightness" => {
                self.brightness = serde_json::Value::String(serde_json::to_string(&value).unwrap())
            }
            _ => {}
        }
        self
    }
}

pub enum FieldValue {
    Float(f64),
    U8(u8),
    U16(u16),
}
