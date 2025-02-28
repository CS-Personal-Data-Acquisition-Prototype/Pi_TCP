//// filepath: /Users/brad/Library/Mobile Documents/com~apple~CloudDocs/OSU/Capstone/Pi_TCP/src/main.rs
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write};
use rusqlite::{params, Connection};
mod config;
use std::error::Error;
use std::time::Duration;
use socket2::{Socket, Domain, Type};
use serialport::{SerialPort, SerialPortSettings};

fn init_db(conn: &Connection) -> Result<(), Box<dyn Error>> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sensor_data (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sessionID INTEGER,
            timestamp TEXT,
            latitude REAL,
            longitude REAL,
            altitude REAL,
            accel_x REAL,
            accel_y REAL,
            accel_z REAL,
            gyro_x REAL,
            gyro_y REAL,
            gyro_z REAL,
            dac_1 REAL,
            dac_2 REAL,
            dac_3 REAL,
            dac_4 REAL
        )",
        [],
    )?;
    Ok(())
}

fn handle_client(mut stream: TcpStream, conn: &Connection) -> Result<(), Box<dyn Error>> {
    // Wrap the TcpStream in a Socket for advanced options
    let socket = Socket::from(stream.try_clone()?);
    
    // Set keepalive option
    socket.set_keepalive(true)?;
    
    // Set the keepalive time (if supported by your platform)
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    socket.set_tcp_keepalive(&socket2::TcpKeepalive::new().with_time(Duration::from_secs(300)))?;
    
    println!("Starting to collect data from client...");
    let reader = BufReader::new(&stream);
    let mut record_count = 0;
    
    // Process one CSV record per line
    for line in reader.lines() {
        let line = line?;
        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }
        // Split the CSV line into fields (assumes 15 comma-separated values)
        let fields: Vec<&str> = line.trim().split(',').collect();
        if fields.len() < 15 {
            eprintln!("Received incomplete data: {}", line);
            continue;
        }
        // Parse data – note that the first field is sessionID, which might be "None"
        let session_id = if fields[0] == "None" { None } else { fields[0].parse::<i32>().ok() };
        let timestamp = fields[1];
        let latitude = fields[2].parse::<f64>().unwrap_or(0.0);
        let longitude = fields[3].parse::<f64>().unwrap_or(0.0);
        let altitude = fields[4].parse::<f64>().unwrap_or(0.0);
        let accel_x = fields[5].parse::<f64>().unwrap_or(0.0);
        let accel_y = fields[6].parse::<f64>().unwrap_or(0.0);
        let accel_z = fields[7].parse::<f64>().unwrap_or(0.0);
        let gyro_x = fields[8].parse::<f64>().unwrap_or(0.0);
        let gyro_y = fields[9].parse::<f64>().unwrap_or(0.0);
        let gyro_z = fields[10].parse::<f64>().unwrap_or(0.0);
        let dac_1 = fields[11].parse::<f64>().unwrap_or(0.0);
        let dac_2 = fields[12].parse::<f64>().unwrap_or(0.0);
        let dac_3 = fields[13].parse::<f64>().unwrap_or(0.0);
        let dac_4 = fields[14].parse::<f64>().unwrap_or(0.0);

        // Insert the record into the SQLite database
        conn.execute(
            "INSERT INTO sensor_data (
                sessionID, timestamp, latitude, longitude, altitude,
                accel_x, accel_y, accel_z,
                gyro_x, gyro_y, gyro_z,
                dac_1, dac_2, dac_3, dac_4
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![
                session_id,
                timestamp,
                latitude,
                longitude,
                altitude,
                accel_x,
                accel_y,
                accel_z,
                gyro_x,
                gyro_y,
                gyro_z,
                dac_1,
                dac_2,
                dac_3,
                dac_4,
            ],
        )?;

        record_count += 1;
        if record_count % 100 == 0 {
            println!("Processed {} records from current connection", record_count);
        }
    }
    
    println!("Client disconnected. Processed {} total records.", record_count);
    Ok(())
}

fn forward_data(conn: &Connection) -> Result<(), Box<dyn Error>> {
    // Use the config file path, e.g., "server_config.txt"
    let server_ip = config::read_server_ip()?;
    let server_addr = format!("{}:7879", server_ip);

    // Query the stored data (all rows) and format each row as CSV
    let mut stmt = conn.prepare("SELECT sessionID, timestamp, latitude, longitude, altitude, 
                                      accel_x, accel_y, accel_z, 
                                      gyro_x, gyro_y, gyro_z, 
                                      dac_1, dac_2, dac_3, dac_4 FROM sensor_data")?;
    let mut rows = stmt.query([])?;
    let mut csv_data = String::new();

    while let Some(row) = rows.next()? {
        let session_id: Option<i32> = row.get(0)?;
        let timestamp: String = row.get(1)?;
        let latitude: f64 = row.get(2)?;
        let longitude: f64 = row.get(3)?;
        let altitude: f64 = row.get(4)?;
        let accel_x: f64 = row.get(5)?;
        let accel_y: f64 = row.get(6)?;
        let accel_z: f64 = row.get(7)?;
        let gyro_x: f64 = row.get(8)?;
        let gyro_y: f64 = row.get(9)?;
        let gyro_z: f64 = row.get(10)?;
        let dac_1: f64 = row.get(11)?;
        let dac_2: f64 = row.get(12)?;
        let dac_3: f64 = row.get(13)?;
        let dac_4: f64 = row.get(14)?;

        let csv_line = format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            session_id
                .map(|s| s.to_string())
                .unwrap_or("None".to_string()),
            timestamp,
            latitude,
            longitude,
            altitude,
            accel_x,
            accel_y,
            accel_z,
            gyro_x,
            gyro_y,
            gyro_z,
            dac_1,
            dac_2,
            dac_3,
            dac_4
        );
        csv_data.push_str(&csv_line);
    }

    // Connect to the remote server and send the data
    let mut stream = TcpStream::connect(&server_addr)?;
    stream.write_all(csv_data.as_bytes())?;
    println!("Data forwarded to server at {}", server_addr);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Bind the TCP server to all interfaces on port 7878
    let listener = TcpListener::bind("0.0.0.0:7878")?;
    // Open (or create) the SQLite database file on the Raspberry Pi
    let conn = Connection::open("data_acquisition.db")?;
    init_db(&conn)?;

    println!("Rust TCP server listening on port 7878...");

    // Accept connections in a loop
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Client connected: {:?}", stream.peer_addr()?);
                if let Err(e) = handle_client(stream, &conn) {
                    eprintln!("Error while handling client: {}", e);
                }
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    // To forward data, you can call:
    // forward_data(&conn)?;

    Ok(())
}