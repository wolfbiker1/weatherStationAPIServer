use atomic_float::AtomicF64;
use std::{io::{Cursor, Read, Seek, SeekFrom, Write}, sync::mpsc::Receiver};
use super::super::FIELDS;
use std::io::prelude::*;

pub static TEMPERATURE: AtomicF64 = AtomicF64::new(0.0);
pub static PRESSURE: AtomicF64 = AtomicF64::new(0.0);
pub static HUMIDITY: AtomicF64 = AtomicF64::new(0.0);
pub static BRIGHTNESS: AtomicF64 = AtomicF64::new(0.0);

fn write_ten_bytes_at_end<W: Write + Seek>(writer: &mut W) -> std::io::Result<()> {
    writer.seek(SeekFrom::End(-10))?;

    for i in 0..10 {
        writer.write(&[i])?;
    }

    // all went well
    Ok(())
}


fn mem_buffer_listener (receiver: Receiver<u8>) {
    let mock_file: Vec<f64> = Vec::with_capacity(FIELDS.len()); 
    let mut c = Cursor::new(mock_file);
    c.set_position(0);

    for value in receiver {
        // write_ten_bytes_at_end(&mut c);
    }

}