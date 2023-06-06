watch:
    cargo watch \
        --shell "just dev-loop" \
        --ignore '*.svg'

dev-loop:
    clear
    just dev-loop-inner

dev-loop-inner: build-diagrams format test run-doctest verify run-examples 

test:
    cargo test -q

run-examples:
    #!/bin/bash
    for example_path in ./examples/*
    do
        example_name="$(basename ${example_path})";
        echo "---";
        if [ -e "./examples/${example_name}" ] ;
        then
            cargo run --example "${example_name}";
        fi
    done

run-doctest:
    cargo test --doc -q

verify: verify-check verify-clippy

verify-check:
    cargo check --workspace --all-features

verify-clippy: 
    cargo clippy \
        --workspace \
        -- \
            --deny clippy::pedantic \
            --allow clippy::inline-always

format:
    cargo fmt

build-diagrams:
    make svg