# SETUP
# Build for Windows

See https://mq.agical.se/release-desktop.html for additional build targets

## Build using Windows GNU target

Before running the build the for first time you need to install the build target. You will only have to run this command once.

rustup target add x86_64-pc-windows-gnu

To build the game, use the following command:

cargo build --release --target x86_64-pc-windows-gnu

The binary file created will be stored in the directory target/x86_64-pc-windows-gnu/release/.

## Build using Windows MSVC target
Before running the build the for first time you need to install the build target. You will only have to run this command once.

rustup target add x86_64-pc-windows-msvc

To build the game, use the following command:

cargo build --release --target x86_64-pc-windows-msvc

The binary file created will be stored in the directory target/x86_64-pc-windows-msvc/release/.

# Build for the Web
Source: [Build for the web - Game development in Rust with Macroquad](https://mq.agical.se/release-web.html)
Publish via GitHub pages: [Your first Macroquad app - Game development in Rust with Macroquad](https://mq.agical.se/ch1-first-program.html#publish-on-the-web-if-you-want)


## Install WASM build target

Start by installing the build target for WebAssembly using the command rustup.

rustup target add wasm32-unknown-unknown

## Build a WebAssembly binary

Using the WebAssembly target you can build a WASM binary file that can be loaded from a web page.

cargo build --release --target wasm32-unknown-unknown

The WASM binary file will be placed in the directory target/wasm32-unknown-unknown/release/ with the extension .wasm.

## Copy WebAssembly binary

You need to copy the WebAssembly binary to the root of your crate, in the same place where the assets directory is placed.

If you have named your crate something else than my-game, the name of the binary will have the same name, but with the file e

cp target/wasm32-unknown-unknown/release/my-game.wasm .

## Create an HTML page

You will need an HTML page to load the WebAssembly binary. It needs to load a Javascript file from Macroquad which contains code to run the WebAssembly binary and communicate with the browser. You also need to add a canvas element that Macroquad will use to draw the graphics. Remember to change the name of the WebAssembly binary file in the load() call from my-game.wasm to the name of your game if you have changed it.

Create a file with the name index.html in the root of your crate with the content found in the source.

## Install a simple web server

This is only to be able to test the game locally before you upload it to a proper web hosting account. To serve your game locally on your computer you can install a simple web server with the following command:

cargo install basic-http-server

## Run the web server

This command will start the web server and print an address where you can reach the web page. Open your web browser and load the URL, this will be something similar to http://localhost:4000. The game should now run in your browser instead of as a native application.

basic-http-server .

## Publish your game

If you have access to a web hosting account, you can publish the files there to let other people play your game. You need to upload the HTML file, the WASM file, and the assets directory.

index.html
my-game.wasm
assets/*


