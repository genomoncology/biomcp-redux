# 0143 — List SEER sites and read incidence and survival by site

Status: OPEN.

Milestone: 3

Depends on: 0139

## Outcome

`search disease --source seer` returns the SEER Explorer site catalog with each native site code, exact label and active flag and the catalog version or date, paged with `--limit` and `--offset`. `get disease 47 incidence --source seer` and `get disease 55 survival --source seer` return the structured native rows for that site, paged with `--section-limit` and `--section-offset`. Incidence takes `--sex`, `--race`, `--age-range`, `--dataset` and `--period`. Survival takes `--sex`, `--race`, `--age-range`, `--dataset` and `--interval`, and has no `--period`. Each page echoes every requested selector beside the returned selectors, period and native values, lists the unverified dimensions, and names the catalog as its next action.

- Why: D19 and D20 ask incidence and survival for an exact SEER site, and the 0117 PM amendments put SEER inside search and get with the site in the ID position and incidence and survival as named sections.
- Starts from: disease.md's SEER contract, the structured SEER repair and fresh clinical ruling, the SEER rows of the case map, and the 0117 recheck amendments 1 and 2; the retained 73-site catalog and the two structured bodies from the repair job; 0138's statistics record and 0139's registry.
- Rules: the caller selects the site from the catalog; nothing maps an ontology label or a name table to a site, and a get with an ontology CURIE fails before I/O. The adapter decodes the one JSON-string wrapper and validates the structured object, and never parses HTML. Incidence stage rows stay stage-stratified rates. Survival returns the annual series, and years with only modeled values are labeled model-only with observed values absent, never zero. Nothing averages annual rows or supplies the recent five-year aggregate, which the PM dropped. A returned selector that contradicts the request adds a problem. Dataset, standard population, delay adjustment and registry membership stay unverified unless the reply states them. Paging a retained reply makes no extra provider request.
- Proof: outside-in cases first, watched to fail. Before adapter code, one bounded keyless pm job captures the site catalog with a second page, the site 47 incidence request with each incidence selector, and the site 55 survival request with each survival selector and no period. Cases D19-sites-first/next, D20-sites-selection, D19-incidence, D20-survival, D19-both-sexes/all-races/all-ages/year5, D20-female/interval5/native-annual-period31, D19-stage-page/terminal and D20-annual-page/model-only replay the recorded replies; a survival `--period` case fails before I/O. Recorded 429, 500, timeout and broken-body cases assert a failed outcome with source and terms. Each case has a size ceiling from its measured output, starting from the design's allocations of 950 bytes for the site page, 2,650 for incidence and 3,400 for survival. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
