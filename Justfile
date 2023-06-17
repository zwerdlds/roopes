watch:
    cargo watch \
        --why \
        --shell 'just dev-loop' \
        --ignore '*.svg' \
        --ignore 'lcov.info' \
        --ignore '/home/zwerdlds/Development/roopes/mutants.out*/**'

dev-loop:
    just dev-loop-inner

dev-loop-inner: 
    just build-diagrams &
    just test
    just test-examples
    just format
    just verify
    just update-coverage
    just mutants

test:
    cargo test --workspace -q

test-examples:
    cargo test --examples -q

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

update-coverage:
    cargo tarpaulin \
        --out Lcov \
        --skip-clean

mutants:
    cargo mutants --jobs 24