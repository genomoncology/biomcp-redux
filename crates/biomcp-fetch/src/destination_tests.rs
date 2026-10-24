//! Security tables exercise DNS classifications without a public network dependency.
use super::{public, redirect};

#[test]
fn public_dns_answers_exclude_reserved_and_transition_addresses() {
    for value in [
        "0.0.0.0",
        "0.1.2.3",
        "10.0.0.1",
        "100.64.0.1",
        "100.127.255.255",
        "127.0.0.1",
        "169.254.169.254",
        "172.16.0.1",
        "172.31.255.255",
        "192.0.0.1",
        "192.0.2.1",
        "192.88.99.1",
        "192.168.1.1",
        "198.18.0.1",
        "198.19.255.255",
        "198.51.100.1",
        "203.0.113.1",
        "224.0.0.1",
        "255.255.255.255",
        "::",
        "::1",
        "fc00::1",
        "fd00::1",
        "fe80::1",
        "ff02::1",
        "::ffff:127.0.0.1",
        "64:ff9b::a00:1",
        "2001::1",
        "2001:1ff::1",
        "2001:db8::1",
        "2002:a00:1::1",
        "3fff::1",
    ] {
        assert!(!public(value.parse().unwrap()), "accepted {value}");
    }
    for value in [
        "8.8.8.8",
        "1.1.1.1",
        "100.63.255.255",
        "100.128.0.0",
        "172.15.255.255",
        "172.32.0.0",
        "2001:4860:4860::8888",
        "2606:4700:4700::1111",
    ] {
        assert!(public(value.parse().unwrap()), "refused public {value}");
    }
}
#[test]
fn redirects_recheck_credentials_scheme_and_literal_addresses() {
    for target in [
        "http://127.0.0.1/private",
        "//[::1]/private",
        "http://2130706433/private",
        "https://user:password@example.org/file",
        "file:///etc/passwd",
        "https://example.org/file#fragment",
    ] {
        assert!(
            redirect("https://example.org/article/file.pdf", target).is_err(),
            "accepted {target}"
        );
    }
    assert_eq!(
        redirect("https://example.org/article/file.pdf", "../copy.pdf").unwrap(),
        "https://example.org/copy.pdf"
    );
}
