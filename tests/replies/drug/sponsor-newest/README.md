# Drugs@FDA sponsor search, newest first

The two replies were recorded unchanged from the public openFDA Drugs@FDA endpoint on 2026-11-13 by `biomcp --record` in pm job j38975, without a key. The publisher's dataset date was 2026-11-12.

| Request | Status |
| --- | --- |
| `https://api.fda.gov/drug/drugsfda.json?search=sponsor_name%3ANOVARTIS%2A&limit=5&sort=submissions.submission_status_date%3Adesc%2Capplication_number%3Aasc` | 200 |
| `https://api.fda.gov/drug/drugsfda.json?search=sponsor_name%3ANOVARTIS%2A&limit=5&sort=submissions.submission_status_date%3Adesc%2Capplication_number%3Aasc&skip=5` | 200 |

Checked live the same day: the source sorts the many-valued `submissions.submission_status_date` by each application's largest value, accepts a second sort key, and puts the 19 of 204 applications with no submissions last. `application_number:asc` breaks the tie between BLA125504 and BLA761349, both last acted on 2026-10-15. The prefix `NOVARTIS` matches several sponsor names; the case makes no claim about the company family.
