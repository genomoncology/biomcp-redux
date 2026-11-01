#!/bin/sh
set -eu
case "${1:-focused}" in
red) cargo test --locked -p biomcp --test cases article-search/late-store-lock-keeps-fetched-items ;;
cheap) make fmt lint ratchets ;;
focused)
 cargo test --locked -p biomcp --test cases article-search/late-store-lock-keeps-fetched-items
 cargo test --locked -p biomcp --test cases cache-resilience/
 cargo test --locked -p biomcp --test cases store-clean/
 ;;
*) exit 2 ;;
esac
