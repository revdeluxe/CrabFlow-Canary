# CrabFlow Canary 🦀

CrabFlow Canary is an advanced SDN (Software Defined Networking) controller and Captive Portal solution built with Rust and Tauri. It turns any Windows device into a powerful network appliance capable of managing users, bandwidth, and security.

## Project Status

This project is currently under active development and should be considered **alpha** software. APIs and features may change frequently.

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
    *   [Tauri](https://tauri.app/) `v2.0.0-beta` 🖼️ - Desktop application framework.
*   **Frontend**:
    *   [SvelteKit](https://kit.svelte.dev/) `(next)` - UI Framework.
    *   [Chart.js](https://www.chartjs.org/) `v4.4.3` - Traffic visualization.
    *   [AdminLTE](https://adminlte.io/) `v4.0.0` - Dashboard template.
    *   [Bootstrap](https://getbootstrap.com/) `v5.3.3` - CSS framework.

## Prerequisites

Before you begin, ensure you have the following tools installed:

*   **Node.js & pnpm**: [https://nodejs.org/](https://nodejs.org/)
*   **Rust & Cargo**: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

## Installation

1.  Clone the repository:
    ```bash
    git clone https://github.com/yourusername/CrabFlow-Canary.git
    ```
2.  Navigate to the application directory:
    ```bash
    cd CrabFlow-Canary/bin/crabflow
    ```
3.  Install the dependencies:
    ```bash
    npm install
    ```

## Development

To run the application in development mode, use the following command:

```bash
cargo tauri dev
```

## Building for Production

To build the application for production, use the following command:

```bash
cargo tauri build
```

The compiled application will be located in `bin/crabflow/src-tauri/target/release`.

## Configuration

The application can be configured via the **Setup Wizard** on first run, or through the **Settings** page in the dashboard.

Configuration files are stored in `bin/crabflow/config/`.

## License

This project is licensed under the terms of the license specified in the `LICENSE` file.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue if you have any feedback or suggestions.
