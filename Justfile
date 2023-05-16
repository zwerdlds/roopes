watch:
    cargo watch -s "just dev-loop"

dev-loop:
    clear
    just test
    just run-doctest
    just run-demo

test:
    cargo test -q

run-demo:
    cargo run --example roopes-demo-bin

run-doctest:
    cargo test --doc -q