# Drugs@FDA applicant search

The response bodies were captured unchanged from the public openFDA Drugs@FDA endpoint on 2026-11-11 with `scripts/source_capture.py`, without a key. The recording wrappers hold each body as parsed JSON, or as text for the absence reply.

| Request | Status | Bytes | SHA-256 of the original body |
| --- | --- | --- | --- |
| `https://api.fda.gov/drug/drugsfda.json?search=sponsor_name%3A%22ALNYLAM%20PHARMS%20INC%22&limit=2` | 200 | 14,550 | `0963807748ac1b6d5a6e6698d85e73defec04a9fbd4c686db79fe3cb551c0868` |
| `https://api.fda.gov/drug/drugsfda.json?search=sponsor_name%3A%22ALNYLAM%20PHARMS%20INC%22&limit=2&skip=2` | 200 | 11,526 | `a591704151e55ee833d571a62f190596e05f8a7cb4eac5483761254c3dc7f182` |
| `https://api.fda.gov/drug/drugsfda.json?search=sponsor_name%3A%22NO%20SUCH%20SPONSOR%22&limit=10` | 404 | 80 | `57b1e7534d003e4246182162fd2469cdf038de39405056f44fc006715e5496da` |

The source matches `sponsor_name` as one exact, case-sensitive value. On the capture date `sponsor_name:"Alnylam"` and `sponsor_name:"ALNYLAM PHARMS"` both answered `NOT_FOUND`. The case asserts the sponsor's four applications across two pages and the empty answer for an unknown sponsor. It makes no claim about the sponsor's corporate family.
