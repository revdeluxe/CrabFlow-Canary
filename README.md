# CrabFlow Canary 🦀

CrabFlow Canary is an advanced SDN (Software Defined Networking) controller and Captive Portal solution built with Rust and Tauri. It turns any Windows device into a powerful network appliance capable of managing users, bandwidth, and security.

## Features 🚀

*   **Network Management**:
    *   **DHCP Server**: Built-in DHCP server with custom ranges, static leases, and captive portal integration.
    *   **DNS Server**: Custom DNS forwarder with support for blacklisting, local records, and split-interface routing (LAN vs WAN).
    *   **Upstream Interface Selection**: Explicitly route user traffic through a specific internet connection while serving clients on a local hotspot.
*   **Captive Portal**:
    *   **Force authentication** for new users.
    *   **Customizable portal pages** with a built-in editor.
    *   User bandwidth and session tracking.
*   **User Management**:
    *   Creating and managing users and groups.
    *   Role-based access control (RBAC).
*   **Monitoring**:
    *   Real-time system stats (CPU, Memory).
    *   Live network traffic graphs.
    *   DNS Query logs and System logs.
*   **Modern UI**:
    *   Built with SvelteKit and AdminLTE for a responsive, professional dashboard.
    *   Dark mode support.

## Technologies Used 🛠️

*   **Backend**:
    *   [Rust](https://www.rust-lang.org/) 🦀 - Core logic, Networking stack (DHCP/DNS/Sockets).
    *   [Tauri](https://tauri.app/) 🖼️ - Desktop application framework.
*   **Frontend**:
    *   [SvelteKit](https://kit.svelte.dev/) - UI Framework.
    *   [Chart.js](https://www.chartjs.org/) - Traffic visualization.

## Getting Started

1.  **Prerequisites**:
    *   Node.js & npm/pnpm
    *   Rust & Cargo

2.  **Installation**:
    ```bash
    git clone https://github.com/yourusername/CrabFlow-Canary.git
    cd CrabFlow-Canary/bin/crabflow
    npm install
    ```

3.  **Development**:
    ```bash
    cargo tauri dev
    ```

4.  **Building**:
    ```bash
    cargo tauri build
    ```

## Configuration

The application can be configured via the **Setup Wizard** on first run, or through the **Settings** page in the dashboard.
Configuration files are stored in `bin/crabflow/config/`.

## License

This project is licensed under the terms of the license specified in the `LICENSE` file.
