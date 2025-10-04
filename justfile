default:
    just --list

# Run all lints and tests.
test-all:
    cargo vet --locked
    cargo +stable deny --all-features --locked check
    cargo +stable fmt -- --check
    cargo +stable doc --all-features
    cargo +stable build --locked
    cargo +stable build --locked --all-features
    cargo +stable clippy --locked --all-targets --all-features
    cargo +stable test --locked --all-features
