set -eux
cd "$(dirname $0)"

cargo +nightly fmt --all -- --check
cargo sort --grouped --workspace --check

# Workaround for code-gen bug
cargo check --quiet 2>/dev/null || :

cargo clippy --all-targets

cargo test

cargo build -p integration-tests-bindings
cargo run -p async-bindgen-gen-dart -- \
    --ffi-class IntegrationTestsFfi \
    --genesis integration_tests/lib/src/genesis.ffigen.dart

cd async_bindgen_dart_utils
dart pub get
dart analyze

cd ../integration_tests
dart pub get
dart analyze
dart test
