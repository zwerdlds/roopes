defaultwatch:= "dev-loop-fast"

watch watchtarget=defaultwatch:
    cargo watch \
        --why \
        --shell 'just {{watchtarget}}' \
        --ignore '*.svg' \
        --ignore 'lcov.info' \
        --ignore '/home/zwerdlds/Development/roopes/mutants.out*/**'


dev-loop:
    clear
    just dev-loop-fast
    echo "format \
        build-diagrams \
        verify \
        update-coverage \
        doc" | \
    parallel --tty just silently {}
    just silently mutants
    @echo -e "\033[0;32m .-------------------------------.\033[0m"
    @echo -e "\033[0;32m | Dev Loop Exited Without Error |\033[0m"
    @echo -e "\033[0;32m .-------------------------------.\033[0m"



dev-loop-fast:
    @just silently format
    @just silently test

dev-doc:
    @just dev-loop-fast
    @just doc

test:
    CARGO_TERM_COLOR="always" \
    cargo test \
        --target-dir target/tests \
        --workspace -q

silently recipe:
    chronic just {{recipe}}
    @echo -e "\033[0;32m{{recipe}} Exited Without Error.\033[0m"

test-examples:
    CARGO_TERM_COLOR="always" \
    cargo test \
        --target-dir target/test-examples \
        --examples -q

verify: verify-check verify-clippy

verify-check:
    CARGO_TERM_COLOR="always" \
    cargo check \
        --target-dir target/check \
        --workspace \
        --all-features

verify-clippy: 
    CARGO_TERM_COLOR="always" \
    cargo clippy \
        --workspace \
        --target-dir target/clippy \
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
        --target-dir target/tarpaulin \
        --out Lcov \
        --skip-clean

mutants:
    CARGO_TERM_COLOR="always" \
    cargo mutants \
        --no-times

doc:
    CARGO_TERM_COLOR="always" \
    RUSTFLAGS="-Dmissing_docs" \
    cargo doc \
        --target-dir target/doc \
