use std::io::Write;

pub fn update_static_values(field_name: &str, value: f64) -> std::io::Result<()> {
    let path = format!(
        "{}/{}/{}",
        std::env::current_dir().unwrap().display(),
        "data",
        field_name
    );
    let mut file = std::fs::File::create(path)?;
    writeln!(file, "{}", value)?;
    Ok(())
}

pub fn read_static_value(field_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = format!(
        "{}/{}/{}",
        std::env::current_dir().unwrap().display(),
        "data",
        field_name
    );
    let f: String = std::fs::read_to_string(path)?.parse()?;
    Ok(f)
}
