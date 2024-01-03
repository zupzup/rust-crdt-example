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
        * Implement CRDT logic on server - LWWValue in a 3x3 Vec, or a LWWVec

## Setup

use `trunk serve --open` to start the app locally on http://127.0.0.1:8080.


