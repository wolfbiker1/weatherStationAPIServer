use std::io::Write;

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
