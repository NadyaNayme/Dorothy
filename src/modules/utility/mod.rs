use std::fs;

pub fn get_percentage(x: f32, y: f32) -> String {
    let result = (x * 100.0) / y;
    return format!("{:.2}%", result);
}

pub fn create_path(path: &str) -> std::io::Result<()> {
    fs::create_dir(path)?;
    Ok(())
}
