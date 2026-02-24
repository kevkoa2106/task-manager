# Task Manager

A lightweight system monitor application built with Rust and [egui](https://github.com/emilk/egui). Displays real-time CPU and memory usage with live-updating graphs.

## Features

- **Real-time CPU monitoring** — tracks global CPU usage percentage and frequency (GHz)
- **Real-time memory monitoring** — tracks memory usage percentage and total available memory
- **Live graphs** — plots CPU and memory usage over time with color-coded line charts
- **Thumbnail previews** — sidebar shows miniature graphs for quick at-a-glance status
- **System uptime** — displays how long the system has been running

## Screenshots

*Coming soon*

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)
- A working GPU driver for egui/wgpu rendering

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
| [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) | Application framework for egui |
| [egui_extras](https://github.com/emilk/egui/tree/master/crates/egui_extras) | Extra widgets and image loading |
| [egui_plot](https://github.com/emilk/egui_plot) | Plotting widgets for line charts |
| [sysinfo](https://crates.io/crates/sysinfo) | Cross-platform system information (CPU, memory, uptime) |

## Project Structure

```
src/
├── main.rs                 # Application entry point and window setup
└── taskmanager/
    ├── mod.rs              # Module declarations
    └── ui.rs               # UI layout, system data collection, and graph rendering
```

## License

This project is licensed under the GNU General Public License v3.0 — see the [LICENSE](LICENSE) file for details.
