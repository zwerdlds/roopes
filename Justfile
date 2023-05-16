watch:
    cargo watch -s "just dev-loop"

dev-loop:
    clear
    just dev-loop-inner

dev-loop-inner: build-diagrams test run-doctest verify run-demo

test:
    cargo test -q

run-demo:
    cargo run --example roopes-demo-bin

run-doctest:
    cargo test --doc -q

verify: verify-check verify-clippy verify-fmt

verify-check:
	cargo check --workspace --all-features

verify-clippy: 
	cargo +nightly clippy --workspace --no-default-features --release -- --deny warnings
	cargo +nightly clippy --workspace --all-features --release -- --deny warnings

verify-fmt:
    cargo fmt -- --check

build-diagrams:
    make svg