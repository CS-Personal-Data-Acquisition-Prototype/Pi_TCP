//// filepath: src/config.rs
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// You can define your fixed file path here.
// For a boot process, using an absolute path (e.g., "/etc/myapp/server_config.txt") is recommended.
const CONFIG_FILE_PATH: &str = "./server_config.txt";

pub fn read_server_ip() -> io::Result<String> {
    let file = File::open(CONFIG_FILE_PATH)?;
    let reader = BufReader::new(file);
    // Read the first non-empty line
    for line in reader.lines() {
        let ip = line?;
        if !ip.trim().is_empty() {
            return Ok(ip.trim().to_owned());
        }
    }
    Err(io::Error::new(
        io::ErrorKind::Other,
        "No IP found in config file",
    ))
}