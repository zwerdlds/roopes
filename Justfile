default: watch

defaultwatch:= "dev-loop-iter"

watch watchtarget=defaultwatch:
    cargo watch \
        --clear \
        --shell 'just {{watchtarget}}' \
        --ignore '**/*.svg' \
        --ignore 'lcov.info' \
        --ignore 'wip/*' \
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
        verify \
        update-coverage

dev-loop-iter-mutants:
    just dev-loop-iter
    just mutants

dev-loop-doc:
    parallel --tty just quietly ::: \
        format \
        test \
        verify \
        update-coverage \
        doc

test:
    CARGO_TERM_COLOR="always" \
    cargo test \
        --target-dir target/just-test \
        --workspace \
        --quiet \
        --all-targets

quietly recipe:
    @chronic unbuffer just {{recipe}}
    @echo -e "\033[0;32m{{recipe}} exited without error.\033[0m"

verify: verify-check verify-clippy

verify-check:
    RUSTFLAGS="-D warnings" \
    CARGO_TERM_COLOR="always" \
    cargo check \
        --target-dir target/just-check \
        --workspace \
        --all-features

verify-clippy: 
    RUSTFLAGS="-D warnings" \
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
    just build-diagrams
    CARGO_TERM_COLOR="always" \
    RUSTFLAGS="-Dmissing_docs" \
    cargo +nightly doc \
        --features doc-images \
        --target-dir target/just-doc
    
