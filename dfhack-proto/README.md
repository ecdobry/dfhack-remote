# dfhack_proto

[![Crates.io](https://img.shields.io/crates/v/dfhack-proto)](https://crates.io/crates/dfhack-proto)
[![docs.rs](https://img.shields.io/docsrs/dfhack-proto)](https://docs.rs/dfhack-proto)
![Crates.io](https://img.shields.io/crates/l/dfhack-proto)

This subcomponent of [dfhack_remote](https://docs.rs/dfhack-remote/) contains all the generated code
for interacting with DFHack remote API.

It contains two main modules:

 - [messages] exposes the protobuf messages. This is the standard generated protobuf.
 - [stubs] exposes the plugins and their RPC. DFHack is not using gRPC and this is a custom implementation

The terminology is based on `gRPC`: The `stubs` expose the feature from the plugin. They are built from
a `channel` implementing the data exchange.

Both modules are generated at build time from the proto files in `dfhack-proto-srcs`.
