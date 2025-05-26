 use tokio::net::TcpListener; 
2. use tokio::io::{AsyncBufReadExt, BufReader}; 
3. use serde::Deserialize; 
4. use reqwest::Client; 
5.   
6. #[derive(Deserialize, Debug)] 
7. struct SensorData { 
8.     timestamp: String, 
9.     sensor_id: String, 
10.     location: String, 
11.     process_stage: String, 
12.     temperature_celsius: f32, 
13.     humidity_percent: f32, 
14. } 
15.   
16. #[tokio::main] 
17. async fn main() -> Result<(), Box<dyn std::error::error="">> { 
18.     let listener = TcpListener::bind("0.0.0.0:9000").await?; 
19.     let influx_url = 
"http://localhost:8086/api/v2/write?org=ITS&bucket=sensor&precision=s"; 
20.     let token = 
"7cxktB5BI0lWjTM77vAqPL8WaTdXz4X7GLTqA63JI6JF4jjaqe2yw4LgDqCTg9U_JpsvaSgT8mjeS
 VDLDMv59Q=="; 
21.     let client = Client::new(); 
22.   
23.     println!("🚪 TCP Server listening on port 9000..."); 
24.   
25.     loop { 
26.         let (socket, addr) = listener.accept().await?; 
27.         println!("🔌 Koneksi masuk dari {}", addr); 
28.   
29.         let client = client.clone(); 
30.         let influx_url = influx_url.to_string(); 
31.         let token = token.to_string(); 
32.   
33.         tokio::spawn(async move { 
34.             let reader = BufReader::new(socket); 
35.             let mut lines = reader.lines(); 
36.   
37.             while let Ok(Some(line)) = lines.next_line().await { 
38.                 match serde_json::from_str::<sensordata>(&line) { 
39.                     Ok(data) => { 
40.                         println!("📥 Data diterima: {:?}", data); 
41.   
42.                         // Line Protocol format: measurement,tag1=value1 
field1=val1,field2=val2 timestamp 
43.                         let line = format!( 
44.                             "monitoring,sensor_id={},location={},stage={
 } temperature={},humidity={} {}", 
45.                             data.sensor_id.replace(" ", "\\ "), 
46.                             data.location.replace(" ", "\\ "), 
47.                             data.process_stage.replace(" ", "\\ "), 
48.                             data.temperature_celsius, 
49.                             data.humidity_percent, 
50.                             chrono::DateTime::parse_from_rfc3339(&data.t
 imestamp) 
51.                                 .unwrap() 
52.                                 .timestamp() 
53.                         ); 
54.   
55.                         // Kirim ke InfluxDB 
56.                         let res = client.post(&influx_url) 
57.                             .header("Authorization", format!("Token {}", 
token)) 
58.                             .header("Content-Type", "text/plain") 
59.                             .body(line) 
60.                             .send() 
61.                             .await; 
62.   
63.                         match res { 
64.                             Ok(resp) if resp.status().is_success() => { 
65.                                 println!("✅ Data dikirim ke 
InfluxDB"); 
66.                             }, 
67.                             Ok(resp) => { 
68.                                 println!("⚠ Gagal kirim ke InfluxDB: 
{}", resp.status()); 
69.                             }, 
70.                             Err(e) => { 
71.                                 println!("❌ HTTP Error: {}", e); 
72.                             } 
73.                         } 
74.                     }, 
75.                     Err(e) => println!("❌ Format JSON tidak valid: 
{}", e), 
76.                 } 
77.             } 
78.         }); 
79.     } 
80. }</sensordata></dyn> 
