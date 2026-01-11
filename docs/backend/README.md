# Backend Documentation

The backend of CrabFlow Canary is built with Rust and Tauri. It is responsible for the core logic of the application, including the networking stack, API, and system modules.

## Table of Contents
- [Architecture](#architecture)
- [Modules](#modules)

## Architecture

The backend is a multi-threaded application that uses the following key components:

*   **Tauri**: The main application framework, which provides the webview for the frontend and the bridge for communication between the frontend and backend.
*   **Axum**: A web framework for building the API that is used by the frontend to interact with the backend.
*   **SurrealDB**: A multi-model database that is used to store the application's data, including users, devices, and settings.
*   **Tokio**: An asynchronous runtime for Rust, which is used to power the networking stack and other I/O-bound operations.

## Modules

The backend is divided into the following modules:

*   **`commands`**: Tauri commands that can be invoked from the frontend.
*   **`http_server`**: The Axum-based HTTP server that provides the API.
*   **`network`**: The networking stack, which includes the DHCP and DNS servers, as well as the captive portal.
*   **`setup`**: The setup wizard that is used to configure the application on first run.
*   **`sysmodules`**: System modules that are responsible for tasks such as logging, database management, and power management.
*   **`user_management`**: User management and authentication.
