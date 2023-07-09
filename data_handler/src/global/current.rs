use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn update_static_values(field_name: &str, value: f64, node_number: u8) -> std::io::Result<()> {
    let path = format!(
        "{}/{}/{}_{}",
        std::env::current_dir().unwrap().display(),
        "data",
        field_name,
        node_number
    );
    let mut file = std::fs::File::create(path)?;
    writeln!(file, "{}", value)?;
    Ok(())
}

pub fn read_static_value(
    field_name: &str,
    node_number: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let path = format!(
        "{}/{}/{}_{}",
        std::env::current_dir().unwrap().display(),
        "data",
        field_name,
        node_number
    );
    let f: String = std::fs::read_to_string(path)?.parse()?;
    Ok(f)
}

pub fn create_file(file_name: &str) {
    let path = format!(
        "{}/{}/{}",
        std::env::current_dir().unwrap().display(),
        "data",
        file_name,
    );
    println!("PATH {} ", path);

    let f = OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(path);
    match f {
        Ok(_) => { /* ok */ }
        Err(e) => {
            println!("{}", e)
        }
    }
}

pub fn append_to_file(file_name: &str, data: &str) {
    let path = format!(
        "{}/{}/{}",
        std::env::current_dir().unwrap().display(),
        "data",
        file_name,
    );

    let f = OpenOptions::new().write(true).append(true).open(path);

    match f {
        Ok(mut file) => {
            let result = write!(file, "{},", data);
            match result {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e)
                }
            }
        }
        Err(e) => {
            println!("{}", e)
        }
    }
}

pub fn file_exists(_file_name: &str) -> bool {
    let p = std::env::current_dir().unwrap();

    println!("file_exists PATH {}", p.display());
    Path::new(&p).exists()
}

pub fn read_file(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = format!(
        "{}/{}/{}",
        std::env::current_dir().unwrap().display(),
        "data",
        file_name,
    );
    println!("read_file PATH {}", path);

    let f: String = std::fs::read_to_string(path)?.parse()?;
    Ok(f)
}
