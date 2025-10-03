default:
    just --list

# Run all lints and tests.
test-all:
    cargo +stable fmt -- --check
    cargo +stable doc
    cargo +stable build --locked
    cargo +stable clippy --locked --all-targets
    cargo +stable test --locked
