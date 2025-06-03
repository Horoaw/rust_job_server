# 🛠️ rust_job_server

A lightweight Rust web server for managing simple jobs using Actix Web and SQLite.  

---

## 🚀 Features

- 🔗 Connects to an SQLite database at `data/data.db`
- 🗂️ Automatically creates a `jobs` table if missing
- 📥 Add new jobs via HTTP POST requests
- 📋 List all jobs via HTTP GET requests
- ⚡ Fast and easy to extend for more complex task management

---

## 🔧 Setup Instructions

1. Create the data folder and empty database file:
  mkdir -p data

  touch data/data.db

3. Run the server:
  cargo run
4. Use your favorite HTTP client (curl, Postman, etc.) to interact:

- Add a job:

  ```
  curl -X POST http://localhost:8080/add_job/your_command_here
  ```

- List all jobs:

  ```
  curl http://localhost:8080/jobs
  ```

---

## 💡 Project Idea

This project provides a basic job management server where jobs are simple commands stored persistently in a local SQLite database. The web API allows adding jobs and retrieving the list of jobs.

It serves as a foundation to build:

- ✅ Persistent job queue management  
- 🕒 Job scheduling and execution  
- 📊 Job status tracking and reporting  
- 🔐 Secure access and authentication  
- 🌐 Distributed task processing  

---

## 📅 Future Work

- Implement job execution and monitoring  
- Add job statuses (pending, running, completed, failed)  
- Build a web dashboard for easy job management  
- Introduce user authentication and role-based access  
- Support more complex job definitions (e.g., scripts, parameters)  
- Enable job retries, cancellations, and priority handling  

---

Thank you for checking out the project! Feel free to contribute or suggest features. 🚀
