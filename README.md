# Tugas-isi-kelompok-3
Sistem Monitoring Temperatur dan Kelembapan pada Proses Fermentasi Kedelai untuk Optimasi Kualitas Tempe dan Oncom Hitam

Nama Kelompok:
Anggota Kelompok:	
1. Muhammad 'Azmilfadhil Syamsudin	(2042231003)
2. Zudan Rizky Aditya	(2042231007)
3. Bagus Wijaksono	(2042231039)

Dosen Pengampu:Ahmad Radhy, S.Si., M.Si
Mata Kuliah: Interkoneksi Sistem Instrumentasi

Latar Belakang
Kedelai merupakan komoditas penting di Indonesia yang banyak diolah menjadi produk fermentasi seperti tempe dan oncom hitam. Tempe dikenal luas karena kandungan gizi tinggi, sementara oncom hitam memiliki nilai fungsional dan memanfaatkan limbah pertanian. Proses fermentasi kedua produk ini sangat bergantung pada kondisi lingkungan, terutama suhu dan kelembapan, yang memengaruhi aktivitas kapang Rhizopus sp Di tingkat UMKM, proses fermentasi masih banyak dilakukan secara manual, sehingga rentan terhadap fluktuasi lingkungan yang dapat menurunkan kualitas dan konsistensi produk. Sebagai solusi, teknologi sistem monitoring berbasis sensor dapat diterapkan untuk memantau suhu dan kelembapan secara real-time. Pendekatan ini membantu menjaga kondisi fermentasi tetap optimal, meningkatkan efisiensi, dan menstandarkan kualitas produk. Oleh karena itu, pengembangan sistem monitoring suhu dan kelembapan untuk fermentasi kedelai menjadi penting dalam upaya meningkatkan mutu tempe dan oncom hitam secara berkelanjutan.

Rumusan Masalah
1. Bagaimana menciptakan kondisi suhu dan kelembapan yang optimal dan stabil selama proses fermentasi tempe dan oncom hitam agar mendukung pertumbuhan kapang Rhizopus sp?
2. Bagaimana merancang dan mengimplementasikan sistem inkubator fermentasi berbasis sensor SHT dan mikrokontroler yang mampu memantau serta mengatur suhu dan kelembapan secara otomatis dan real-time?
3. Sejauh mana efektivitas sistem monitoring dan kontrol suhu-kelembapan dalam meningkatkan konsistensi kualitas serta efisiensi waktu fermentasi pada produksi tempe dan oncom hitam?

Tinjauan Penelitian
1. Mengembangkan sistem monitoring dan kontrol otomatis terhadap parameter suhu dan kelembapan selama proses fermentasi tempe dan oncom hitam menggunakan sensor SHT serta mikrokontroler berbasis IoT.
2. Merancang inkubator fermentasi  yang mampu menciptakan  lingkungan stabil dan sesuai  dengan kebutuhan pertumbuhan  kapang Rhizopus sp guna  menunjang keberhasilan fermentasi.
3. Meningkatkan efisiensi proses fermentasi melalui penciptaan kondisi lingkungan yang ideal, sehingga dapat mempercepat waktu fermentasi serta menghasilkan tempe dan oncom hitam dengan kualitas yang lebih konsisten dan optimal.

Manfaat
1. Bagi mahasiswa, meningkatkan pemahaman tentang prinsip-prinsip fermentasi kedelai, khususnya tempe dan oncom hitam, serta faktor-faktor lingkungan yang mempengaruhinya.
2. Bagi UMKM, membantu menstandarisasi dan meningkatkan konsistensi kualitas produk tempe dan oncom hitam, sehingga lebih kompetitif di pasar lokal maupun nasional.
3. Bagi masyarakat umum, Menjamin ketersediaan produk tempe dan oncom hitam berkualitas tinggi yang lebih konsisten, higienis, dan bernutrisi bagi konsumen.


Pengertian  dari Tools
1. Sensor SHT20
Sensor SHT20 adalah sensor digital generasi keempat buatan Sensirion yang dirancang untuk mengukur suhu dan kelembapan dengan akurasi tinggi serta konsumsi daya yang sangat rendah. Sensor ini menggunakan antarmuka komunikasi I2C dan memiliki rentang pengukuran suhu dari –40°C hingga +125°C serta kelembapan relatif dari 0 hingga 100% RH.
2. Antar Muka RS-485
Antarmuka RS-485 umumnya digunakan untuk membantu mikrokontroller dalam melakukan komunikasi data secara serial. RS-485 menggunakan dua kabel untuk mengirimkan sinyal data dan tidak memerlukan commond ground. Sistem penyaluran data ini sering disebut dengan system differensial atau balanced.
3. TCP (Transmission Control Protocol)
TCP (Transmission Control Protocol) merupakan protokol jaringan yang banyak digunakan dalam komunikasi data berbasis internet dan intranet. Dalam proyek ini, TCP Serve digunakan sebagai antarmuka penerima data dari sensor yang dibaca oleh Modbus client. Rust, sebagai bahasa pemrograman sistem modern, menawarkan performa tinggi dan keamanan memori tanpa garbage collector.
4. InfluxDB sebagai Database Time-Series
InfluxDB adalah basis data time-series yang dirancang khusus untuk menangani data yang bersifat waktu (timestamp-based) seperti suhu, kelembaban, tekanan, dan data IoT lainnya. InfluxDB unggul dalam pencatatan dan kueri data berdasarkan waktu, serta memiliki struktur penyimpanan yang efisien dan query language yang sederhana (InfluxQL atau Flux). Dalam sistem ini, data yang diterima oleh TCP Server akan disimpan ke dalam InfluxDB dengan metadata waktu pencatatan. Penyimpanan ini penting untuk analisis tren, monitoring historis, serta integrasi dengan sistem visualisasi data real-time
5. Visualisasi Real-Time Menggunakan Grafana
Grafana adalah platform open-source yang digunakan untuk visualisasi data dalam bentuk grafik interaktif, tabel, dan alert. Grafana mendukung berbagai sumber data, termasuk InfluxDB, dan dapat diakses melalui antarmuka web. Dalam proyek ini, Grafana digunakan untuk menampilkan data suhu dan kelembaban dari gudang fermentasi secara real-time.

Metode
1. Kode Rust Modbus Client Untuk membaca data dari sensor menggunakan protokol Modbus RTU, langkah pertama yang dilakukan adalah menghubungkan sensor ke sistem melalui koneksi serial, yaitu menggunakan antarmuka RS-485 to USB. Pada sistem operasi Linux, koneksi ini biasanya dikenali sebagai port serial dengan path /dev/ttyUSB0.
2. Kode Rust TCP Server tahap kedua adalah menerima data dari client menggunakan program Rust TCP Server. Server ini terus-menerus membaca koneksi pada port USB. Ketika data JSON diterima, program memprosesnya menggunakan pustaka serde_json untuk memparsing informasi yang dikirim oleh client.
3. Konfigurasi InfluxDB dan Integrasi server dibangun untuk berjalan di port 7878 pada localhost, dan dirancang agar dapat menerima koneksi dari beberapa client secara asynchronous. Hal ini memungkinkan sistem untuk menangani banyak koneksi secara bersamaan tanpa mengganggu performa atau menghambat proses lainnya.
4. Dashboard Grafana tahap akhir adalah menampilkan data dalam bentuk visual menggunakan Grafana. Grafana dihubungkan ke InfluxDB untuk menampilkan grafik suhu dan kelembaban secara real-time

Codingan Sensor.src
use tokio_modbus::{client::rtu, prelude::*};
use tokio_serial::{SerialPortBuilderExt, Parity, StopBits, DataBits};
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use serde::Serialize;
use chrono::Utc;
use std::error::Error;
use tokio::time::{sleep, Duration};

#[derive(Serialize)]
struct SensorData {
    timestamp: String,
    sensor_id: String,
    location: String,
    process_stage: String,
    temperature_celsius: f32,
    humidity_percent: f32,
}

async fn read_sensor(slave: u8) -> Result<Vec<u16>, Box<dyn Error>> {
    let builder = tokio_serial::new("/dev/ttyUSB0", 9600)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .data_bits(DataBits::Eight)
        .timeout(std::time::Duration::from_secs(1));

    let port = builder.open_native_async()?;
    let mut ctx = rtu::connect_slave(port, Slave(slave)).await?;
    let response = ctx.read_input_registers(1, 2).await?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    loop {
        match read_sensor(1).await {
            Ok(response) if response.len() == 2 => {
                let temp = response[0] as f32 / 10.0;
                let rh = response[1] as f32 / 10.0;

                println!("📡 Temp: {:.1} °C | RH: {:.1} %", temp, rh);

                let data = SensorData {
                    timestamp: Utc::now().to_rfc3339(),
                    sensor_id: "SHT20".into(),
                    location: "Ruang Fermentasi".into(),
                    process_stage: "Fermentasi Tempe dan Oncom Hitam".into(),
                    temperature_celsius: temp,
                    humidity_percent: rh,
                };

                let json = serde_json::to_string(&data)?;
                
                match TcpStream::connect("127.0.0.1:9000").await {
                    Ok(mut stream) => {
                        stream.write_all(json.as_bytes()).await?;
                        stream.write_all(b"\n").await?;
                        println!("✅ Data dikirim ke TCP server");
                    },
                    Err(e) => {
                        println!("❌ Gagal konek ke TCP server: {}", e);
                    }
                }
            },
            Ok(other) => {
                println!("⚠️ Data tidak lengkap: {:?}", other);
            },
            Err(e) => {
                println!("❌ Gagal baca sensor: {}", e);
            }
        }

        sleep(Duration::from_secs(5)).await;
    }
}

Codingan Server.src
use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, BufReader};
use serde::Deserialize;
use reqwest::Client;

use ethers::prelude::*;
use ethers::abi::Abi;
use std::{fs, sync::Arc};
use chrono::DateTime;

#[derive(Deserialize, Debug)]
struct SensorData {
    timestamp: String,
    sensor_id: String,
    location: String,
    process_stage: String,
    temperature_celsius: f32,
    humidity_percent: f32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // --- InfluxDB setup ---
    let influx_url = "http://localhost:8086/api/v2/write?org=bagus&bucket=sensor&precision=s";
    let influx_token = "7cxktB5BI0lWjTM77vAqPL8WaTdXz4X7GLTqA63JI6JF4jjaqe2yw4LgDqCTg9U_JpsvaSgT8mjeSVDLDMv59Q==";
    let http_client = Client::new();

    // --- Ethereum setup ---
    let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    let wallet: LocalWallet = "0xf90efc0585aca78cae0610d22d8c001b8184505886b342bce47ae17b3ac221d8"
        .parse::<LocalWallet>()?
        .with_chain_id(1337u64);
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    // Baca dan parse ABI dan bytecode dengan benar
    let abi_str = fs::read_to_string("build/SensorStorage.abi")?;
    let bytecode_str = fs::read_to_string("build/SensorStorage.bin")?;

    let abi: Abi = serde_json::from_str(&abi_str)?;
    let bytecode = bytecode_str.trim().parse::<Bytes>()?;

    let factory = ContractFactory::new(abi, bytecode, client.clone());

    let contract = factory.deploy(())?.send().await?;
    println!("✅ Smart contract deployed at: {:?}", contract.address());

    // --- TCP Server ---
    let listener = TcpListener::bind("0.0.0.0:9000").await?;
    println!("🚪 TCP Server listening on port 9000...");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("🔌 New connection from {}", addr);

        let influx_url = influx_url.to_string();
        let influx_token = influx_token.to_string();
        let http_client = http_client.clone();
        let contract = contract.clone();

        tokio::spawn(async move {
            let reader = BufReader::new(socket);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                match serde_json::from_str::<SensorData>(&line) {
                    Ok(data) => {
                        println!("📥 Received sensor data: {:?}", data);

                        // --- InfluxDB Write ---
                        let timestamp = DateTime::parse_from_rfc3339(&data.timestamp)
                            .unwrap()
                            .timestamp();

                        let line_protocol = format!(
                            "monitoring,sensor_id={},location={},stage={} temperature={},humidity={} {}",
                            data.sensor_id.replace(" ", "\\ "),
                            data.location.replace(" ", "\\ "),
                            data.process_stage.replace(" ", "\\ "),
                            data.temperature_celsius,
                            data.humidity_percent,
                            timestamp
                        );

                        match http_client
                            .post(&influx_url)
                            .header("Authorization", format!("Token {}", influx_token))
                            .header("Content-Type", "text/plain")
                            .body(line_protocol)
                            .send()
                            .await
                        {
                            Ok(resp) if resp.status().is_success() => {
                                println!("✅ InfluxDB: data written");
                            }
                            Ok(resp) => {
                                println!("⚠️ InfluxDB error: {}", resp.status());
                            }
                            Err(e) => {
                                println!("❌ InfluxDB HTTP error: {}", e);
                            }
                        }

                        // --- Ethereum Contract Write ---
                        let method_call = contract
    .method::<_, H256>("storeData", (
        timestamp as u64,
        data.sensor_id.clone(),
        data.location.clone(),
        data.process_stage.clone(),
        (data.temperature_celsius * 100.0) as i64,
        (data.humidity_percent * 100.0) as i64,
    ))
    .unwrap();

let tx = method_call.send().await;

                        match tx {
                            Ok(pending_tx) => {
                                println!("📡 Ethereum: tx sent: {:?}", pending_tx);
                            }
                            Err(e) => {
                                println!("❌ Ethereum tx error: {:?}", e);
                            }
                        }
                    }
                    Err(e) => println!("❌ Invalid JSON received: {}", e),
                }
            }
        });
    }
}


Codingan Web3 (script.js)
const contractAddress = "0x25b23ee3f98f8ee75150a6b76085c825e9c58909"; // Ganti dengan alamat kontrakmu
const abiPath = "abi/SensorStorage.abi"; // Pastikan file ABI ini tersedia

let chart;

async function loadSensorData() {
  const abiRes = await fetch(abiPath);
  const abi = await abiRes.json();

  const provider = new ethers.BrowserProvider(window.ethereum || "http://localhost:8545");
  await provider.send("eth_requestAccounts", []);
  const signer = await provider.getSigner();
  const contract = new ethers.Contract(contractAddress, abi, signer);

  const filter = contract.filters.DataStored();
  const events = await contract.queryFilter(filter, 0, "latest");

  const tableBody = document.querySelector("#sensorTable tbody");
  tableBody.innerHTML = "";

  const labels = [];
  const temps = [];
  const hums = [];

  events.forEach((e) => {
    const data = e.args;
    const timeStr = new Date(Number(data.timestamp) * 1000).toLocaleString();
    const temp = Number(data.temperature) / 100;
    const hum = Number(data.humidity) / 100;

    tableBody.innerHTML += `
      <tr>
        <td>${timeStr}</td>
        <td>${data.sensorId}</td>
        <td>${data.location}</td>
        <td>${data.stage}</td>
        <td>${temp.toFixed(2)}</td>
        <td>${hum.toFixed(2)}</td>
      </tr>
    `;

    labels.push(timeStr);
    temps.push(temp);
    hums.push(hum);
  });

  renderChart(labels, temps, hums);

  // Deteksi berdasarkan data terakhir
  if (events.length > 0) {
    const latest = events[events.length - 1];
    const latestTemp = Number(latest.args.temperature) / 100;
    const latestHum = Number(latest.args.humidity) / 100;
    deteksiProduksi(latestTemp, latestHum);
  }
}

function renderChart(labels, temps, hums) {
  const ctx = document.getElementById('chart').getContext('2d');

  if (chart) chart.destroy();

  chart = new Chart(ctx, {
    type: 'line',
    data: {
      labels,
      datasets: [
        {
          label: "Temperature (°C)",
          data: temps,
          borderColor: 'rgba(255, 99, 132, 1)',
          fill: false
        },
        {
          label: "Humidity (%)",
          data: hums,
          borderColor: 'rgba(54, 162, 235, 1)',
          fill: false
        }
      ]
    },
    options: {
      responsive: true,
      scales: {
        y: { beginAtZero: true }
      }
    }
  });
}

function deteksiProduksi(temp, hum) {
  const setTemp = parseFloat(document.getElementById("setTemp").value);
  const setHum = parseFloat(document.getElementById("setHum").value);
  const hasil = document.getElementById("hasilDeteksi");

  if (temp >= 25 && hum >= 60) {
    hasil.textContent = "Produksi Tempe ✅";
    hasil.style.color = "green";
  } else if (temp >= 24 && hum >= 48) {
    hasil.textContent = "Produksi Oncom Hitam ✅";
    hasil.style.color = "blue";
  } else {
    hasil.textContent = "Data tidak cocok dengan produksi tempe atau oncom hitam.";
    hasil.style.color = "red";
  }
}

 Codingan Web3 (Index.html)
 <!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>Sensor Dashboard</title>
  <script src="https://cdn.jsdelivr.net/npm/ethers/dist/ethers.umd.min.js"></script>
  <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
  <link rel="stylesheet" href="style.css">
</head>
<body>
  <h1>🌡️ Sensor Data Dashboard</h1>

  <button onclick="loadSensorData()">🔄 Load Sensor Data</button>

  <h2>🎯 Deteksi Produksi Otomatis</h2>
  <div>
    <label for="setTemp">Set Point Suhu Minimum (°C):</label>
    <input type="number" id="setTemp" value="31" step="0.1"><br>

    <label for="setHum">Set Point Kelembapan Minimum (%):</label>
    <input type="number" id="setHum" value="66" step="0.1"><br>

    <p><strong>Hasil Deteksi:</strong> <span id="hasilDeteksi">-</span></p>
  </div>

  <table id="sensorTable">
    <thead>
      <tr>
        <th>Timestamp</th>
        <th>Sensor ID</th>
        <th>Location</th>
        <th>Stage</th>
        <th>Temperature (°C)</th>
        <th>Humidity (%)</th>
      </tr>
    </thead>
    <tbody></tbody>
  </table>

  <canvas id="chart" height="100"></canvas>

  <script src="script.js"></script>
</body>
</html>


Codingan pyqt
import tkinter as tk
from tkinter import ttk
import requests
import threading
import time
import csv
from io import StringIO
from matplotlib.backends.backend_tkagg import FigureCanvasTkAgg
from matplotlib.figure import Figure
from collections import deque

# Konfigurasi InfluxDB
INFLUX_QUERY_URL = "http://localhost:8086/api/v2/query"
ORG = "bagus"
BUCKET = "sensor"
TOKEN = "7cxktB5BI0lWjTM77vAqPL8WaTdXz4X7GLTqA63JI6JF4jjaqe2yw4LgDqCTg9U_JpsvaSgT8mjeSVDLDMv59Q=="

# Riwayat data
history_length = 50
temp_history = deque(maxlen=history_length)
rh_history = deque(maxlen=history_length)
time_history = deque(maxlen=history_length)

def get_latest_data():
    flux_query = f'''
    from(bucket: "{BUCKET}")
      |> range(start: -1m)
      |> filter(fn: (r) => r._measurement == "monitoring")
      |> filter(fn: (r) => r._field == "temperature" or r._field == "humidity")
      |> last()
    '''

    headers = {
        "Authorization": f"Token {TOKEN}",
        "Content-Type": "application/vnd.flux",
        "Accept": "application/csv"
    }

    try:
        response = requests.post(
            INFLUX_QUERY_URL,
            params={"org": ORG},
            headers=headers,
            data=flux_query
        )

        reader = csv.DictReader(StringIO(response.text))
        data = {}
        for row in reader:
            try:
                field = row["_field"]
                value = float(row["_value"])
                data[field] = value
            except:
                continue

        if "temperature" in data and "humidity" in data:
            return data["temperature"], data["humidity"]
        return None
    except Exception as e:
        print("❌ Exception query Influx:", e)
        return None

def get_data_range(start_time, end_time):
    flux_query = f'''
    from(bucket: "{BUCKET}")
      |> range(start: {start_time}, stop: {end_time})
      |> filter(fn: (r) => r._measurement == "monitoring")
      |> filter(fn: (r) => r._field == "temperature" or r._field == "humidity")
    '''

    headers = {
        "Authorization": f"Token {TOKEN}",
        "Content-Type": "application/vnd.flux",
        "Accept": "application/csv"
    }

    try:
        response = requests.post(
            INFLUX_QUERY_URL,
            params={"org": ORG},
            headers=headers,
            data=flux_query
        )

        reader = csv.DictReader(StringIO(response.text))
        temp_map = {}
        rh_map = {}

        for row in reader:
            try:
                t = row["_time"]
                field = row["_field"]
                value = float(row["_value"])
                if field == "temperature":
                    temp_map[t] = value
                elif field == "humidity":
                    rh_map[t] = value
            except:
                continue

        sorted_keys = sorted(set(temp_map.keys()) & set(rh_map.keys()))
        temps = [temp_map[t] for t in sorted_keys]
        rhs = [rh_map[t] for t in sorted_keys]
        times = [t[11:19] for t in sorted_keys]  # jam:menit:detik

        return temps, rhs, times
    except Exception as e:
        print("❌ Exception query Influx:", e)
        return [], [], []

def update_data():
    while True:
        result = get_latest_data()
        current_time = time.strftime('%H:%M:%S')

        if result:
            temp, rh = result
            label_temp.config(text=f"Suhu: {temp:.1f} °C")
            label_rh.config(text=f"Kelembaban: {rh:.1f} %")
            status_label.config(text="✅ Data dari Influx")

            temp_history.append(temp)
            rh_history.append(rh)
            time_history.append(current_time)

            plot_graph()
        else:
            label_temp.config(text="Suhu: ---")
            label_rh.config(text="Kelembaban: ---")
            status_label.config(text="❌ Gagal ambil data")

        time.sleep(2)

def plot_graph():
    ax1.clear()
    ax2.clear()

    # Set background ke hitam
    fig.patch.set_facecolor('black')
    ax1.set_facecolor('black')
    ax2.set_facecolor('black')

    x = list(range(len(time_history)))
    times = list(time_history)

    # Plot data dengan garis dan warna yang kontras
    ax1.plot(x, list(temp_history), label='Suhu (°C)', color='red', marker='o', linestyle='-')
    ax2.plot(x, list(rh_history), label='Kelembaban (%)', color='cyan', marker='x', linestyle='-')

    ax1.set_title("Grafik Suhu", color='white')
    ax2.set_title("Grafik Kelembaban", color='white')
    ax1.set_ylabel("°C", color='white')
    ax2.set_ylabel("%", color='white')

    # Tampilkan hanya label waktu setiap 5 data
    interval = 5
    tick_positions = x[::interval]
    tick_labels = times[::interval]

    ax1.set_xticks(tick_positions)
    ax2.set_xticks(tick_positions)
    ax1.set_xticklabels(tick_labels, rotation=45, ha="right", color='white')
    ax2.set_xticklabels(tick_labels, rotation=45, ha="right", color='white')

    for ax in [ax1, ax2]:
        ax.tick_params(axis='y', colors='white')
        ax.tick_params(axis='x', colors='white')
        ax.grid(True, linestyle='--', alpha=0.3, color='gray')
        for spine in ax.spines.values():
            spine.set_color('white')

    fig.tight_layout()
    canvas.draw()

def show_history():
    start = entry_start.get()
    end = entry_end.get()
    temps, rhs, times = get_data_range(start, end)

    if temps and rhs:
        temp_history.clear()
        rh_history.clear()
        time_history.clear()

        temp_history.extend(temps)
        rh_history.extend(rhs)
        time_history.extend(times)

        label_temp.config(text="(Hist) Suhu: -- °C")
        label_rh.config(text="(Hist) RH: -- %")
        status_label.config(text="📈 Menampilkan data historis")
        plot_graph()
    else:
        status_label.config(text="❌ Tidak ada data historis")

# GUI Setup
root = tk.Tk()
root.title("Monitor SHT20 dari InfluxDB")
root.geometry("800x650")

label_temp = tk.Label(root, text="Suhu: -- °C", font=("Helvetica", 16))
label_temp.pack(pady=5)

label_rh = tk.Label(root, text="Kelembaban: -- %", font=("Helvetica", 16))
label_rh.pack(pady=5)

status_label = tk.Label(root, text="Status: ---", fg="blue")
status_label.pack(pady=5)

frame_input = tk.Frame(root)
frame_input.pack(pady=5)

tk.Label(frame_input, text="Start (RFC3339):").grid(row=0, column=0, padx=5)
entry_start = tk.Entry(frame_input, width=30)
entry_start.grid(row=0, column=1)

tk.Label(frame_input, text="End (RFC3339):").grid(row=1, column=0, padx=5)
entry_end = tk.Entry(frame_input, width=30)
entry_end.grid(row=1, column=1)

btn_show = tk.Button(frame_input, text="Tampilkan Riwayat", command=show_history)
btn_show.grid(row=0, column=2, rowspan=2, padx=10)

fig = Figure(figsize=(6, 4), dpi=100)
ax1 = fig.add_subplot(211)
ax2 = fig.add_subplot(212)

canvas = FigureCanvasTkAgg(fig, master=root)
canvas.get_tk_widget().pack(pady=10)

# Mulai update realtime
threading.Thread(target=update_data, daemon=True).start()

root.mainloop()

