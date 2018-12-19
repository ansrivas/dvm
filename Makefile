.DEFAULT_GOAL := help
help:             ## Show available options with this Makefile
	@grep -F -h "##" $(MAKEFILE_LIST) | grep -v grep | awk 'BEGIN { FS = ":.*?##" }; { printf "%-18s  %s\n", $$1,$$2 }'

IS_SCCACHE :=$(shell type -P sccache)
PROJECT_BASE="dvm-rs"
PROJECT_NAME=$(shell echo $(PROJECT_BASE) | sed  's/-/_/g')

.PHONY : test
test:             ## Run all the tests
	cargo test --all -- --nocapture

.PHONY : test_cover

test_cover:             ## Run all the tests with coverage, this also tries to install cargo-tarpaulin
	@echo "Installing cargo-tarpaulin and running tests and coverage"
	@RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo install cargo-tarpaulin ; \
	cargo +nightly tarpaulin --no-count

.PHONY : lint
lint:             ## Run clippy as linter
	cargo clippy
    #cargo test --all && cargo clippy --all && cargo fmt --all -- --check

.PHONY : build_release
build_release:  ## Create a release build out of this project
	@cargo build --release

.PHONY : build
build:  ## Create a build out of this project
	@cargo build

.PHONY : clean
clean:         ## Clean the application
	@cargo clean

.PHONY: build_no_debug_sym
build_no_debug_sym: clean test ## Create a release build but without debug symbols = smaller size of binary.
	@RUSTFLAGS='-C link-args=-s' cargo build --release

.PHONY: docs
docs:  ## Generate the docs for this project. Docs are located in target/doc/{{app_with_under_score}}
	@cargo doc --no-deps

.PHONY: dev_test
dev_test: ## Run this using development run to execute tests
	@RUST_TEST_THREADS=1 cargo watch  -x check -x test --watch `pwd`/src --watch `pwd`/tests

.PHONY: dev_run
dev_run: ## Run this using development run to execute the app ( simply cargo run, when a file changes )
ifdef IS_SCCACHE
	@echo "************** SCCACHE is present in the path. Will use cached binaries for compilation. **********"
	@RUSTC_WRAPPER=$(HOME)/.cargo/bin/sccache cargo watch  -x run --watch `pwd`/src
else
	@echo "************** SCCACHE is not found in the path. Will use standard cargo-compilation **********"
	@cargo watch  -x run --watch `pwd`/src
endif

.PHONY: docs-open
docs-open: docs ## Generate docs and open with xdg-open
	@xdg-open target/doc/$(PROJECT_NAME)/index.html
