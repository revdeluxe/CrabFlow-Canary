# CrabFlow Canary Technical Documentation

Welcome to the technical documentation for CrabFlow Canary. This documentation is intended for developers who want to contribute to the project or understand its inner workings.

## Table of Contents
- [Overview](#overview)
- [Backend Documentation](./backend/README.md)
- [Frontend Documentation](./frontend/README.md)

## Overview

CrabFlow Canary is an SDN (Software Defined Networking) controller and Captive Portal solution built with Rust and Tauri. It turns any Windows device into a powerful network appliance capable of managing users, bandwidth, and security.

This documentation is divided into two main sections:

*   **[Backend Documentation](./backend/README.md)**: Information about the Rust-based backend, including the networking stack, API, and system modules.
*   **[Frontend Documentation](./frontend/README.md)**: Information about the SvelteKit-based frontend, including the UI components, stores, and routes.

## Quick Operational Notes

This section lists a few important operational details developers and testers should know when running CrabFlow Canary locally or on a gateway machine.

- Captive Portal server: the backend HTTP API and captive portal pages run on `0.0.0.0:3030`. Captive-portal detection endpoints (for Windows/macOS/Android) are exposed under that server, and most devices will be redirected to `/captive` or `/login` when the portal is active.

- DHCP server: runs on UDP port 67 and serves leases to clients from the configured `range_start`..`range_end`. The server avoids handing out the configured gateway and the server bind address to clients. Leases are persisted to `bin/crabflow/config/leases.json`.

- DNS & Ad-blocking: the DNS forwarder honors the ACL blacklist and can optionally return 0.0.0.0 for blocked entries. Upstream DNS binding uses the configured upstream interface.

- Interface naming: on Windows the app resolves friendly adapter names via PowerShell (`Get-NetAdapter`) and presents those to the UI while preserving the original adapter identifier (GUID) for internal use. On Linux/macOS the OS interface names (eth0, enp3s0, en0) are used directly.

- Elevated privileges: networking features such as binding to low ports (53, 67) and adding firewall/NAT rules require the app to run with elevated permissions (Administrator on Windows, root on Unix-like systems). Some platform-specific setup (stop ICS on Windows, enable ip_forward on Linux) is performed at startup when possible.

See the backend and frontend documentation for API details and developer instructions.
