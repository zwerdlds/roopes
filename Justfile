watchtarget := "parallel-iter"


default:
    @echo "No default command configured."
    @just --list


watch watchtarget=watchtarget:
    cargo watch \
        --clear \
        --shell 'just {{watchtarget}}' \
        --ignore '**/*.svg' \
        --ignore 'lcov.info' \
        --ignore 'wip/*' \
        --ignore 'README.md' \
        --ignore 'mutants.out*/**'

parallel-iter:
    @parallel just quietly -- \
        format \
        test \
        verify \
        update-coverage \
        docs \
        mutants

setup-cargo-plugins:
    cargo install cargo-watch --force
    cargo install cargo-tarpaulin --force
    cargo install cargo-doc --force
    cargo install cargo-mutants --force
    cargo install cargo-readme --force

test:
    RUST_BACKTRACE=1 \
    CARGO_TERM_COLOR="always" \
    cargo test \
        --target-dir target/just-test \
        --workspace \
        --quiet \
        --all-targets && \
    cargo test \
        --target-dir target/just-test-docs \
        --workspace \
        --quiet \
        --doc

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
        --target-dir target/just-clippy

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

publish-crates:
    cargo publish -p "roopes-core"
    cargo publish -p "roopes-derive"
    cargo publish -p "roopes"

