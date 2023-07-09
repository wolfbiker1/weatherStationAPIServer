use serde::{Deserialize, Serialize};

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
