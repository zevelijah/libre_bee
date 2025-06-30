# Build script for supported OSes
# Install mingw64-gcc for building for Windows while on Linux, I know its avaiable with sudo dnf install mingw64-gcc for RHEL-based systems
# cargo install cargo-zigbuild for MacOS builds while on Linux or Windows (I think)
cargo build
cargo build --release
cargo zigbuild --target aarch64-apple-darwin 
cargo zigbuild --release --target aarch64-apple-darwin 
cargo zigbuild --target x86_64-apple-darwin 
cargo zigbuild --release --target x86_64-apple-darwin
cargo build --target x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu