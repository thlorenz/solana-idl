DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

CARGO_TEST=nextest run
CARGO_TEST_NOCAP=nextest run --nocapture
$(if $(shell command -v cargo-nextest 2> /dev/null),,$(eval CARGO_TEST=test))
$(if $(shell command -v cargo-nextest 2> /dev/null),,$(eval CARGO_TEST_NOCAP=test -- --nocapture))

test:
	cargo $(CARGO_TEST)

fmt:
	cargo +nightly fmt -- --config-path rustfmt-nightly.toml

list:
	@LC_ALL=C $(MAKE) -pRrq -f $(firstword $(MAKEFILE_LIST)) : 2>/dev/null | awk -v RS= -F: '/(^|\n)# Files(\n|$$)/,/(^|\n)# Finished Make data base/ {if ($$1 !~ "^[#.]") {print $$1}}' | sort | egrep -v -e '^[^[:alnum:]]' -e '^$@$$'

ci-test:
	cargo $(CARGO_TEST_NOCAP)

ci-fmt:
	cargo +nightly fmt --check -- --config-path rustfmt-nightly.toml

ci-clippy:
	cargo clippy --all-targets -- -D warnings -A unexpected_cfgs

.PHONY: test list ci-test ci-fmt ci-lint
