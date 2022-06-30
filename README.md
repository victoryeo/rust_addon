This project is about using Rust with NodeJS. 
We use NodeJS Node-API(N-API) to build native addon. We implement the native addon using Rust.

The steps to create, build, execute the project:
cargo init --lib
cargo build --release
cp target/release/librust_addon.so index.node
node index.js

To build the native addon, llvm ad clang are required:
sudo apt install llvm-10
cd /usr/bin
sudo ln -s llvm-config-10 llvm-config
sudo apt install libclang-dev
