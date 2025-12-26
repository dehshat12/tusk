use std::fs;
use std::io::Result;

pub fn load_file(path: &str) -> Result<Vec<String>> {
    let content = fs::read_to_string(path)?;
    Ok(content.split('\n').map(|l| l.to_string()).collect())
}

pub fn save_file(path: &str, lines: &[String]) -> Result<()> {
    let data = lines.join("\n");
    fs::write(path, data)?;
    Ok(())
}
