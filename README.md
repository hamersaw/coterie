#coterie

##Overview
Distributed string fuzzy matching application.

##Compile
* protoc --rust_out=src/ protobuf/message.proto
* cargo build

##Run
- env RUST_LOG=debug ./target/debug/coterie examples/config.toml
- env RUST_LOG=debug ./target/debug/coterie examples/config2.toml
- env RUST_LOG=debug ./target/debug/coterie examples/config3.toml

##TODO
- build protobuf from build.rs
- add logging output to coterie and coterie-cli
- application work
