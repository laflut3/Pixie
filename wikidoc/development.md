# Development

Run Pixie from the repository root with the sample configuration:

```sh
PIXIE_CONFIG=infrastructure/config/config-pixie.yml cargo run --locked
```

The sample server listens on `0.0.0.0:8080`. Pages are stored in `web/`; `/` serves `hello.html`, and a missing or unsupported route uses `404.html`.

## Tests

Run the test suite with:

```sh
cargo test --locked
```

Tests are in `tests/` and cover configuration, routing, the worker pool, logging, and the server.

## Coverage

Collect LLVM coverage from the repository root:

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
