.DEFAULT_GOAL := landing-gate

.PHONY: landing-gate fmt lint ratchets test audit coverage unreached

landing-gate: docs-check fmt lint coverage audit

fmt:
	cargo fmt --all -- --check

lint:
	cargo clippy --workspace --all-targets --locked -- -D warnings

ratchets:
	cargo test --locked -p biomcp --test ratchets

test:
	cargo test --workspace --locked

audit:
	cargo deny --locked check licenses advisories

coverage: ratchets
	sh scripts/coverage.sh run

unreached:
	sh scripts/coverage.sh unreached

.PHONY: docs-check
docs-check:
	cargo test --locked -p biomcp --test cases article-help/executable-examples
	python3 release/tools.py
	python3 -m unittest release.test_tools
	python3 scripts/check_docs.py
	python3 scripts/test_docs.py
	python3 scripts/test_source_capture.py
	python3 scripts/test_source_capture_plan.py
	python3 scripts/test_coverage.py
	python3 scripts/test_nightly_live_check.py
	python3 -m unittest release.test_installer release.test_checks
.PHONY: release-artifacts
release-artifacts:
	sh release/matrix.sh $(CONTAINER_ARGS)

.PHONY: nightly-live-check
nightly-live-check:
	python3 scripts/nightly_live_check.py

.PHONY: source-capture
source-capture:
	python3 scripts/source_capture.py --plan "$(PLAN)" --out "$(OUT)"
