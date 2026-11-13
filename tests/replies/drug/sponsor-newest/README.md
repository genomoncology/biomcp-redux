# Drugs@FDA sponsor search, newest first

The two replies were recorded unchanged from the public openFDA Drugs@FDA endpoint on 2026-11-13 by `biomcp --record` in pm job j38975, without a key. The publisher's dataset date was 2026-11-12. The recording wrappers hold each body as parsed JSON. The same requests, fetched again the same day with curl, returned bodies whose parsed JSON equals the recorded values exactly; the byte counts and SHA-256 below are of those original bodies.

| Request | Status | Bytes | SHA-256 of the original body |
| --- | --- | --- | --- |
| `https://api.fda.gov/drug/drugsfda.json?search=sponsor_name%3ANOVARTIS%2A&limit=5&sort=submissions.submission_status_date%3Adesc%2Capplication_number%3Aasc` | 200 | 64,252 | `2227c11e280fa8741fe74da7ea91ec0450add375424617bc80ac077964b40152` |
| `https://api.fda.gov/drug/drugsfda.json?search=sponsor_name%3ANOVARTIS%2A&limit=5&sort=submissions.submission_status_date%3Adesc%2Capplication_number%3Aasc&skip=5` | 200 | 67,586 | `49ad2bdc29d793de1abb2d2980d0583aaddef12391ab27b71ba1f4fd205779fe` |

Checked live the same day: the source sorts the many-valued `submissions.submission_status_date` by each application's largest value, accepts a second sort key, and puts the 19 of 204 applications with no submissions last. `application_number:asc` breaks the tie between BLA125504 and BLA761349, both last acted on 2026-10-15. All ten latest submissions on these two pages are supplements; the original approvals date from 1986-12-15 to 2025-09-30. NDA215358 lists two approved originals on 2021-10-29, and the row gives submission 1. The prefix `NOVARTIS` matches several sponsor names; the case makes no claim about the company family.
