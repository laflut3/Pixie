# Application architecture

Pixie is a static HTTP server written in Rust. It loads YAML configuration, listens on a TCP address, maps routes to HTML files, and uses a worker pool to handle connections.

## Entry point

`src/main.rs` loads the runtime configuration and starts the server. Run locally from the repository root with `PIXIE_CONFIG=infrastructure/config/config-pixie.yml cargo run --locked`.

## Main modules

### Configuration

`src/config.rs` loads configuration from `PIXIE_CONFIG`, then `./config-pixie.yml`, then `/etc/pixie/config-pixie.yml`. It supports `addr`, `host`, `port`, `workers`, and aliases `address` and `nb_worker`, with built-in defaults when no file is available.

### Server and router

`src/server.rs` binds the configured TCP address, accepts connections, and writes HTTP responses. `src/router.rs` resolves the web root using `PIXIE_WEB_ROOT` and maps `/` to `hello.html`; missing pages use `404.html` when available.

### Worker pool and logs

`src/threadpool/` contains the message queue and worker lifecycle. `src/logger.rs` provides standardized `info`, `warn`, and `error` output.

## Request flow

1. A client opens a TCP connection.
2. The server submits the connection to the worker pool.
3. The handler reads the HTTP request line.
4. The router selects an HTML file.
5. The server returns the status and page content.

## Tests and coverage

Tests are in `tests/`. Run them from the repository root with `cargo test --locked`. Coverage can be collected with LLVM instrumentation:

```sh
cargo clean
RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='pixie-%p-%m.profraw' cargo test
RUST_HOST="$(rustc -vV | sed -n 's/^host: //p')"
TOOLBIN="$(rustc --print sysroot)/lib/rustlib/${RUST_HOST}/bin"
"$TOOLBIN/llvm-profdata" merge -sparse pixie-*.profraw -o pixie.profdata
"$TOOLBIN/llvm-cov" report \
  --object target/debug/deps/config_tests-* \
  --object target/debug/deps/threadpool_tests-* \
  --object target/debug/deps/router_tests-* \
  --object target/debug/deps/server_tests-* \
  --object target/debug/deps/logger_tests-* \
  --instr-profile pixie.profdata \
  --ignore-filename-regex '/\\.cargo/registry|/tests/'
```
