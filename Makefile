.DEFAULT_GOAL := landing-gate

.PHONY: landing-gate fmt lint ratchets test audit coverage unreached

landing-gate: fmt lint coverage audit

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

coverage:
	sh scripts/coverage.sh run

unreached:
	sh scripts/coverage.sh unreached
