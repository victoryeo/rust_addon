This project is about using Rust with NodeJS. 
1. We use NodeJS Node-API(N-API) to build native addon. We implement the native addon using Rust.

2. We use NodeJS FFI(Foreign function interface) to build ffi addon. We implement the ffi addon using Rust. (It seems FFI is obsolete, the last commit was in Jan, 2019)

The steps to create, build, execute the project:
- For native project:
cargo init --lib
cargo build --release
cp target/release/libnative.so index.node
node index.js
- For ffi project:
cargo build --release
cp target/release/libffi.so index.node
node index.js

To build the native addon, llvm ad clang are required:
sudo apt install llvm-10
cd /usr/bin; sudo ln -s llvm-config-10 llvm-config
sudo apt install libclang-dev
