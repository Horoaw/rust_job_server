# 🚀 rust_job_server

A high-performance asynchronous task scheduling server written in Rust. This project lays the groundwork for a decentralized computing cluster where GPU resources from multiple servers can be dynamically allocated to client-side tasks — without requiring SSH access.

---

## 🧠 Vision

🌍 **Make remote computing as easy as visiting a website!**

This project aims to build a decentralized GPU task execution network where:

- 🖥️ Clients can offload heavy tasks to remote servers without SSH
- 📡 Servers form a distributed, decentralized compute network
- 🔄 Tasks are automatically scheduled and executed on available resources
- ⚙️ Ideal for AI inference, training jobs, and general heavy workloads

Eventually, we want:

- ⚡ Lightweight client app for submitting jobs
- 🧠 Intelligent, decentralized task dispatcher (like a GPU DAO)
- 📈 Scalable, multi-server compute pooling
- 🔐 Secure, efficient, encrypted communication

---

## ✅ Features Completed

| Module           | Description                                      |
|------------------|--------------------------------------------------|
| 🧱 Backend Setup | Actix-web based asynchronous server               |
| 🗃️ SQLite DB     | Manages job queue and execution metadata         |
| 📤 Job Submit API| `/add_job/{command}` for submitting shell tasks  |
| 🔁 Async Execution | Jobs are run in background threads with updates |
| 🔐 TLS Support   | HTTPS via `rustls` encryption                     |

---

## 🛠️ How to Use

### 1️⃣ Run the Server

```bash
  cargo run
```bash



