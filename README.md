# rust-crdt-example

Example for a real-time, collaborative web app using CRDTs in Rust

## Tech Stack

* Axum
* Leptos
* TODOs
    * Frontend
        * Create basic UI
        * Connect to websockets
            * https://leptos-use.rs/network/use_websocket.html
        * Show connected clients
        * Create Text field
            * submit var text = $('.editable').html();
        * Connect text-field to websocket API to query and update the text field with changes
        * Implement CRDT logic on the client
        * use Leptosfmt https://github.com/bram209/leptosfmt with rust analyzer
    * Backend
        * Create Axum Webserver
        * Add Websockets endpoint
        * Add list of connected clients and way to query that
        * Add in-memory state on the server for the text field
        * Implement basic communication on websocket, for clients to manipulate the text field state
        * Implement CRDT logic on server

## Setup

use `trunk serve --open` to start the app locally on http://127.0.0.1:8080.


