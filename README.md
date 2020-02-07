# Project Structure

The project consists of three Crates organized in a common workspace.

1. Web Server: wasm-svg-ws, based on Actix Web
2. Library: wasm-svg-lib, wrapper around WEB API based on web-sys
3. Examples: wasm-svg-examples, example code using the library, compile target: wasm

## Web Server

#### How to Compile & Run

In the project root run

`cargo build -p wasm-svg-ws`

`cargo run -p wasm-svg-ws`

The web server binds on localhost port 8080.


## Library

#### How to Compile

In the project root run 

`cargo build -p wasm-svg-lib`


## Examples

#### How to Compile

Change to the Crate root of the example you are interested in and run 

```
cd examples/simple
examples/simple$ wasm-pack build --target web
```
