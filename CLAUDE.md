# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Is

A Rust client library for interacting with Dwarf Fortress via the DFHack remote API. Communicates over TCP using a custom RPC protocol (not gRPC) with protobuf-serialized messages.

## Build & Test Commands

```bash
cargo build                          # Build all crates
cargo test                           # Run tests (no DF connection needed)
cargo test --features test-with-df   # Integration tests (requires running DF with DFHack)
cargo test <test_name>               # Run a single test
cargo clippy --workspace             # Lint
```

### Proto Sources

Proto files are checked in under `dfhack-proto-srcs`. Rust code is generated at build time into `$OUT_DIR`. To download fresh protos from a different DFHack version:

```bash
DFHACK_DOWNLOAD=1 cargo build                    # Download protos and rebuild
DFHACK_ZIP_URL=<url> DFHACK_DOWNLOAD=1 cargo build  # Target different DFHack version
```

### Integration Tests

Require `DF_EXE` env var pointing to a Dwarf Fortress executable and the `test-with-df` feature flag. Port defaults to 5000, overridable with `DFHACK_PORT`.

### Release

Uses `cargo-release`: `cargo release --workspace --execute` (or `cargo release [level] --workspace --execute`). All crates share a single version (`release.toml: shared-version = true`). Publishing happens via GitHub Actions on tag push, not `cargo release`.

## Architecture

Three-crate workspace with a layered code generation pipeline:

### `dfhack-proto-srcs` (proto source management)
- Downloads DFHack source zip and extracts `.proto` files to `src/protos/`
- Only runs when `DFHACK_DOWNLOAD=1` is set

### `dfhack-proto` (code generation)
- **Message generation**: prost-build compiles `.proto` files into Rust types
- **Stub generation**: Custom parser in `build.rs` reads RPC definitions from proto comments (`// Plugin: X`, `// RPC Method : Input -> Output`) and generates typed method stubs
- Generated code is produced into `$OUT_DIR` at build time
- Defines the `Channel` trait (transport abstraction) and `Message` trait

### `dfhack-remote` (main library)
- `channel.rs`: TCP client implementing `Channel` trait, method binding (lazy RPC ID assignment), request/response exchange
- `message.rs`: Wire protocol — handshake (`"DFHack?\n"` / `"DFHack!\n"`), headers (i16 method ID + i32 payload size), RPC reply codes
- `lib.rs`: `connect()` / `connect_to()` entry points, `Error` enum, `Reply<T>` wrapper (response + text fragments)
- Re-exports all generated types from `dfhack-proto` so consumers only need `dfhack-remote`

### Key types
- `Client` = `Stubs<Channel>` — the main API surface
- `Reply<T>` — wraps every RPC response with optional `CoreTextFragment`s
- Generated stubs apply syntactic sugar: `EmptyMessage` inputs become no-arg methods, `SingleBool`/`IntMessage`/`StringMessage` get unwrapped to primitives

### Wire protocol
- Handshake: 8-byte magic string + i32 version (1)
- Request: i16 method ID + i16 padding + i32 payload size + protobuf bytes
- Responses use special method IDs: Result(-1), Fail(-2), Text(-3), Quit(-4)
- Methods are bound dynamically at first call via `BindMethod` (id=0)
