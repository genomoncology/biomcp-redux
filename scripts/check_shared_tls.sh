#!/bin/sh
set -eu
case "${1:-focused}" in
cheap) make fmt lint ratchets ;;
focused)
    cargo test --locked -p biomcp-fetch --lib ten_hosted_calls_build_one_tls_configuration
    cargo test --locked -p biomcp-fetch --test shared_tls
    cargo test --locked -p biomcp-fetch --test transport
    cargo test --locked -p biomcp-fetch --test grpc_worker
    ;;
*) exit 2 ;;
esac
