use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use serde::Deserialize;
use reqwest::blocking::Client;

#[derive(Deserialize, Debug)]
struct SensorData {
    timestamp: String,
    sensor_id: String,
    location: String,
    process_stage: String,
    temperature_celsius: f64,
    humidity_percent: f64,
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                let data = String::from_utf8_lossy(&buffer[..n]).to_string();
                println!("Received: {}", data);

                if let Err(e) = write_to_influx_v2(&data) {
                    eprintln!("InfluxDB Error: {}", e);
                }

                stream.write_all(b"OK\n").unwrap_or_default();
            }
            Err(_) => break,
        }
    }
}

fn write_to_influx_v2(data: &str) -> Result<(), Box<dyn std::error::Error>> {
    let token = "9BuYLHAV7W_q1-V3HPHMOtv0xEQMGc24LRxD7FHVYgRDsUHlqD5mkuTblSlz8ZAmJ8sSqUnCtuAEdb2PoVxvZw==";
    let org = "rival team";
    let bucket = "sensor_data";
    let url = format!(
        "http://localhost:8086/api/v2/write?org={}&bucket={}&precision=s",
        org, bucket
    );

    // Parse JSON string
    let sensor: SensorData = serde_json::from_str(data)?;

    // Format line protocol
    let line = format!(
        "sensor_data,sensor_id={},location={},process_stage={} temperature_celsius={},humidity_percent={} {}",
        sensor.sensor_id.replace(" ", "\\ "),
        sensor.location.replace(" ", "\\ "),
        sensor.process_stage.replace(" ", "\\ "),
        sensor.temperature_celsius,
        sensor.humidity_percent,
        chrono::DateTime::parse_from_rfc3339(&sensor.timestamp)?.timestamp()
    );

    let client = Client::new();
    let res = client
        .post(&url)
        .header("Authorization", format!("Token {}", token))
        .header("Content-Type", "text/plain")
        .body(line)
        .send()?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(format!("Failed with status: {}", res.status()).into())
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:7878")?;
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
