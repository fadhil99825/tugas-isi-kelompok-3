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
Sensor SHT20
Sensor SHT20 adalah sensor digital generasi keempat buatan Sensirion yang dirancang untuk mengukur suhu dan kelembapan dengan akurasi tinggi serta konsumsi daya yang sangat rendah. Sensor ini menggunakan antarmuka komunikasi I2C dan memiliki rentang pengukuran suhu dari –40°C hingga +125°C serta kelembapan relatif dari 0 hingga 100% RH.
Antar Muka RS-485
Antarmuka RS-485 umumnya digunakan untuk membantu mikrokontroller dalam melakukan komunikasi data secara serial. RS-485 menggunakan dua kabel untuk mengirimkan sinyal data dan tidak memerlukan commond ground. Sistem penyaluran data ini sering disebut dengan system differensial atau balanced.
TCP (Transmission Control Protocol)
TCP (Transmission Control Protocol) merupakan protokol jaringan yang banyak digunakan dalam komunikasi data berbasis internet dan intranet. Dalam proyek ini, TCP Serve digunakan sebagai antarmuka penerima data dari sensor yang dibaca oleh Modbus client. Rust, sebagai bahasa pemrograman sistem modern, menawarkan performa tinggi dan keamanan memori tanpa garbage collector.
InfluxDB sebagai Database Time-Series
InfluxDB adalah basis data time-series yang dirancang khusus untuk menangani data yang bersifat waktu (timestamp-based) seperti suhu, kelembaban, tekanan, dan data IoT lainnya. InfluxDB unggul dalam pencatatan dan kueri data berdasarkan waktu, serta memiliki struktur penyimpanan yang efisien dan query language yang sederhana (InfluxQL atau Flux). Dalam sistem ini, data yang diterima oleh TCP Server akan disimpan ke dalam InfluxDB dengan metadata waktu pencatatan. Penyimpanan ini penting untuk analisis tren, monitoring historis, serta integrasi dengan sistem visualisasi data real-time
Visualisasi Real-Time Menggunakan Grafana
Grafana adalah platform open-source yang digunakan untuk visualisasi data dalam bentuk grafik interaktif, tabel, dan alert. Grafana mendukung berbagai sumber data, termasuk InfluxDB, dan dapat diakses melalui antarmuka web. Dalam proyek ini, Grafana digunakan untuk menampilkan data suhu dan kelembaban dari gudang fermentasi secara real-time.
