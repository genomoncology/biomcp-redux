# 0362 — Make reply output consistent

Status: COMPLETE.
Landed: 8d566e27defb5d96aa2c94371205d6a156505bdc

Milestone: 5

## Outcome

Empty lists are left out of JSON, as the JSON output decision says, including gene `previous_symbols`. Variant ID lists use one form: every entry carries its namespace prefix, such as `rs:113488022` or `clinvar:13961`, or the ticket cites an existing ruling for the bare rsID and follows it everywhere. Trial `result.request` no longer echoes raw ClinicalTrials.gov parameters. `source_updated` carries a timezone or says the source gave none. The query echo leaves out defaults the user did not set, such as `has_results: false`.

- Why: the docs lead's product findings report (2026-10-27), item D5. Ian's ruling that 1.0 ships every known fix (2026-10-27).
- Starts from: `gene/model.rs`, the variant ID builders, the trial reply, the `source_updated` type and query echo serialization.
- Rules: the query echo still shows every value that changed the request.
- Proof: a check over all recorded replies fails on an empty list; cases for each change; the baseline is regenerated if a schema changes.
- Changes: the named types, serializers, cases and the baseline.
- Defers: flattening deep result paths, which waits for the next schema major because it would rename most paths.
