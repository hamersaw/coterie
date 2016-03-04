#coterie

##Overview
Distributed string fuzzy matching application.

##Compile
protoc --rust_out=src/ protobuf/message.proto
cargo build

##TODO
- build protobuf from build.rs
- dht work, start with join messages
