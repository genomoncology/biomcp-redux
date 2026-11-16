# Two days of one saved trial search

These replies prove `saved changed` (ticket 0982): one ClinicalTrials.gov search, `search trial --condition "uveal melanoma" --phase 3`, read on two days.

`day1/` is a real recording. The actual CLI recorded it from the public source with `saved changed --record` in job j41517 on 2026-11-16: the version endpoint, the first page (22 matching studies), the two continuation pages its page tokens name, and the one-row probe the search sends. The bodies are unchanged.

`day2/` is an authored variant, not a second day of the registry. The job recorded the same search a second time a few seconds later, and the registry answered byte for byte the same studies, so two real days could not differ in the time a job runs. The second recording was then edited by hand, keeping every request URL and page token, so the replay asks the same pages:

- Removed (gone): NCT00036816 from the first page and NCT00811200 from the second page.
- Added (new): NCT06626516, whose row was copied unchanged from a real recording of `search trial --condition "uveal melanoma" --status RECRUITING` made in the same job. It is the only row of the last page.
- Updated (changed): NCT07804186's overall status moved from RECRUITING to ACTIVE_NOT_RECRUITING (in the first page and in both answers of the one-row probe), and NCT05502900's last update date moved from 2025-05-15 to 2026-11-14.
- The first page and the probe state a total of 21 instead of 22. The rows were shifted forward so pages hold 10, 10 and 1 studies.

The hand count is in `hand-count.json`: 1 new, 2 gone, 2 changed, and the totals 22 and 21. The runner trial `saved-work/changed-two-recorded-days` checks that the report equals it.

ClinicalTrials.gov data are public registry data subject to the publisher's attribution and source-date terms. These cases make no clinical eligibility claim.
