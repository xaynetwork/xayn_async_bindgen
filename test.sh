set -eu
cd "$(dirname $0)"

cargo +nightly fmt --all -- --check
cargo sort --grouped --workspace --check

cargo clippy --all

cargo test

cargo build -p integration-tests-bindings
cd integration_tests
dart test
