default: watch

defaultwatch:= "dev-loop-iter"

watch watchtarget=defaultwatch:
    cargo watch \
        --clear \
        --shell 'just {{watchtarget}}' \
        --ignore '**/*.svg' \
        --ignore 'lcov.info' \
        --ignore 'README.md' \
        --ignore 'mutants.out*/**'

reinit-workspace:
    cargo install cargo-watch --force
    cargo install cargo-tarpaulin --force
    cargo install cargo-doc --force
    cargo install cargo-mutants --force
    cargo install cargo-readme --force
    
dev-loop-iter:
    parallel --tty just quietly ::: \
        format \
        test \
        test-examples \
        build-diagrams \
        verify \
        update-coverage \
        docs

dev-loop-iter-mutants:
    just dev-loop-iter
    just mutants

dev-doc:
    just dev-loop-iter-fast
    just doc

test:
    CARGO_TERM_COLOR="always" \
    cargo test \
        --target-dir target/just-test \
        --workspace \
        -q

quietly recipe:
    @chronic unbuffer just {{recipe}}
    @echo -e "\033[0;32m{{recipe}} exited without error.\033[0m"

test-examples:
    CARGO_TERM_COLOR="always" \
    cargo test \
        --target-dir target/just-test-examples \
        --examples \
        -q

verify: verify-check verify-clippy

verify-check:
    CARGO_TERM_COLOR="always" \
    cargo check \
        --target-dir target/just-check \
        --workspace \
        --all-features

verify-clippy: 
    CARGO_TERM_COLOR="always" \
    cargo clippy \
        --workspace \
        --target-dir target/just-clippy \
        -- \
            --deny clippy::pedantic \
            --deny clippy::correctness \
            --deny clippy::style \
            --deny clippy::complexity

format:
    CARGO_TERM_COLOR="always" \
    cargo fmt

build-diagrams:
    make svg

update-coverage:
    CARGO_TERM_COLOR="always" \
    cargo tarpaulin \
        --target-dir target/just-tarpaulin \
        --out Lcov \
        --skip-clean

mutants:
    CARGO_TERM_COLOR="always" \
    cargo mutants \
        --no-times

docs:
    CARGO_TERM_COLOR="always" \
    RUSTFLAGS="-Dmissing_docs" \
    cargo +nightly doc \
        --features doc-images \
        --target-dir target/just-doc
    
