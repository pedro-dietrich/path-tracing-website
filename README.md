# Path Tracing Graduation Project Website

## Description

This is a simple website made to showcase my graduation project, named **Path Tracing: Performance Analysis on Ray Tracing Dedicated Hardware**.


## Requirements

- Rust toolchain (`rustup`, `cargo` and `rustc`)
- WebAssembly target
  - Run the command: `rustup target add wasm32-unknown-unknown`
- Trunk
  - Run the command: `cargo install --locked trunk`


## Build and Run

### Backend

Run the command `cargo run -p backend` to compile and execute the project.


### Frontend

To build and run the frontend, use the following commands:

```bash
cd frontend
trunk build
trunk watch
```
