.DEFAULT_GOAL := landing-gate

# Cargo and the test harnesses do not interpret pm's fractional core booking.
ifneq ($(strip $(PM_JOB_CORES)),)
PM_WORKERS := $(shell python3 -c 'import math,os; print(max(1, math.floor(float(os.environ["PM_JOB_CORES"]))))')
ifeq ($(PM_WORKERS),)
$(error PM_JOB_CORES must be a finite number)
endif
export CARGO_BUILD_JOBS ?= $(PM_WORKERS)
export RUST_TEST_THREADS ?= $(PM_WORKERS)
endif
TEST_THREADS = $(if $(strip $(RUST_TEST_THREADS)),-- --test-threads $(RUST_TEST_THREADS))

.PHONY: landing-gate plain-cli-build fmt lint ratchets test audit coverage unreached

landing-gate: workflow-lint plain-cli-build fmt lint docs-check tensor-proto-check coverage audit secrets

plain-cli-build:
	cargo build --locked -p biomcp --bin biomcp --timings

fmt:
	cargo fmt --all -- --check

lint:
	cargo check --locked -p biodata -p biomcp-types --no-default-features
	cargo clippy --workspace --all-targets --locked -- -D warnings

ratchets:
	cargo test --locked -p biomcp --test ratchets $(TEST_THREADS)

test:
	cargo test --workspace --locked $(TEST_THREADS)

audit:
	cargo deny --locked check licenses advisories bans sources

coverage:
	sh scripts/coverage.sh run

unreached:
	sh scripts/coverage.sh unreached

.PHONY: docs-check
docs-check:
	cargo test --locked -p biomcp --bin biomcp grammar::reference::reference_matches_grammar $(TEST_THREADS)
	cargo test --locked -p biomcp --test cases article-help/executable-examples $(TEST_THREADS)
	python3 release/toolchain.py check
	python3 -m unittest release.test_toolchain
	python3 release/tools.py
	python3 -m unittest release.test_tools release.test_linux
	python3 scripts/check_docs.py
	python3 scripts/test_docs.py
	python3 scripts/test_source_capture.py
	python3 scripts/test_source_capture_plan.py
	python3 scripts/test_coverage.py
	python3 scripts/test_build_workers.py
	python3 scripts/test_nightly_live_check.py
	python3 -m unittest release.test_installer release.test_checks release.test_gate release.test_windows
	python3 -m unittest release.test_workflow release.test_security release.test_licenses
	python3 -m unittest release.test_source_revision release.test_workflow_contracts
.PHONY: release-linux
release-linux:
	sh release/linux.sh $(CONTAINER_ARGS)

.PHONY: release-artifacts
release-artifacts:
	sh release/matrix.sh $(CONTAINER_ARGS)

.PHONY: nightly-live-check
nightly-live-check:
	python3 scripts/nightly_live_check.py

.PHONY: source-capture
source-capture:
	python3 scripts/source_capture.py --plan "$(PLAN)" --out "$(OUT)"

.PHONY: docs-generate
docs-generate:
	cargo test --locked -p biomcp --bin biomcp grammar::reference::write_reference $(if $(strip $(RUST_TEST_THREADS)),-- --test-threads $(RUST_TEST_THREADS) --ignored,-- --ignored)
.PHONY: schema-release-check
schema-release-check:
	python3 scripts/schema_release.py

.PHONY: parity-run
parity-run:
	sh scripts/parity_runner.sh "$(abspath $(PLAN))" "$(BASELINE)" "$(abspath $(OUT))"

.PHONY: parity-inventory
parity-inventory:
	cargo test --locked -p biomcp --test cases -- --parity-inventory "$(abspath $(PLAN))" "$(CURDIR)/tests/cases" "$(abspath $(OUT))"

.PHONY: tensor-proto-check
tensor-proto-check:
	python3 scripts/regenerate_tensor_proto.py --check

.PHONY: secrets
secrets:
	python3 release/secrets.py history .

COLD_REPORT ?= $(CURDIR)/dist/cold-report.json

.PHONY: release-cold
release-cold:
	python3 release/rehearse.py cold --report "$(COLD_REPORT)" -- $(CONTAINER_ARGS)

.PHONY: workflow-lint
workflow-lint:
	cargo check --locked -p biodata -p biomcp-types --no-default-features
	actionlint
