# rust-crdt-example

Example for a real-time, collaborative web app using CRDTs in Rust

## Tech Stack

* TODOs
    * Frontend
        * Create Text field
            * submit var text = $('.editable').html();
        * Connect text-field to websocket API to query and update the text field with changes
        * Implement CRDT logic on the client
    * Backend
        * Add in-memory state on the server for the text field
        * Implement basic communication on websocket, for clients to manipulate the text field state
        * Implement CRDT logic on server

## Setup

use `trunk serve --open` to start the app locally on http://127.0.0.1:8080.


