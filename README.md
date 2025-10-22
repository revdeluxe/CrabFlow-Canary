# CrabFlow Canary 🦀

CrabFlow Canary is a network monitoring and management tool built with Rust, Rocket, and Tauri. It provides a web-based UI to monitor system stats, network traffic, and manage DHCP leases.

## Features 🚀

*   **Dashboard**: A comprehensive dashboard that displays:
    *   System information (OS, CPU, Memory) 💻.
    *   Live network traffic graph with per-interface selection and adjustable update interval 📈.
    *   System logs 📝.
*   **Device Manager**: A page to manage network devices, starting with a display of active DHCP leases ⚙️.
*   **Settings Page**: A page to configure the application settings ⚙️.
*   **Theme Toggle**: Switch between light and dark themes 🎨.

## Technologies Used 🛠️

*   **Backend**:
    *   [Rust](https://www.rust-lang.org/) 🦀
    *   [Rocket](https://rocket.rs/) (web framework) 🚀
    *   [Tera](https://keats.github.io/tera/) (templating engine) 📝
    *   [sysinfo](https://crates.io/crates/sysinfo) (for system information) 💻
*   **Frontend**:
    *   HTML5, CSS3, JavaScript
    *   [Chart.js](https://www.chartjs.org/) (for the traffic graph) 📊
*   **Desktop App**:
    *   [Tauri](https://tauri.app/) 🖼️

## How to Run

1.  Make sure you have Rust and the Tauri prerequisites installed.
2.  Clone the repository.
3.  Run the application in development mode:
    ```bash
    cargo tauri dev
    ```
4.  Open your browser and navigate to `http://localhost:8000` (or the configured port).

## License

This project is licensed under the terms of the license specified in the `LICENSE` file.
