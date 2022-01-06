set -eux
cd "$(dirname $0)"

cargo +nightly fmt --all -- --check
cargo sort --grouped --workspace --check

cargo clippy --all

cargo test

cargo build -p integration-tests-bindings
cargo run -p async-bindgen-gen-dart -- \
    --ffi-class IntegrationTestsFfi \
    --genesis integration_tests/lib/src/genesis.ffigen.dart
cd integration_tests
dart test
