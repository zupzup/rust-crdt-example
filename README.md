# rust-crdt-example

Example for a simple real-time, collaborative web app using CRDTs in Rust

## Setup Clients

In the `client` folder, use `trunk serve --open` to start the app locally on http://127.0.0.1:8080. You can open it in multiple browser tabs to have multiple clients


## Setup Server

In the `server` folder, use `RUST_LOG=info cargo run` to start the websocket server on http://127.0.0.1:3000.
