use atomic_float::AtomicF64;
use chrono::format::format;
use std::{io::{Cursor, Read, Seek, SeekFrom, Write}, sync::mpsc::Receiver};
use super::super::FIELDS;
use std::io::prelude::*;

pub static TEMPERATURE: AtomicF64 = AtomicF64::new(0.0);
pub static PRESSURE: AtomicF64 = AtomicF64::new(0.0);
pub static HUMIDITY: AtomicF64 = AtomicF64::new(0.0);
pub static BRIGHTNESS: AtomicF64 = AtomicF64::new(0.0);


pub fn update_static_values(field_name: &str, value: f64) -> std::io::Result<()> {
    let path = format!("{}/{}/{}", std::env::current_dir().unwrap().display(), "data", field_name);
    let mut file = std::fs::File::create(path)?;
    writeln!(file, "{}", value);
    // file.write_all(&value.to_ne_bytes())?;
    Ok(())
}