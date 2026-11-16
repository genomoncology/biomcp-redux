# 0990 — orphadata-captures-become-synthetic

Status: OPEN.

Milestone: 7

## Outcome

The public repo holds no real Orphadata bytes. Every Orphadata case replays synthetic recordings, as the other licensed and noncommercial sources already do.

## Why

Ian, 2026-11-16 (`sdlc/decisions/2026-11-16-review-follow-ups.md`, item 1), on finding C3 of `sdlc/records/2026-11-14-adversarial-review.md`:

- `tests/replies/disease/orphadata/` holds real files from `www.orphadata.com` whose inline `Licence` element states Orphanet's terms;
- the CC BY 4.0 claim cites a different site's FAQ;
- a synthetic set already exists in `tests/replies/disease/orphadata-synthetic/`.

## Proof

- Proof: `tests/replies/disease/orphadata/` is gone (or holds only synthetic bytes), and no case or test reads real Orphadata bytes.
- Proof: the synthetic-captures check from 0920 and 0973 covers Orphadata, and it would fail if real Orphadata bytes came back.
- Proof: every case `tests/cases/disease/orphadata.yaml` covered is still covered, against the synthetic set.

## Starts from

- Starts from: both case files and both reply folders above; 0889; 0920 and 0973's check.

## Keeps

- Keeps: Orphanet's statement on Orphadata results (`sdlc/decisions/2026-11-15-source-terms-live-in-help-sources.md`, item 2). No change to the adapter.

## Defers

- Defers: rewriting git history. The real bytes stay in past commits until the history append (0294).

## Changes

- Changes: Orphadata cases and recordings, the synthetic-captures check, any README in the reply folders.

## Progress

- `tests/replies/disease/orphadata/` and `tests/cases/disease/orphadata.yaml` are deleted. The real-file case's steps now live in `tests/cases/disease/orphadata-synthetic.yaml`, which replays `disease/orphadata-synthetic` and keeps its own class-only and PubMed-form step.
- The synthetic files gain a second authored disorder at Orphanet code 2322, the code MONDO:0016512 links to, so the MONDO join is still tested against the unchanged live MyDisease reply. Its name ("Synthetic disorder at a MONDO-linked code"), estimates, geographies and PubMed IDs (90000001-90000003, above any PubMed has issued) are invented. It keeps every shape the real case asserted: four rows mixing birth and point prevalence, a `[EXPERT]_<pmid>[PMID]` source, three onset ages, two inheritance modes, the release date, the Orphanet notice and the MONDO link. The MyDisease replies for MONDO:0016512 and MONDO:0007254 moved unchanged into `orphadata-synthetic/mydisease/`.
- Sync now installs 4 rows (two disorders in each file), so the case and the `disease-orphadata/other-licence-keeps-set` runner trial expect 4.
- `tests/replies/disease/orphadata-synthetic/README.md` is new: it carries `Synthetic: orphadata` and says which parts are authored and which are live MyDisease.
- Check: `crates/biomcp/tests/ratchets/noncommercial_replies.rs` treats Orphadata as synthetic-only though its terms are attribution, citing the 2026-11-16 decision, so an Orphadata recording without the marker fails. It also fails on any Orphadata body that is a `JDBOR` document without `dbserver="synthetic"`: real files name Orphanet's database server, so real bytes fail even under the marker. Unit test `real_orphadata_replies_fail` plants an unmarked reply, a marked authored one and a marked real-looking one.
- Check job j41616 at 159acaf3b: both Orphadata trials (`disease-orphadata/disease/orphadata-synthetic.yaml` and `disease-orphadata/other-licence-keeps-set`) and every ratchet unit test, including `real_orphadata_replies_fail`, passed; `check-cheap` failed only on `cargo fmt` and the 160-character line limit in the new check, both fixed. No docs or schemas regenerated.
- Gate `make --keep-going landing-gate docs-check schema-release-check` passed in pm job j41624 at bef5a17fe (branch merged with origin/main ee909e969).
