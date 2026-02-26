# Task Manager

A lightweight system monitor application built with Rust and [Iced](https://github.com/iced-rs/iced). Displays real-time CPU and memory usage with live-updating graphs.

## Features

- **Real-time CPU monitoring** — tracks global CPU usage percentage and frequency (GHz)
- **Real-time memory monitoring** — tracks memory usage percentage and total available memory
- **Live graphs** — plots CPU and memory usage over time with color-coded line charts
- **Thumbnail previews** — sidebar shows miniature graphs for quick at-a-glance status
- **System uptime** — displays how long the system has been running

## Screenshots

### CPU Monitoring
![CPU monitoring view](assets/screenshots/s1.png)

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)

## Building

```sh
cargo build
```

For an optimized release build:

```sh
cargo build --release
```

## Running

```sh
cargo run
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| [iced](https://github.com/iced-rs/iced) | Cross-platform GUI framework |
| [plotters-iced2](https://crates.io/crates/plotters-iced2) | Plotters backend for Iced |
| [plotters](https://crates.io/crates/plotters) | Plotting library for charts |
| [sysinfo](https://crates.io/crates/sysinfo) | Cross-platform system information (CPU, memory, uptime) |
| [image](https://crates.io/crates/image) | Image processing |

## Project Structure

```
src/
├── main.rs                 # Application entry point
├── lib.rs                  # Library root
└── taskmanager/
    ├── mod.rs              # Module declarations
    └── ui.rs               # UI layout, system data collection, and graph rendering
```

## License

This project is licensed under the GNU General Public License v3.0 — see the [LICENSE](LICENSE) file for details.
