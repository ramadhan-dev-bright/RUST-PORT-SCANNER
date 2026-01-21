# RUST-PORT-SCANNER
# ⚡ Neuron Port Scanner V20

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />
  <img src="https://img.shields.io/badge/Tokio-Asynchronous-blue?style=for-the-badge" />
  <img src="https://img.shields.io/badge/Status-Stable-green?style=for-the-badge" />
</p>

---

### 🚀 Overview
**Neuron Port Scanner** adalah tool pemindaian port TCP yang dibangun menggunakan **Rust**. Fokus utama project ini adalah kecepatan ekstrem melalui *parallel asynchronous tasks* dan akurasi tinggi untuk menghindari *false positives*.

### 🛠️ Technical Specs
* **Runtime**: [Tokio](https://tokio.rs/) (Multi-threaded)
* **Concurrency**: 100 parallel scans.
* **Accuracy**: Menggunakan TCP handshake validation.
* **UI**: Terminal output berwarna menggunakan crate `colored`.

### 📦 Installation & Usage


```bash
