.DEFAULT_GOAL := landing-gate

# release/notices.py needs tomllib (Python 3.11+), and the build machine's
# python3 is 3.9. Run every Python step on the newest Python 3.11+ found, or on
# PYTHON=/path/to/python when given. When python3 itself is older, put a folder
# holding only a python3 link to the chosen Python first on PATH, so scripts that
# call python3 get it and no other tool on PATH changes.
PYTHON_FOUND := $(shell for p in $(if $(PYTHON),$(PYTHON),python3.13 python3.12 python3.11 python3); do \
	$$p -c 'import sys; sys.version_info >= (3, 11) and print(sys.executable)' 2>/dev/null \
	| grep . && break; done)
ifeq ($(PYTHON_FOUND),)
$(error no Python 3.11 or newer found$(if $(PYTHON), at PYTHON=$(PYTHON)); install one or run make PYTHON=/path/to/python3.11+)
endif
override PYTHON := $(PYTHON_FOUND)
export PYTHON
ifneq ($(shell python3 -c 'import sys; print(sys.version_info >= (3, 11))' 2>/dev/null),True)
PYTHON_SHIM := $(CURDIR)/target/python-shim
$(shell mkdir -p "$(PYTHON_SHIM)" && ln -sf "$(PYTHON)" "$(PYTHON_SHIM)/python3")
export PATH := $(PYTHON_SHIM):$(PATH)
endif
BIOMCP_DEBUG = $(if $(CARGO_TARGET_DIR),$(CARGO_TARGET_DIR),target)/debug/biomcp

# Cargo and the test harnesses do not interpret pm's fractional core booking.
ifneq ($(strip $(PM_JOB_CORES)),)
PM_WORKERS := $(shell $(PYTHON) -c 'import math,os; print(max(1, math.floor(float(os.environ["PM_JOB_CORES"]))))')
ifeq ($(PM_WORKERS),)
$(error PM_JOB_CORES must be a finite number)
endif
export CARGO_BUILD_JOBS ?= $(PM_WORKERS)
export RUST_TEST_THREADS ?= $(PM_WORKERS)
endif
TEST_THREADS = $(if $(strip $(RUST_TEST_THREADS)),-- --test-threads $(RUST_TEST_THREADS))

.PHONY: landing-gate full-suite plain-cli-build fmt lint ratchets test audit coverage unreached

landing-gate:
	+$(PYTHON) scripts/landing_gate.py

full-suite:
	+$(PYTHON) scripts/landing_gate.py --full

# Keep these sequential even when the caller uses make -j.
.PHONY: pre-job check-cheap proof gate-stage-check
pre-job: check-cheap

LINT_GOALS = unused-dependencies check-no-default-features lint-workspace lint-tools lint-rustdoc

check-cheap:
	$(PYTHON) scripts/check_docs.py
	+$(MAKE) --no-print-directory --keep-going -j1 shell-check fmt $(LINT_GOALS) ratchets

.PHONY: shell-check
shell-check:
	$(PYTHON) scripts/check_shell.py

proof: pre-job
	$(PYTHON) scripts/trials.py --target tools --proofs $(PROOFS)

gate-stage-check:
	$(PYTHON) -m unittest scripts.test_gate_stages

plain-cli-build:
	cargo build --locked -p biomcp --bin biomcp --timings

fmt:
	cargo fmt --all -- --check

.PHONY: $(LINT_GOALS)
lint:
	+$(MAKE) --no-print-directory --keep-going -j1 $(LINT_GOALS)

check-no-default-features:
	cargo check --keep-going --locked -p biodata -p biomcp-types --no-default-features

lint-workspace:
	cargo clippy --keep-going --workspace --all-targets --locked -- -D warnings

lint-tools:
	cargo clippy --keep-going --locked -p biomcp --test tools -- -D warnings

lint-rustdoc:
	RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked --keep-going

ratchets: plain-cli-build
	cargo test --locked -p biomcp --test ratchets $(TEST_THREADS)

test:
	cargo test --workspace --locked $(TEST_THREADS)

audit:
	cargo deny --locked check licenses advisories bans sources

.PHONY: cases
cases:
	cargo test --locked -p biomcp --test cases $(TEST_THREADS)

.PHONY: rights
rights:
	$(PYTHON) scripts/article_licenses.py
	$(PYTHON) scripts/test_article_licenses.py

.PHONY: library-tests
library-tests:
	cargo test --workspace --exclude biomcp --features biomcp-types/schema --locked $(TEST_THREADS)
	cargo test --locked -p biomcp --bin biomcp $(TEST_THREADS)

# The landing gate's share of library-tests: only the crates the branch changed (0914).
.PHONY: changed-library-tests
changed-library-tests:
	$(PYTHON) scripts/changed_library_tests.py

coverage: schema-shape-check
	sh scripts/coverage.sh run

.PHONY: schema-shape-check
schema-shape-check: docs-check library-tests
	cargo test --locked -p biomcp --test cases mcp-research/research-registry-projection-paths $(TEST_THREADS)
	cargo test --locked -p biomcp --test cases mcp-research/research-search-fetch-parity $(TEST_THREADS)

unreached:
	sh scripts/coverage.sh unreached

# The landing gate runs this after pre-job builds the debug binary: documented
# example calls must pass their tool schemas, and the first read must hold.
.PHONY: first-read-check
first-read-check:
	$(PYTHON) scripts/check_docs.py --executable "$(BIOMCP_DEBUG)"
	$(PYTHON) scripts/check_front_map.py "$(BIOMCP_DEBUG)"

.PHONY: docs-check
docs-check:
	cargo test --locked -p biomcp --test cases readme/ $(TEST_THREADS)
	cargo test --locked -p biomcp --bin biomcp grammar::reference::reference_matches_grammar $(TEST_THREADS)
	$(PYTHON) release/toolchain.py check
	$(PYTHON) -m unittest release.test_toolchain
	$(PYTHON) release/tools.py
	$(PYTHON) -m unittest release.test_tools release.test_linux release.test_mutations
	+$(MAKE) --no-print-directory first-read-check
	+$(MAKE) --no-print-directory rights
	$(PYTHON) scripts/test_docs.py
	$(PYTHON) scripts/test_source_capture.py
	$(PYTHON) scripts/test_source_capture_plan.py
	$(PYTHON) scripts/test_coverage.py
	$(PYTHON) scripts/test_reply_diff.py
	$(PYTHON) scripts/test_schema_release.py
	+$(MAKE) --no-print-directory gate-stage-check
	$(PYTHON) scripts/test_build_workers.py
	$(PYTHON) scripts/test_trials.py
	$(PYTHON) scripts/test_cache_process_load.py
	$(PYTHON) scripts/test_nightly_live_check.py
	$(PYTHON) -m unittest release.test_installer release.test_checks release.test_gate release.test_windows release.test_artifacts.ReleaseCommands
	$(PYTHON) -m unittest release.test_workflow release.test_security release.test_licenses release.test_docs
	$(PYTHON) -m unittest release.test_source_revision release.test_workflow_contracts release.test_publication release.test_promotion
.PHONY: release-linux
release-linux:
	sh release/linux.sh $(CONTAINER_ARGS)

.PHONY: release-artifacts
release-artifacts:
	sh release/matrix.sh $(CONTAINER_ARGS)

.PHONY: nightly-live-check
nightly-live-check:
	$(PYTHON) scripts/nightly_live_check.py

.PHONY: source-capture
source-capture:
	$(PYTHON) scripts/source_capture.py --plan "$(PLAN)" --out "$(OUT)"

.PHONY: docs-generate
docs-generate:
	cargo test --locked -p biomcp --bin biomcp grammar::reference::write_reference $(if $(strip $(RUST_TEST_THREADS)),-- --test-threads $(RUST_TEST_THREADS) --ignored,-- --ignored)
.PHONY: schema-generate schema-release-check
schema-generate:
	$(PYTHON) scripts/schema_release.py --generate

schema-release-check:
	$(PYTHON) scripts/schema_release.py

.PHONY: tensor-proto-check
tensor-proto-check:
	$(PYTHON) scripts/regenerate_tensor_proto.py --check

.PHONY: secrets
secrets:
	$(PYTHON) release/secrets.py history . --since "$(SECRET_SCAN_BASE)"

# This ancestor was covered by the retained j24209 full-history scan.
# Advance only after the replacement ancestor has passed secret screening.
SECRET_SCAN_BASE ?= $(shell cat release/secrets-main)
.PHONY: secrets-full
secrets-full:
	$(PYTHON) release/secrets.py history .

COLD_REPORT ?= $(CURDIR)/dist/cold-report.json

.PHONY: release-cold
release-cold:
	$(PYTHON) release/rehearse.py cold --report "$(COLD_REPORT)" -- $(CONTAINER_ARGS)

.PHONY: release-mutations
release-mutations:
	@test -n "$(MUTATION_REPORT)" || (echo 'Set MUTATION_REPORT to a job-owned stage report'; exit 1)
	$(PYTHON) release/rehearse.py mutation --report "$(MUTATION_REPORT)" -- $(MUTATION_ARGS)

.PHONY: workflow-lint
workflow-lint:
	cargo check --locked -p biodata -p biomcp-types --no-default-features
	actionlint

.PHONY: unused-dependencies
unused-dependencies:
	$(PYTHON) release/toolchain.py install-machete
	cargo machete

# Long suite: real-time and multi-process checks repeated under load, run at final
# release and for touched checks: make stress STRESS_ARGS="--only fetch/retries".
.PHONY: stress
stress:
	mkdir -p target/stress
	cargo build --locked -p biomcp -p biomcp-fetch --bin biomcp --test cases --test tools --test cache --test retries --test transport --test quotas --test grpc_worker --test grpc_persistence --test bundle --message-format=json > target/stress/build.jsonl
	$(PYTHON) scripts/cache_process_load.py target/stress/build.jsonl $(STRESS_ARGS)

.PHONY: hosted-http-load
hosted-http-load:
	cargo test --locked -p biomcp --bin biomcp hosted_http_sync_load -- --ignored --nocapture

.PHONY: hosted-surface-check
hosted-surface-check:
	$(PYTHON) scripts/trials.py --installed mcp-hosted/served-surface-and-source-policy mcp-parity/typed-hosted-catalog
