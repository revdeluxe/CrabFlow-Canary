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

### Important runtime notes

- Captive Portal HTTP server: the built-in HTTP API / captive portal listens on port **3030** on all interfaces (0.0.0.0:3030). If you previously accessed the dev UI on port 1420 you may still do so locally, but captive-portal clients are redirected to port 3030.
- DHCP: the bundled DHCP server binds to port 67 and will not hand out the configured gateway or the server bind address. If you enable DHCP ensure the `range_start`/`range_end` in Setup cover free addresses in your LAN (for example `10.0.0.2` - `10.0.0.254`).
- Interface names: on Windows the app resolves friendly adapter names (e.g. "Ethernet - Realtek PCIe ...") by querying PowerShell `Get-NetAdapter` and preserves the original internal id (GUID) as the option value. On Linux/macOS the kernel names (eth0, en0, etc.) are used.
- Permissions: some networking operations (binding to low ports, manipulating iptables or stopping services) require elevated privileges. On Windows run the app as Administrator; on Linux run as root.

### ACL / Captive Portal notes

- Saving ACL config: Use Admin → ACL & Permissions → Save. The frontend sends the ACL object directly to the backend `save_acl_config` command which persists `acl_config.json` into the app config directory.
- Captive portal redirect rules: on Linux the app may apply `iptables` NAT rules to redirect HTTP (port 80) traffic to the portal port (3030). On Windows the app uses other techniques; you may need to configure port forwarding or allow the portal port through the local firewall.

If you encounter problems saving configuration, check the application logs (Dashboard → Monitor → Logs) for error messages about file permissions or serialization errors.

## License

This project is licensed under the terms of the license specified in the `LICENSE` file.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue if you have any feedback or suggestions.
