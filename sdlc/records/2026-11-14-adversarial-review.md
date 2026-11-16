# Adversarial review, 2026-11-14: code, code quality and the project

Full-project adversarial review at Ian's request, run after the persona-testing wave: six independent read-only review lanes over a pinned snapshot of main (586c602a5, 2026-11-14), each finding verified by the coordinating reviewer, then weighed by a fresh agent against the project's stances and the prior findings register (records/2026-11-10-product-review.md). Nothing was built, run or pushed into the product; every claim is static reading. The weighed register follows; the six lane reports are appended after it with their full evidence (file, line, quote). Internal machine names are generalized in this copy.

# Weighed register — adversarial review of BioMCP @ 586c602a (pinned 2026-11-14)

Weighed by an independent agent that wrote neither the review nor the plan, per `skills/adversarial-project-review/references/weighing.md`. Snapshot: `the pinned snapshot` = origin/main `586c602a5`, read-only; no builds, tests or network. Claims were confirmed against the snapshot code: every item the parent had not verified was re-read by the weigher (findings marked **[w]** below were confirmed by my own read this session; the rest were confirmed by the parent's direct read, folded in per instruction). The PM's declined-ideas register (`pm idea list --closed --repo …`) was read; two entries bear directly (E4, D3) and are cited.

"Holds" vocabulary: confirmed / already fixed / partly true / wrong / unconfirmed. No finding was already fixed; none is unconfirmed. "Before the cut" means before the 1.0 release cut — the release is parked and persona testing is the current priority, so nothing here blocks today's persona work.

## Weighed findings

| ID | Finding (short) | Lane label | Holds | Stance | Fix cost | What it buys | Recommendation |
|---|---|---|---|---|---|---|---|
| A1 | NCI trial continuation advances by admitted rows; duplicates valid rows after any refusal, all-refused page strands the rest | likely defect, med | confirmed | F4 precedent (0796): same defect class the team already accepted for ctgov | ~15–25 lines (adapter `received` + token math) + two-page refusal fixture; no schema change | Prevents silently duplicated trials and unreachable pages in paged NCI results whenever NCI refuses a malformed row; any persona paging NCI trials meets it; frequency set by NCI malformation (rare, real) | **Accept, before the cut** — the sibling owner the accepted F4 fix missed |
| A2 | Drugs@FDA / openFDA-label name walks have no row cap (MyChem's is 250); slow-then-Failed on very common names | question, low-med | confirmed **[w]** (fda closure loops to `page.total`; label `search_names` reads every page; only MyChem capped) | None; the FDA ranking promise (NDA/BLA before ANDA, earliest approval) may make whole-or-fail deliberate — no comment/ticket records the choice | Decision fix near-zero (comment + release-note line); MyChem-style cap + Partial is small | Disclosure: a user searching a household ingredient gets a documented whole-or-fail (or a disclosed partial) instead of a surprise Failed after a long walk; no wrong data today | **Accept cheaper fix**: record the deliberate whole-or-fail decision (comment + notes line) before the cut; the code cap may follow after |
| A3 | PGx annotation continuations fall back to admitted `returned` (no native consumed) | risk, low | confirmed as latent code shape **[w]** (`steps`→`steps_with(None)`; annotations owner uses `steps`; whether CPIC annotation rows can refuse mid-page unproven) | Same invariant as A1/F4 | ~10 lines, thread the count or assert local slicing | Hardens the native-consumption invariant; real-use reach not shown to exist today | **Accept cheaper fix**: fold into A1's ticket as the same class; prove or close the refusal path there |
| A4 | `DrugNameResolution.total` says "Total exact source records" but reports examined candidates (≤250 when capped) | inconsistency, low | confirmed **[w]** (response.rs:340 wording vs `candidates.len()`; adjacent notice discloses the cap) | No-backwards-compat: schema description bump is allowed, no old readers | One description line + schema N+1 bump; minutes | Honest field semantics for schema consumers reading the field without the notice | **Accept, before the cut** while schemas still move weekly |
| A5 | `get drug … markets` drops the successful Drugs@FDA application leg's provenance; `add_source` dedupe can drop a second leg's notices | risk, low | confirmed **[w]** (success branch never `add_source`s `read`; dedupe keys `(source, state)`) | None | ~10 lines | Per-leg retrieval metadata for operators; no wrong clinical row | **Defer past the cut** — provenance-only |
| A6 | Trial paging assigns (not ORs) `outcome.dropped`; a future earlier flag would be silently cleared | risk, low | confirmed | Refusal-invariant the release claims | One character | Cheap hardening of the refusal invariant | **Accept, before the cut** (trivial) |
| B1 | Recording caps (50 MiB binary / 16 MiB text) far below GTR's real parts (384 MiB declared, ~184 MB gzip); `--record` on real `sync gtr` fails after the full download; team hand-cuts fixtures | design gap, med | confirmed | Nov-10 direction: review evidence is the standing workflow; 0954/0963 line | Small code (spool large sidecars or raise record-mode bundle-part cap) + one over-cap test | Lets the evidence tool capture the source that most needs evidence; today every GTR evidence refresh wastes a 184 MB download and fails | **Accept, before the cut** — it produces the release's own evidence |
| B2 | Single-bundle `StreamedXml` capped at 128 MiB while the format declares up to 512 MiB; only set parts honor the format cap | inconsistency, low | confirmed, latent **[w]** (`stream(R::MAX_BYTES)`, `validate_range` vs `MAX_BUNDLE_BYTES`; no current single-bundle StreamedXml) | None | Few lines + unit test | Prevents a download/parse disagreement the day a streamed single bundle is declared | **Defer past the cut** — latent; fold into the next bundle-format change |
| B3 | Replay reads recording JSON via plain `File::open` (no symlink check) though sidecars refuse symlinked ancestors | risk, low | confirmed **[w]** (`read()` at record.rs:508) | Not a trust boundary (local fixtures, recorded identity must still match) | Few lines + symlinked-fixture test | Provenance hygiene on local fixtures | **Defer past the cut**; adjacent to ticketed 0963 |
| B4 | Old-format cache migrations retained (ALTER TABLE v1/v2; migratable_versions; missing-predecessor tolerance) despite "no readers or migrations for old formats" | inconsistency, low | confirmed **[w]** | No-backwards-compat (2026-11-01) — plain reading conflicts; the decision's scope line says "the CLI and MCP", so an exemption ruling is available | Decision route: one line, nearly free; deletion route: small + tests | Either removes stance-contradicting code or makes the stance file truthful about local on-disk state | **Accept cheaper fix**: PM records the one-line exemption (local cache upgrades in place) before the cut; deletion optional later |
| B5 | Recording sidecars carry no content hash; a corrupted `.bin` replays silently as the captured body | risk, low | confirmed **[w]** (`Body::Sidecar(String)`, length-check only) | `deny_unknown_fields` + no-compat: optional hash field is addable | Small + one test | Test-evidence integrity; silent fixture corruption fails or passes wrongly | **Defer past the cut** — test infrastructure |
| B6 | Multi-part set syncs take no claim/lease; two concurrent `sync gtr` both download ~245 MB | design gap, low | confirmed **[w]** (`sync_set` has no claim; single-bundle path fences via `bundle_sync`) | None | Small if the claim row is reused; zero if documented | Prevents duplicate large downloads; readers never torn (atomic publication); syncs are explicit scheduled commands | **Defer past the cut**, or document the duplicate-download behavior in the sync docs |
| C1 | Hosted stdio has no per-caller use declaration: tier controls exist only on HTTP; noncommercial sources (ORCID, S2) serve with the operator's fallback credentials | likely defect, med (high if a stdio hosted core is ever shared) | confirmed | 2026-11-04 hosted-source-modes: "Hosting will use restricted … whatever header or key a caller sends"; the same decision's "local CLI and stdio keep every configured source" is the ambiguity to settle | (a) refuse `--hosted` without `--http`: ~10 lines; (b) `--use` ceiling + docs: ~30 lines | Keeps the hosted promise on every hosted transport; real reach today near zero if the wrapper always speaks HTTP | **Accept cheaper fix (a) before the cut**, after confirming the wrapper's transport; if Ian wants hosted stdio supported, take (b). Ian question below |
| C2 | Plain-HTTP `article pdf`/`trial document` downloads the whole document, then refuses the save (hosted refuses up front) | inconsistency, low | confirmed **[w]** (http block refuses Sync/StoreClean/fresh only; required-save refusal is hosted-only; `PdfSaveRefused` after download) | None | ~5 lines + one test | No wasted publisher download; plain HTTP consistent with hosted; no data leaves, no file written | **Accept, before the cut** (trivial) |
| C3 | Orphadata captures are real bytes from `www.orphadata.com` while the README's CC BY 4.0 claim cites `sciences.orphadata.com`'s FAQ; every other licensed capture set is `-synthetic` | question, low-med | confirmed | Stance 7 (captures from noncommercial/licensed sources are synthetic); 2026-11-04: license correctness must hold "before I start promoting this" | Investigation only, zero code; provenance note or swap to the existing synthetic set | Data-rights exposure in a repo that is public now; cheap to settle, expensive after a complaint | **Accept: settle now by owner investigation**; blocks promotion, not the cut. Ian question below |
| C4 | 429 containment comment says "never read" but the keyed body is read (for credential scan), then cleared | risk, low | confirmed **[w]** (comment at request.rs:364-366; `wire_body` reads when keyed) | None | One comment | Protects this week's NCBI rate-limit fix from a maintainer trusting the comment | **Accept, before the cut** (trivial) |
| D1 | Release notes name `UMLS_API_KEY`; the code has seven credential names, none UMLS | confirmed defect, med | confirmed | No-backwards-compat: the notes are the migration contract | One-line edit; no gate regeneration needed | A commercial user following the primary migration doc gets a credential that silently does nothing | **Accept, before the cut** — the notes freeze into the GitHub release at the tag |
| D2 | ~60 landed tickets absent from CHANGELOG/release notes (GTR specimens, ClinVar counts, FDA recall/PMA/CDx surface, HPO ancestors, PGx filter, Europe PMC/bioRxiv PDFs, `sync_timeout_seconds`, symlink-refusing folders) | design gap, med | confirmed | No-backwards-compat: notes list every removed/changed command, option, config key, output field | One writing pass; no code | The 0.9→1.0 contract is complete; migrating users see what changed | **Accept, before the cut** |
| D3 | Schema-contract comparison no-ops unless `BIOMCP_SCHEMA_BASELINE` is set; only rehearsal/release/tag run it | design gap, med | confirmed | Stance 5 (nothing runs first in the release). Distinct from declined idea "add schema-release-check before 0336": that decline rested on the then-stale baseline; D3 is about contract narrowing landing green through the routine gate | One Makefile stage (path-selected) + 1–2 min warm cargo on schema-touching tickets | A schema narrowing without a version bump or review entry fails at landing, not at the tag | **Accept, before the cut** — cheap while schemas move weekly |
| D4 | Nine `tests/tool-cases/` YAMLs are run by no gate stage; look like coverage, cannot fail (0765 precedent: a stale duplicate misled triage) | likely defect, low | confirmed | Stance 5 adjacent: dead checks are false trust | Small scripts/cases change; minutes of gate time | Removes false coverage that actively misleads during incidents | **Accept, before the cut** |
| D5 | `manifest.json` tool names validated only by full suite/release, never by the landing gate; changed-selection ignores the file | design gap, low | confirmed **[w]** (LANDING stages; PROOF_INPUTS lack manifest/plugin files) | None | Few lines in `check_docs.py` (already in pre-job); seconds | A tool rename missing the manifest fails at landing, not at release staging | **Accept** — before the cut preferred, after acceptable |
| D6 | Generated pages speak two vocabularies: clap help says "native"/"literal" with `help <command>` pointers; the details list below rewrites to "source"/"exact" and drops the pointer | inconsistency, low | confirmed **[w]** (commands.md:955-957 vs :1086; two generators) | 0913/0923 plain-wording line; Ian's ninth call (cosmetic capacity) | One `plain()` call at clap-help generation + `make docs-generate`; minutes | Terminal help stops contradicting the packaged reference beside it | **Accept, before the cut** (minutes; ninth call covers it) |
| D7 | Public `sdlc/` tickets name internal machines that the repo's own checker hashes as internal; `sdlc/` exempted by a code comment, no decision record | question, low | confirmed **[w]** (hashes recomputed: all four plaintexts match the checker's sets; `INTERNAL_SKIP` exempts sdlc/) | Public-repo hygiene rules; stance 7 keeps history as it is — after the history append (0294) the names are permanent | Decision line: zero; scrub pass: small, before the append | Stops baking internal machine names into public git history permanently | **Accept**: PM records the decision or scrubs before the history append. Ian question below |
| D8 | Changed-selection scripts disagree on the Makefile: a Makefile change runs every tool proof but no extra library test | risk, low | confirmed **[w]** (`WORKSPACE_FILES` differ between the two scripts) | None | One line + one expectation; seconds on Makefile changes only | Closes a silent-skip hole in Makefile test-wiring changes | **Accept, before the cut** (trivial) |
| E1 | Hosted reply cache has no byte ceiling; each distinct drug-name search pins a ~30 MiB label page for a day; hosted refuses `store clean` | design gap, med | confirmed | Hosted rollout parked (plan head); stance 3 (judge hosted on the real path) | Byte ceiling at write time + config knob, small | Prevents silent disk exhaustion on a hosted server that cannot self-clean; reached at hosted scale-out, not before | **Defer past the cut** with trigger: before hosted deployment/AWS publication resumes |
| E2 | Concurrent hosted label calls hold several whole copies of 32 MiB pages; ~0.7–1 GiB transient at capacity 8 | risk, med | confirmed as static arithmetic **[w]** (constants verified; RSS not measured) | None | Measurement first (existing load harness), then admission cap if real | Prevents OOM-class spikes under hosted concurrency; transient, unmeasured | **Defer past the cut** with measurement-first trigger (hosted load run) |
| E3 | ClinVar gene counts: fixed 15 s leg, ~16+ paced requests, NCBI bucket shared with PubMed; big genes honestly fail; user cannot raise the cap | design gap, med | confirmed | None; VAERS/Monarch already declare `command_budget` for exactly this | One SPEC declaration (+ tests); small | The shipped feature succeeds on the genes it claims to count; today's failure is a truthful typed error, not wrong data | **Accept**: add the declaration; land with the next ClinVar-adjacent work or when a persona run hits the timeout; does not block the cut alone |
| E4 | Hosted HTTP admission has no per-caller limit; one caller can occupy all 8 slots | question, low | confirmed **[w]** (global FIFO; `State` carries no caller identity) | **Declined idea** (2026-11-01): "the hosting wrapper caps work per verified user; 0548 defers per-user caps to it"; stance 3 | None needed per the decline; core sub-cap would be small if ever needed | Fairness across tenants — already owned by the wrapper on the real path | **Reject** — cites that declined idea and stance 3; reopen only if the wrapper's per-user cap goes away |
| E5 | Stdio/plain-HTTP capacity refusals under-report retry time when a slow-source call (60 s VAERS) holds the slot | inconsistency, low | confirmed **[w]** (`ends = now + queue.budget` vs declared source budgets) | None | Few lines | Honest retry advice; self-corrects on each retry | **Defer past the cut** |
| E6 | `fan_out::run` spawns one thread per job, no cap, beside the bounded batch primitive | risk, low | confirmed **[w]**; all current callers bounded (largest: health, ~60 threads) | None | Tiny, folded | Nothing today; exposure only when a new caller is added | **Reject as a standalone change** — fold into whichever change next adds a fan-out caller (the lane's own timing) |
| F1 | Live openFDA punctuation fix (REFUSED set, proven live 2026-11-13) landed in one of six escapers; drug recall, DrugsFDA, label and MAUDE still send characters openFDA answers 400 | likely defect, med | confirmed **[w]** (REFUSED in fda_device_recall only; old `literal`/`phrase` at the four named sites; `device_search` already reaches into the sibling) | None | Small move into `crate::openfda` + adopt at six sites + one replayed 400 case per source | Legal input (punctuation in a drug or device name) stops failing on four sources while succeeding on the fifth; common path, user-visible | **Accept, before the cut** — the most user-reaching defect in this review |
| F2 | `check_docs.py` success line claims case/feature agreement it does not check (the comparison lives in the Rust runner); `cases`/`names` are not dead (non-empty guard, duplicate check) | confirmed defect, low | partly true, reframed and confirmed **[w]** | Stance: gate honesty (same family as D3/D4) | Few lines: reword the line (or move the comparison into Python) | A gate stage's success message states only what it checks | **Accept cheaper fix** (reword) before the cut |
| F3 | `winnow` is an unused workspace dependency; cargo-machete cannot see this class | confirmed defect, low | confirmed | Dependencies-few-chosen-used decision (2026-10-31, cited by the lane) | One-line deletion (+ optional cargo shear in the gate) | Serves a recorded dependency decision; zero-risk | **Accept, before the cut** |
| F4 | Two public constructors (`gene::in_region`, `usage::argument_values`) have no caller in the tree | confirmed defect, low | confirmed **[w]** (definition-only hits) | No-backwards-compat: no API kept for old clients | Trivial deletion | Removes unreviewed surface nothing gates | **Accept, before the cut** (ninth call: cosmetic capacity) |
| F5 | openFDA page-envelope contract in three copies with visible drift (copies drop `Item::counts`) | design gap, low-med | confirmed **[w]** (`Envelope` + two re-declared `check_status` copies) | None | Small refactor; replayed tests exist | One owner for "a valid openFDA page"; prevents the F2/F3-class drift between adapter copies | **Accept, riding F1's openFDA work or the next edit there**; need not block the cut alone |
| F6 | XML attribute extraction copy-pasted per source with drift (trim/empty policy, first-vs-last duplicate) | risk, low | confirmed **[w]** (gtr first-match/trim/empty→None vs wikipathways last-match/no-trim) | None | Small shared helper + policy doc | Same-kind trusted-boundary facts mean the same thing; two sources' optional fields today | **Defer past the cut**; bundle with the next parser change |
| F7 | Identical openFDA Terms restated inline in five SPEC declarations while a shared const pattern exists | inconsistency, low | confirmed **[w]** (five inline restatements; `FDA_TERMS` precedent) | None | Trivial hoist beside `quotas::OPENFDA` | A terms-URL change becomes one edit, not five | **Accept** with any nearby FDA edit (F1's), else before the cut (trivial) |
| F8 | Wire literal "recall-records" defined in two independent enums (drug vs diagnostic domains) | risk, low | confirmed **[w]** | None | Trivial reuse or consts module | One literal cannot silently diverge across the two recall tables | **Defer past the cut** (cosmetic dedup) |
| F9 | FDA table date typing diverges: PMA/510(k) dates raw `Box<str>` ("including explicit empty") vs recall dates parsed `PartialDate` | question, low | confirmed as divergence **[w]**; whether raw-string dating is right is a clinical/schema call | None recorded — that is the finding | Decision line now; unify at most a small refactor at the next schema cut | Records why same-named native dates differ; schema consumers' assumptions stabilized | **Accept cheaper fix**: record the reason beside the typing now; the unify-or-not call goes to the schema owner with a clinical reviewer at the next schema cut |
| F10 | 37 ticket-number narrating comments in `crates/*/src` contradict the deslop rule; no ruling records an exception | risk, low | confirmed **[w]** (37 matches) | deslop-code skill (delete narrating comments); pervasive use suggests an unrecorded convention | Ruling: one line; strip: mechanical batch across ~32 files | Either the convention is recorded or the sediment goes; reviewers stop re-deriving it | **Accept cheaper fix**: PM records the ruling (traceability numbers wanted, or strip); stripping batches after the cut |

## Questions only Ian can answer

1. **Orphadata provenance (C3).** The public repo holds real `www.orphadata.com` `en_product9*` bytes whose inline `Licence` element states Orphanet terms, while the README cites CC BY 4.0 from `sciences.orphadata.com`'s FAQ. Does that license cover the captured `www` files? If not — or if uncertain — swap to the existing `orphadata-synthetic` set. Data-rights call; the repo is public today, and Ian's 2026-11-04 words put license correctness before promotion.
2. **Public sdlc machine names (D7).** Public `sdlc/` tickets name an internal machine, the build machine and an internal machine; the repo's own checker exempts `sdlc/` by a code comment only. Is that exemption intended (planning records reviewed separately), or should the names be generalized before the history append freezes them into public git history permanently?
3. **Hosted stdio mode (C1).** Is `serve --hosted` without `--http` a supported deployment (one trusted caller per process), or should the core refuse it so hosted always means HTTP-through-the-wrapper? If supported, per-caller use tiers need a stdio answer; if not, a 10-line guard closes the gap.

## Calls that came up more than once — stances the PM could adopt

1. **Continuations advance by native consumption, never by admitted rows.** Admitted counts are display-only; every paged owner carries the native consumed count, and a new paged source without one is a defect. (A1, A3; F4/0796 is the precedent.)
2. **One publisher fact, one shared owner.** A fact about one publisher — its escaping rules, page envelope, terms URL, wire literals — lives in one shared module; sibling copies are defects, not style. (F1, F5, F6, F7, F8.)
3. **A gate stage claims only what it checks, and every case file is wired or deleted.** A success message that overstates its checks, or a case directory no stage runs, is false trust and gets fixed with the change that creates it. (D3, D4, D5, D8, F2.)
4. **The change ledger updates with the change, not at the cut.** A ticket that adds or changes a user-visible command, filter, source, config key or credential name updates `release/notes.md` in the same change; the cut never discovers the ledger stale. (D1, D2, D6; A2's disposition line rides this.)
5. **An exemption from a recorded ruling lives in `sdlc/decisions/`, never only in a code comment.** Where code or content departs from a ruling (cache migrations vs no-compat, sdlc machine names vs the internal-word check, PMA date typing, ticket-number comments), the exception is recorded or the code conforms. (B4, D7, F9, F10; stance 8 is the existing pattern.)

## Overall verdict

**Ready after listed fixes.** No finding is Critical or High; none blocks the persona testing that is the current priority, and nothing here cancels or re-dates the parked 1.0. Before the 1.0 cut, in order of weight:

- **Blockers of an honest cut:** D1 and D2 (the notes are the 0.9→1.0 contract and freeze at the tag), F1 (legal input fails on four sources — the one defect ordinary users meet on a common path), A1 (duplicated/stranded trial rows, the accepted F4 class).
- **Minutes-scale batch that should ride the same candidate:** A4, A6, C2, C4, D3, D4, D5, D6, D8, F2 (reword), F3, F4, F7, B1 (evidence tooling), plus the four decision lines (A2 whole-or-fail, B4 cache exemption, F10 comment convention, D7 sdlc names) and the C1 stdio guard once the wrapper transport is confirmed.
- **Deferred with named triggers:** hosted-resume items (E1 disk ceiling, E2 measured memory) trigger before hosted deployment; C3 (Orphadata) triggers before any promotion — but the repo is public, so settle it now; A5, B2, B3, B5, B6, E5, F5 (rides F1), F6, F8, F9 (record the reason now) follow after; E3 lands with the next ClinVar-adjacent work or when a persona hits the timeout; A3 folds into A1's ticket.
- **Rejected:** E4 (that declined idea + stance 3 — the wrapper owns per-caller caps), E6 as a standalone change (nothing reaches it today).

Every recommendation above is evidence-backed by static reading of the pinned snapshot; no claim was executed. The register's own residual risk is that none of the forty findings was reproduced at runtime — the tickets that implement the accepts must start from the failing fixtures the lanes describe.

---

# Appended lane reports

---

# Lane A — Correctness and data integrity of the query layer

Reviewed snapshot: `the pinned snapshot` = origin/main `586c602a5cf54b8b2be65f9c6986979682b21fa6` (pinned 2026-11-14), read-only. No builds, tests or network were run; **every claim below is static source tracing, i.e. "not reproduced"** unless stated otherwise.

Scope: `crates/biomcp-query/src` (drug name resolution/expansion, paging, totals, counts), `crates/biomcp-types/src/{query,response}`, error/empty/absent distinction, schema versioning. Prior findings F1–F8 and the eight stances were read first; none is re-raised. Where I checked, the prior query-layer fixes are present and generalized: F1 (`gwas/threshold.rs:247` emits `qualifying_total` only when `(saved.done && !incomplete)`), F2 (`store/pool.rs` no-ID reads require whole observations; `source_snapshot.rs:132-141`), F3 (`drug/search_names.rs:150-152,214-216` refuses when `received != total` or refusals), F4 (`trial.rs:257-258` advances by native `received`; `NativeWindow::next_offset` advances by native slots).

---

## Findings (ranked)

### 1. [likely defect] NCI trial continuation advances by admitted rows, duplicating valid rows after any refusal — severity: medium

**Broken promise:** the reply's page continuation (`next`) is supposed to walk the source's rows exactly once; after a refused NCI record it rewinds the source position, so valid trials already returned reappear on the next page, and an all-refused page ends paging with the rest unreachable.

**Evidence:**
- `crates/biomcp-query/src/trial/nci.rs:129-147` — `let returned = … page.data.len() as u32;` then `Request::SearchTrial(q.at_page(token, offset.saturating_add(returned)))` — the next `from` offset is built from **admitted** rows.
- `crates/biomcp-adapters/src/nci/mod.rs:55-62` — `Page` carries `data: Vec<Trial>` ("Checked publisher trial rows") plus `refusals` separately; refused rows are **not** in `data`, and the adapter carries no `received` count.
- Arithmetically: a page at `from=0, size=100` with one refused row returns 99 rows; the next token is `99`, so native row 99 — already returned and admitted — is fetched again. A page whose rows are all refused returns 0, `returned > 0` fails (`nci.rs:141`) and no Page step is emitted: the remainder is unreachable except by restarting.
- Contrast the fixed ctgov owner: `trial.rs:257-258` `let read = query.offset().saturating_add(page.received);` — the F4 ticket's pattern, not applied to the NCI sibling. `nci.rs:116` also reports `retrieved_count = page.data.len()` (admitted), which matches the ticketed "returned = admitted" convention, but the **token** must use native position.

**Reproduced:** not reproduced (would need an NCI fixture with a refused trial row across two pages; no tests run).

**Smallest credible fix:** add `received: u32` to `nci::Page` (count publisher rows before admission, as `ctgov` does) and build the token/offset from it; keep `returned` admitted. ~15–25 lines across adapter + query owner plus a two-page refusal fixture; no schema change (PageInfo semantics unchanged). Cost: small code, no speed change, one new gate test.

**Timing:** before release — it is the same defect class the team already accepted as worth fixing (F4), in a sibling owner the fix missed; leaving it means silently duplicated rows in paged NCI results under provider malformation.

---

### 2. [question] Name-resolution walks for Drugs@FDA (sequential) and openFDA labels (batched) have no row cap or unsettled state, unlike MyChem's 250-row cap — severity: low-medium

**Broken promise (candidate):** `get drug <name> --source drugsfda` promises a ranked exact answer; for a very common ingredient name (hundreds of applications) it must read every native page first, and if the command budget expires mid-walk the whole get returns Failed after a long runtime, where the MyChem owner would return a disclosed partial answer.

**Evidence:**
- `crates/biomcp-query/src/drug/resolve.rs:13` `const MYCHEM_NAME_CAP: u32 = 250;` — only MyChem is capped, with an explicit unsettled/partial path (`:396`, `:418-428` "Read the first {rows} of {total} matches … later matches were not examined.").
- `resolve.rs:299` (fda closure) `let next = offset + page.records.len() as u32;` with `next_offset: (… < page.total)` — loops **until the source total**, 50 rows per sequential openFDA request, no cap; budget exhaustion surfaces as a Failed discovery via `native_pages`' error path (`:243-252`).
- `openfda_label/search.rs:47-65` reads **every** word-search page ("one name can match hundreds of labels", cited 0871), batched — cheaper, but still unbounded in rows.
- The FDA ranking promise ("NDA/BLA before ANDA, then earliest approval", `resolve.rs:150-152`) genuinely needs the whole candidate set, so capping changes the claim — this may be deliberate; I could not find a comment or ticket that says so for fda/label, while MyChem's cap is documented.

**Reproduced:** not reproduced (runtime behavior; no runs).

**Smallest credible fix:** either a documented decision that fda/label walks are whole-or-fail (one comment + release-note line, near-zero cost), or port the MyChem pattern (cap + `RequestGuidance` notice + Partial). The decision is the fix; code is optional.

**Timing:** decide before release (it is a one-line disposition if deliberate); code change can follow after release with reason — no wrong data is returned, only a slow-then-Failed path.

---

### 3. [risk] PGx annotation continuations fall back to admitted `returned` when native consumption is not supplied — severity: low

**Broken promise (candidate):** continuation offsets must advance by native consumption; the fallback advances by admitted rows, which rewinds position exactly when a row was refused.

**Evidence:** `crates/biomcp-query/src/pgx/next.rs:89` `let consumed = native_consumed.unwrap_or(body.page.returned);` — the search/get owners pass `Some(consumed)` (`pgx.rs:44,82`), but the annotations owner calls `next::steps(...)` without it (`pgx/annotations.rs:117,125,156`), silently taking the admitted-count fallback.

**Reproduced:** not reproduced; whether CPIC annotation rows can actually be refused while their section still pages was not established (local slicing may make the fallback exact).

**Smallest credible fix:** thread the native consumed count from the annotations owner, or assert the collection is local (offset math over an in-memory list is exact). ~10 lines.

**Timing:** after release with reason — latent, contingent on a refusal path not shown to exist; fold into finding 1's ticket as the same class.

---

### 4. [inconsistency] `DrugNameResolution.total` documents "Total exact source records" but reports only examined candidates — severity: low

**Broken promise:** the schema field says total exact source records; in the capped MyChem walk the number counts only the rows read (≤250) that survived filtering, so it can understate the source's exact records.

**Evidence:** `crates/biomcp-types/src/drug/response.rs:340` "Total exact source records, including the selected record." vs `crates/biomcp-query/src/drug/resolve.rs:128` `total: candidates.len() as u64`. In the capped case the adjacent notice (`resolve.rs:418-428`) does disclose "later matches were not examined", so the undercount is disclosed next to the field, but the field itself reads as exhaustive.

**Reproduced:** confirmed by source reading (the capped path is `resolve.rs:393-431`); no runtime reproduction.

**Smallest credible fix:** one doc/schema-description line ("exact records examined") or omit/qualify the field when unsettled. Cost: a schema description bump (`drug-*/N+1`) and a small conditional; minutes of code.

**Timing:** before release if the schema catalog is still moving; otherwise after release with the next schema bump, since the notice already carries the honest disclosure.

---

### 5. [risk] `get drug … markets` drops the provenance of the successful Drugs@FDA application read, and `add_source` can drop a second leg's notices — severity: low

**Broken promise (candidate):** the envelope documents `sources` as "Sources actually asked"; the second Drugs@FDA request (application get) is asked and succeeds but is never recorded, and when both legs share `(source, state)` the dedupe keeps only the first leg's notices.

**Evidence:** `crates/biomcp-query/src/drug/markets.rs:82-95` — on success the `read` outcome is simply not added (`add_source(out, read)` appears only in the failure branch); `markets.rs:162-170` `add_source` dedupes by `(source, state)`, so a later same-state leg's notices/data-dates are dropped. The listed source is still truthful at source granularity; what is lost is per-leg retrieval metadata.

**Reproduced:** not reproduced (provenance-only effect; no wrong clinical row).

**Smallest credible fix:** merge notices into the existing entry (as `status::retain_source_state` does) and record the application leg. ~10 lines.

**Timing:** after release with reason — no user-visible wrong answer, only incomplete provenance.

---

### 6. [risk] Trial paging assigns (not ORs) `outcome.dropped`, so any future upstream refusal flag is silently cleared — severity: low

**Broken promise (candidate):** refusals must never be lost; an assignment instead of `|=` clears any dropped flag set before `apply_page` runs.

**Evidence:** `crates/biomcp-query/src/trial.rs:223` `outcome.dropped = page.refusals == biomcp_adapters::Refusal::Refused;`. Today no earlier stage in `search()` sets `dropped` (verified by grep across `trial/condition.rs`, `trial.rs`), so behavior is currently correct; the pattern is fragile against exactly the kind of stage the file keeps gaining.

**Reproduced:** confirmed as code shape; no behavioral fault demonstrated today.

**Smallest credible fix:** `outcome.dropped |= …`. One character-class change.

**Timing:** before release — trivial, and it hardens the refusal invariant the release claims.

---

## Direct answers to the lane questions

- **Do reply schemas carry versions so a reader can refuse one it cannot read?** Yes. All 106 `SCHEMA` constants in `biomcp-types` are versioned identities (`biomcp.<name>/<n>`; `response/catalog.rs:14` `biomcp.schema-catalog/4` etc.), and `response/envelope.rs:73-83` `read_schema` rejects any envelope whose `schema` string differs from the expected constant on deserialization. The no-backwards-compatibility decision's "reader can refuse" promise is kept.
- **Totals-before-rows:** structurally enforced — `Envelope` declares `page` before `result` (`envelope.rs:42-49`, cited 0939), the page-body macro puts `$before` (totals, PageInfo) before `items` (`response_page.rs`, cited 0942), and e.g. `DrugRecallPage` declares `total` before `items` (`drug/recall.rs`).
- **Errors vs empty vs absent:** well separated. `Status` (Complete/Partial/Absent/Failed) with exit mapping `Complete|Partial→0, Absent→1, Failed/Usage/Local` distinct (`biomcp/src/exit.rs:29-44`); a search with all-Ok sources and zero rows is `Complete` ("a search remains an answer when its providers return no records", `status.rs:222-239`); a get of a missing record is `Absent`; phenotype code distinguishes Monarch `null` from HPO `[]` (`phenotype/sections.rs:88`); openFDA label get distinguishes "section absent" (named hint, `openfda_label.rs absent_heading`) from "no room in this reply's budget" (`no_room`).

## Strengths

1. **Refusal and completeness plumbing is a first-class contract.** `Refusal` rides independently of field problems everywhere I traced; `NativeWindow` (adapters) separates native `received` slots from admitted rows and advances continuations by `received` (`native_window.rs:88-97`); `search_names.rs` now refuses to trust uniqueness when `received != total` (F3 fix generalized beyond the ticket); pools admit only whole observations as current (F2) with extend-only growth and current-mapping deletion on incompleteness (`store/pool.rs:263-320`).
2. **The status owner is centralized and honest.** `status.rs` is the single place that classifies items/searches/syncs; a failed lookup never reads as empty (`search_names.rs:80-81` "a failed source is never read as an empty one"), a failed drug-name lookup degrades a trial search to Partial via `earlier` (`trial.rs:72-74,110-111,143-146`), and skipped/disabled sources stay visible.
3. **Cross-source substitution is disclosed at every step.** The alias layer (`drug/aliases.rs`) retries other sources' exact literal names only with per-retry notices, refuses on ambiguity (`r.total != 1`), and never lets another source's record stand in for the selected one; the widen layer stores its name decision in a snapshot keyed by the restarted query so later pages reuse exactly the first page's names.

## Skipped, and why

- **`biodata` crate internals** (external git dependency, not in the snapshot): could not verify `PmaNumber`/`Text` validation, so `diagnostic/summary.rs:24` `id.get(1..3).unwrap_or_default()` may be unreachable defensive code — left unjudged.
- **biomcp-fetch internals** (NCBI rate-limit retry, recordings/symlink handling, cache NULL/oversized paths): other lanes; I only verified their query-layer consumption (deadlines bound every loop I read).
- **GWAS/variant/protein deep paths** beyond the fixed F1/F4 spots and a paging-math skim: prior review traced them ("no additional defect established"); I spent the budget on the recently-changed drug/diagnostic owners instead.
- **Live behavior, builds, tests, network**: prohibited by the review rules; everything above is marked accordingly.
- **Clinical/biological correctness of any displayed value** (e.g. PMA supplement selection, label heading choice): clinical-reviewer question, not mine.

---

# Lane B — Stored state and data integrity (BioMCP review, snapshot 586c602a, pinned 2026-11-14)

Scope: `crates/biomcp-fetch/src` (transport, cache, recordings, bundle store/sync), the sync/store code reachable from `crates/biomcp/src` (`command/sync.rs`, `config.rs`, `paths.rs`, maintenance/clean), the query-side snapshot pool (`crates/biomcp-query/src/store/pool.rs`, `source_snapshot.rs`), and the GTR/streamed-XML path in `crates/biomcp-adapters/src/gtr`. Read-only: nothing was built, run, or tested; every claim below is source-traced and marked **not reproduced**.

Prior dispositions honored: F1–F8 findings, the eight review stances, and the no-backwards-compatibility decision were read first. Notably, the F2 fix (whole-only current answers) and F6a/F6b cache fixes appear to have landed in this snapshot and are treated as strengths below, not re-raised.

## Findings (ranked)

**[1] design gap | medium | The record/replay tool cannot capture or replay the largest declared bundle: recording caps (50 MiB binary sidecar, 16 MiB text) are far below GTR's real set sizes (~184 MB gzip xml part, 384 MiB declared), so `--record` on a real `sync gtr` fails with `fetch.record_limit` after downloading the whole file.**
Evidence: `crates/biomcp-fetch/src/record_sidecar.rs:8` `const MAX_BINARY_BYTES: u64 = 50 * 1024 * 1024;` vs `crates/biomcp-adapters/src/gtr/bundle.rs:38-40` (xml part `max_bytes: 384 * 1024 * 1024`, 0931 measures 184 MB gzip); the set-part download goes through the ordinary request path (`crates/biomcp-fetch/src/bundle/set.rs:163` `RowFormat::StreamedXml { .. } => crate::Read::bytes()` → binary codec) and every binary body is captured as a sidecar (`crates/biomcp-fetch/src/request.rs:503` `Body::Bytes(wire.body.clone())`), so `sidecar::publish` refuses at 50 MiB. Text parts hit the 16 MiB `MAX_FILE_BYTES` (`record.rs:19`) — the pre-0931 GTR sync was already ~57 MB total, so `--record` on real GTR data plausibly failed before 0931 too. Replay is equally capped: `record_sidecar.rs:44` `let limit = limit.min(MAX_BINARY_BYTES);`. The team evidently works around this by hand-cutting captures (`tests/cases/diagnostic/gtr-specimens.yaml`: the xml was "fetched live on 2026-11-14 and cut to the labs and tests of the EGFR replay"), so evidence fixtures for the biggest source cannot be produced by the tool that exists to produce them.
Reproduced: not reproduced (no runs allowed; path traced end to end).
Smallest fix: allow set-part/bundle binary captures up to the streamed ceiling — e.g. spool large sidecars to a bounded part file in the recording folder (mirroring `bundle/store.rs::create_part`) or raise the sidecar cap for `Mode::Record` bundle-part legs and bound the recording file limit accordingly; one focused test with an over-50-MiB synthetic capture. Cost: small code, no speed change, one gate test.
Timing: before release — the 0954/0963 line of tickets shows live `--record` syncs are the standing evidence workflow, and today the tool fails on the source that most needs evidence, after wasting a full 184 MB download.

**[2] inconsistency | low | A single-bundle `StreamedXml` download is capped by `R::MAX_BYTES`/`MAX_BUNDLE_BYTES` (128 MiB) while the format declares ceilings up to `MAX_STREAMED_BYTES` (512 MiB); only set parts honor the format's own cap — latent, since GTR is today the only `StreamedXml` source and it is a set part.**
Evidence: `crates/biomcp-fetch/src/bundle/download.rs:260` `self.stream(t, response, R::MAX_BYTES)?;` and `download.rs` `validate_range` refuses `total > MAX_BUNDLE_BYTES` on both 200 and 206; but `crates/biomcp-fetch/src/bundle.rs:107-111` documents `StreamedXml { max_bytes }` as "bounded by `MAX_STREAMED_BYTES` instead of the set ceiling", and only `bundle/set.rs:150` (`RowFormat::StreamedXml { max_bytes, .. } => max_bytes.min(super::MAX_STREAMED_BYTES)`) implements that. `parse::rows` (`bundle/parse.rs:153-154`) still accepts bodies up to the format's `max_bytes`, so download and parse would disagree if a `Bundle` (not a `BundlePart`) ever used the format.
Reproduced: not reproduced (no current source exercises the single-bundle streamed path).
Smallest fix: pass the format's `max_bytes` into `stream()`/`validate_range` caps in `sync_bundle`/`consume`, or assert at construction that a `Bundle` never declares `StreamedXml`. Cost: a few lines plus one unit test.
Timing: after release — latent only; becomes real the day someone declares a streamed single bundle.

**[3] risk | low | Replay follows symlinks inside the recording folder when reading a recording JSON: the folder's own path is canonicalized (0954) and sidecar reads/publishes refuse any symlinked ancestor, but `read()` opens `<dir>/<source>/<key>.json` with no symlink check, so a symlinked source subdirectory or a symlinked JSON file is followed on replay reads.**
Evidence: `crates/biomcp-fetch/src/record.rs:348` (Replay canonicalizes the top folder), `record.rs:501-512` (`fn path` builds `dir.join(source).join(key.json)`; `read` does `fs::File::open(path)` with no symlink check) vs the sidecar protections at `record_sidecar.rs:23-30` (`checked()` walks `parent.ancestors()` refusing symlinks) and `record_sidecar.rs:41` (sidecar file itself refused if a symlink). Write-side containment inside `--record` folders is ticketed as 0963; this is the read/replay side, which 0963 does not cover. Impact is low: fixtures are local and the file must still match the recorded identity (method/source/url), so this is provenance confusion, not arbitrary-file disclosure.
Reproduced: not reproduced.
Smallest fix: run the same ancestors walk (or `safe_path`) on the recording file's parent inside `read()` before opening; a few lines plus one symlinked-fixture test mirroring 0963's.
Timing: after release — local-fixture hardening adjacent to already-ticketed work.

**[4] inconsistency | low | Old-format cache migration code is retained despite the 2026-11-01 no-backwards-compatibility decision ("no readers or migrations for old formats").**
Evidence: `crates/biomcp-fetch/src/cache/schema.rs:198-204` migrates v1/v2 reply tables in place (`ALTER TABLE reply ADD COLUMN content_range/pagination`); `schema.rs:102` declares `migratable_versions: &[1, 2, 3, 4]`; `crates/biomcp-fetch/src/bundle/transfer.rs:171` tolerates missing predecessor tables for `("bundle_content", 1..=3) | ("bundle_rejection", 1..=4)`, i.e. readers for old stored formats. The decision (`sdlc/decisions/2026-11-01-no-backwards-compatibility.md`) says BioMCP "carries no compatibility code: no readers or migrations for old formats". The codebase also has deliberate non-compat machinery for exactly this case (`cache_predecessors`, `import_bundle`, `clean_incompatible_files`), so the migration paths may simply be stale. Per the review instructions this is flagged as compat code, not as a missing reader.
Reproduced: not reproduced (no old cache files exercised).
Smallest fix: either delete the in-place ALTER/migratable paths and let the predecessor-copy/cleanup machinery own old files, or record a one-line decision that local cache files upgrade in place and are exempt from the CLI/MCP no-compat ruling. Cost: small deletion plus tests either way; the decision route is nearly free.
Timing: after release with a PM decision — internal state only; no user-visible surface changes.

**[5] risk | low | Recording sidecars carry no content hash: a corrupted or truncated `.bin` sidecar (within its size limit) replays silently as the captured body, with no diagnostic — unlike bundle bodies, which are hashed end to end (`content_revision`).**
Evidence: `crates/biomcp-fetch/src/record.rs:184-190` — `Body::Sidecar(String)` stores only a filename, no digest; `record_sidecar.rs:34-56` reads the file and length-checks it, nothing more; the JSON recording itself is structurally validated (`record.rs` `read`, `deny_unknown_fields`, exchange `valid()`). Compare `bundle.rs` `bundle_sha256` used in `provenance()` for live and replayed bundle bytes (`bundle/download.rs` `provenance`). Recordings are the evidence base for tests (`tests/replies/**`), so silent fixture corruption fails or, worse, passes tests against wrong bytes.
Reproduced: not reproduced.
Smallest fix: store a sha256 beside the sidecar name in the serialized exchange and verify on read (a new optional field; `deny_unknown_fields` means old fixtures without it can be rejected or grandfathered per the no-compat stance). Cost: small code, negligible speed, one test.
Timing: after release — test-infrastructure integrity; nothing user-facing depends on it.

**[6] design gap | low | Multi-part set syncs take no claim/lease, so two concurrent `sync gtr` runs both download ~245 MB and both publish; readers are never torn (publication is atomic) but the duplicate work is unprotected, unlike single-bundle syncs which fence via `bundle_sync` claims.**
Evidence: `crates/biomcp-fetch/src/bundle/set.rs:75-115` (`sync_set` downloads every part through `part_body` and publishes once; no `claim`), vs the single-bundle path `bundle/download.rs:66-97` (`cache.claim(...)`, `Err(CacheError::Busy) => Reason::Syncing`) and `bundle/store.rs` `claim/retain/discard` with lease fencing and OS file locks. `read_stored_rows`' wait loop (`bundle/read.rs`) only watches `bundle_sync` rows, which set syncs never write.
Reproduced: not reproduced.
Smallest fix: reuse the `bundle_sync` claim row for set syncs (name = set name) or accept and document the duplicate-download behavior. Cost: small if the claim row is reused; zero if documented.
Timing: after release, or never with a reason — correctness is preserved by atomic publication; only bandwidth/time is at stake, and set syncs are explicit, scheduled commands.

## Strengths

1. **The F2 fix landed coherently and everywhere the prior review demanded.** Whole-only current answers are enforced at write and read: `pool_current` is keyed by a versioned prefix (`source_snapshot.rs` `current_key`, "biomcp.source_current/2"), `publish_source_pool` installs current only when `!incomplete && sources_whole` (`store/pool.rs:272`), `extend_source_pool` deletes the mapping when an extension goes incomplete (`pool.rs:319-325`), and no-ID reads refuse dropped/partial pools and snapshots (`pool.rs:217-220`), with the same gating visible in protein search/relations, GWAS, ORCID, pathway and study paging callers, plus explicit losing-writer/recovery tests in `store/privacy_tests.rs:345-384`. Verified by reading only, not by running.
2. **Bundle storage is genuinely atomic and crash-fenced.** Part files are `NamedTempFile`s held under OS advisory locks as liveness witnesses, claims are lease-fenced and refuse live holders (`bundle/store.rs` `claim`/`syncing`/`in_use`), publication stages rows in per-connection temp tables and swaps them inside one immediate transaction with a holder check (`bundle/publish.rs`), and set publication publishes all parts' rows in a single transaction, so an interrupted sync can never expose a half-written catalog. Predecessor import re-verifies staged rows against the source before commit (`bundle/transfer.rs` `verify`).
3. **Transport cache hygiene is careful at the reader/writer boundary.** A body too large for one reader is retained for wider readers instead of being deleted (the `oversized` flag, `cache.rs:218-233` — F6a fixed), expired cached 404s in offline mode produce an explicit "current absence is unknown" notice rather than a false absence (`cache_request.rs` `offline` — F6b fixed), cache admission is bounded per endpoint and gated on license checks, and record/reply formats are version-stamped and refused (or recreated) on mismatch (`meta` owner versions; query store drops and recreates on mismatch rather than reading old rows).

## Accepted risks and dispositioned items (one line each)

- GTR set sync peak memory (~1.9 GB, all rows held until publication) is measured and deliberately deferred in ticket 0931 (COMPLETE): accepted, not re-raised.
- The compressed GTR xml body being held whole in RAM is documented in 0931 ("The compressed body is still held in memory like every other part"): accepted.
- Write-side symlink refusal inside `--record` folders is ticket 0963: not re-raised (only the replay read path is raised, finding 3).
- No size-based cap on total cache growth (time-based expiry plus explicit `clean` only): a design choice surfaced by `store_size`; not raised as a defect.

## What I skipped and why

- **No execution at all** (hard rule): every claim above is code-traced and marked not reproduced; no builds, tests, or network calls were made against the snapshot.
- **Quota limiter internals, gRPC connect/decompress, TLS, containment scanning**: skimmed for bounds and found none obviously missing, but not traced line-by-line within this lane's time budget; no defect claimed.
- **SQLite page-level corruption** of cache/bundle databases: SQLite's own domain; the application-level checksums that exist (content_revision, schema versions) were checked, and nothing more was demanded.
- **Secrets**: searched by name only (`proxy`, `FetchConfig::secrets`, credential query stripping in `retry.rs` `canonical`); no credential value was printed, and no leak was found in the files read (recordings are documented and validated as credential-free).
- **Stale doc nit**: `RecordError::NonUtf8`'s doc ("Binary capture needs a future sidecar representation") is outdated now that sidecars exist — the seven failure reason strings themselves are accurately produced at each site; folded here rather than spending a finding slot.
- **Clinical/biological correctness**: out of scope by instruction; nothing in this lane required a clinical judgment.

## Acceptance contract evidence

- criterion-1 (concise result and residual risks): satisfied by this document; residual risks are the "not reproduced" markers — every finding is source-traced only, and findings 1–3 in particular should be confirmed with a controlled run before ticketing fixes.

---

# Security, privacy and data-rights review — BioMCP @ 586c602a (pinned 2026-11-14)

Lane: security, privacy, data rights. Read-only source inspection of `the pinned snapshot`; no builds, no tests, no network. Prior review F1–F8 and the eight review stances were read first; nothing already dispositioned is re-raised. Everything below is source-traced, not executed: every finding is marked **not reproduced**.

## Findings (ranked)

### 1. Hosted stdio has no per-caller use declaration: noncommercial sources silently serve with the server's own credentials

- **Label:** likely defect (design gap on a supported hosted mode) | **Severity:** medium (high if any deployment shares one hosted stdio core across callers)
- **Broken promise:** hosted mode is documented as a shared server with per-caller use tiers and per-caller credential scope, but every per-caller control (`x-biomcp-use`, `x-biomcp-exclude-sources`, `x-biomcp-key-*`) exists only on the HTTP transport; on stdio hosted, use-tier restrictions never apply and the server's fallback-tier credentials are used for every caller.
- **Evidence:**
  - `crates/biomcp/src/command/hosted_use.rs:44-49` — `if !matches!(context.declared_use, Some(DeclaredUse::Restricted | DeclaredUse::Commercial)) { return fetch; }` — with stdio's `declared_use: None` the function returns before applying `restrict_uses([Noncommercial, Licensed, PerRecord])`.
  - `crates/biomcp/src/command/context.rs:16,34` — stdio contexts are built with `declared_use: None`, `secrets: None`, `excluded: None` (`Context::new`).
  - `crates/biomcp/src/mcp/http.rs:251-266` — the `caller()` gate that parses `x-biomcp-use`/`x-biomcp-exclude-sources`/key headers is reached only from the HTTP route.
  - `crates/biomcp-fetch/src/secret.rs:126-145` (`Secrets::hosted`) removes only `KeyTier::NoFallback` server keys; fallback-tier server credentials stay effective.
  - `crates/biomcp-adapters/src/orcid/mod.rs:19-23` and `crates/biomcp-adapters/src/semantic_scholar/mod.rs:11-17` — ORCID (`Use::Noncommercial`, optional Bearer token, fallback tier) and Semantic Scholar (`Use::Noncommercial`, optional `x-api-key`, fallback tier): on hosted stdio both are served, keyless callers included, using the operator's stored token/key, with no commercial/restricted declaration possible.
  - `crates/biomcp/src/grammar/local.rs:14-17` — `serve --hosted` without `--http` is a first-class stdio mode ("Refuse host file saves, health details and store or key changes over MCP"), and `docs/configuration.md:139` markets hosted for "a shared server, with `--http` for HTTP transport".
- **Reproduced:** not reproduced (no runtime; static trace only).
- **Smallest credible fix:** make the stdio gap explicit at startup — either (a) refuse `--hosted` without `--http` (one guard in `serve` + one test; breaks any stdio-hosted wrapper, so confirm the wrapper's transport first), or (b) add a startup `--use` ceiling flag for stdio hosted plus one docs paragraph stating one-caller-per-process. Cost: (a) ~10 lines; (b) ~30 lines + docs; both fit the routine gate.
- **Timing:** before release (cheap, and the no-backwards-compatibility stance makes (a) allowable once the wrapper transport is confirmed).

### 2. Plain-HTTP `article pdf` / `trial document` with `--save` downloads the full document, then always refuses the save

- **Label:** inconsistency | **Severity:** low
- **Broken promise:** hosted and plain HTTP are supposed to refuse MCP file saves, but on plain HTTP the save-shaped commands are admitted, the whole publisher document is fetched, and only then the save is refused — while the identical hosted call is refused up front with `usage.mcp_unavailable`.
- **Evidence:** `crates/biomcp/src/command/mcp_policy.rs:39-54` — the `http` block refuses `Sync`/`StoreClean`/`--fresh` but no save; `mcp_policy.rs:56-66` `saves()` matches only `ArticleText`/`ArticleSupplement` optional saves plus schema-required saves. Because `article-pdf.json`/`trial-document.json` make `save` a *required* field, hosted still refuses them (via `always_refused`), but plain HTTP does not: `crates/biomcp/src/command.rs:331-333` (`tool_http` → `allow_save = false`) only neutralizes the write. `crates/biomcp/src/save/reply.rs:71-76` then returns `UsageError::PdfSaveRefused(link)` after the bytes were downloaded.
- **Reproduced:** not reproduced (static trace).
- **Smallest credible fix:** extend the `http` block in `mcp_policy::refusal` to refuse commands whose schema declares a save (reuse `can_save(&schema)`), so plain HTTP refuses before any publisher traffic. Cost: ~5 lines + one test in the existing hosted-surface runner.
- **Timing:** after release (bandwidth/consistency only; no data leaves and no file is written).

### 3. Orphadata capture provenance: CC BY 4.0 claim is cited from a different site than the captured files

- **Label:** question (data rights; owner decision, not mine to judge) | **Severity:** low-medium
- **Broken promise:** stance 7 — captures from noncommercial or licensed sources in the public repo are synthetic (or demonstrably redistributable); the retained Orphadata captures are real, and the README's license claim is sourced from `sciences.orphadata.com`'s FAQ while the captured bytes come from `www.orphadata.com` XML files whose inline `Licence` element states Orphanet-specific terms.
- **Evidence:** `tests/replies/disease/orphadata/README.md` — "cut from the live files fetched on 2026-11-13: `GET https://www.orphadata.com/data/xml/en_product9_prev.xml` … Orphadata Science data is licensed under CC BY 4.0 (https://sciences.orphadata.com/faq/)"; the retained body in `tests/replies/disease/orphadata/orphadata/f24d53….json` opens with `copyright="Orphanet (c) 2026"` and an inline `<Licence>` element from the www file. Every other licensed capture set in `tests/replies` is explicitly `-synthetic` (e.g. `oncokb-synthetic`, `orphadata-synthetic`), which shows the team applies the stance — this one relies on the cross-site license equivalence.
- **Reproduced:** not reproduced (no network; I could not re-check either site's current terms).
- **Smallest credible fix:** one owner confirmation that `www.orphadata.com`'s `en_product9*` files are covered by the same CC BY 4.0 as Orphadata Science (or replace with the existing synthetic set); a one-line provenance note citing the exact license page for the www files. Cost: investigation only, zero code.
- **Timing:** before release/publication (the repo is public; license verification is cheap now and expensive after a complaint).

### 4. The 429 containment comment overstates the invariant: the throttle body *is* read into memory when the request is keyed

- **Label:** risk (comment contradicts code) | **Severity:** low
- **Broken promise:** the in-code security contract says "A throttle reply is retried and never read", but a keyed 429 body is fully read into memory first (`wire_body` reads whenever `self.keyed()`), then cleared and re-contained; the invariant that actually holds is "never decoded, cached, recorded or logged", and a future maintainer trusting the comment could log `wire.body` believing it empty on 429 paths.
- **Evidence:** `crates/biomcp-fetch/src/request.rs:363-377` — "A throttle reply is retried and never read, so it keeps its status and loses its body"; but `request.rs:348-358` (`wire_body`) reads the body when `recording || self.keyed() || …`, and only afterwards `request.rs:367-376` clears `wire.body` and re-runs `contain_response`.
- **Reproduced:** not reproduced (static trace).
- **Smallest credible fix:** reword the comment to "…never decoded, cached, recorded or logged; the raw body is read only to scan for a reflected credential and is cleared before any consumer". Cost: one comment, zero behavior.
- **Timing:** before release (trivial; protects the NCBI rate-limit fix that landed this week).

## Verified clean (hunt list, with the strongest evidence)

These are the areas the lane was told to hunt; each was traced and found sound, so they appear here rather than as findings:

- **NCBI key-echo 429 fix, all paths including exhausted retries:** a keyed 429 body is read once for scanning, cleared, re-contained (headers-derived metadata included), and never reaches the cache (only success and fresh-404-negative entries are written: `crates/biomcp-fetch/src/cache_request.rs:100-118`), recordings (`request.rs:379-388` records the cleared wire), observations (`observe_exchange` sees the post-clear body; `crates/biomcp-fetch/src/observe.rs` keeps credential-free URLs and shape-only structure), or error text (`SourceError` is a fixed-vocabulary struct, `error.rs:35-64`). A key in any *other* status, or left in the 429's headers, is refused as `BadReply` (containment re-runs on the retained metadata). Other sources get the same or stricter treatment: non-keyed error bodies are never read at all (`request.rs:348-358`).
- **No credential can format:** `Secret`/`Secrets`/`KeyFingerprint`/`TrialCursorKey` Debug/Display are `***` (`secret.rs`, `trial_cursor.rs`); transport errors collapse to enum reasons before any text leaves the crate (`tls.rs:186-199`, gRPC `status()` discards `tonic::Status::message()`); production diagnostics are fixed strings (`exit.rs:25`, `bundle/refresh.rs:100-103`, `http/connections.rs:99-104`); HTTP operator logs record only tool name, duration, status and registry-validated failed source names (`mcp/http/logging.rs:11-28`).
- **Keys never enter argv** (`keys.rs:1` "no credential enters a command argument"; `input()` reads a terminal prompt or bounded stdin, 8192 cap); config-with-keys is written 0600 atomically with a 0700 parent (`config/key_file.rs:50-73`); cache files are created 0600 (`cache.rs:334-346`).
- **URLs, SSRF and redirects:** prepared/cached/observed URLs are credential-free by construction (`retry.rs:62-79`, `wire_request.rs:160-177` builds the keyed URL only at the wire); outbound origins must equal declared bases (`hosts.rs:16-33`); publisher downloads allow domain-only hosts, resolve once through a public-IP-only resolver, cap redirects at 3 with each hop re-checked (`destination.rs:38-160`).
- **File routes:** recordings use source-name + hex-identity filenames only (no caller text in paths, `record.rs:392-395`); replay case caches refuse symlinked ancestors and unexpected entries (`record.rs:108-153`); binary sidecars refuse traversal and symlinked ancestors and install atomically no-clobber (`record_sidecar.rs:11-24,55-77`); saves canonicalize parents, refuse existing/symlink destinations, and write temp-then-rename without following links (`save.rs:41-95`); skill installs take paths only from compiled assets with symlink/traversal checks (`skill/install.rs:8-46`); MCP `--save` refuses every dot-prefixed path component after symlink resolution (`mcp_policy.rs:75-97`).
- **HTTP surface:** non-loopback binds refuse to start without `--allow-host` (`http.rs:66-68`); Host-header authority is strictly parsed (no IP-literal tricks, port must match) and any `Origin` header is rejected (`authority.rs`, `http.rs:380-393`); framing is bounded (16 KiB head, 2 s read budget, strict chunked parser, duplicate-header rejection, `wire.rs`); admissions reserve a reader for `/health`; hosted callers must pass a valid `x-biomcp-use` (missing header defaults to the most restrictive tier, `hosted_use.rs:22-31`); caller key headers are length/charset-checked and ignored entirely in non-hosted HTTP (`wire.rs:29-42`, `http.rs:338-344`).
- **Hosted refusals:** `key`, `health`, `sync`, `store clean`, `--fresh`, `--replace`, and all save-bearing requests (pdf/trial-document saves are schema-*required*, so `always_refused` catches them) are refused on both transports (`mcp_policy.rs:11-54` + `article-pdf.json`/`trial-document.json` `required` arrays); `--record/--replay/--case-cache` are refused as MCP args and `--observe/--config/--source_pacing` are neutralized by `Globals::inherit` (`command.rs:336-343`, `grammar.rs:438-467`).
- **MCP stdio input:** raw `biomcp`-tool args and the `fetch` projection re-enter the full grammar with the same policy gates; provider-supplied ids are shlex-quoted on output and re-split on input, so row text cannot smuggle flags (`research/records.rs:17-21`, `research.rs:81-84`).
- **Markdown/URL escaping:** source text is escaped so Markdown, HTML, GFM autolinks and bidi controls cannot form (`view/escape.rs`); research URLs must be http(s) (`records.rs:17-19`).
- **Env vars:** production set is `HOME/XDG_*/APPDATA/PATH`, proxy vars, per-source key vars, `BIOMCP_TRIAL_CONTINUATION_KEY`, `BIOMCP_SQLITE_DIAGNOSTICS` (documented in docs/), `BIOMCP_STUDY_DIR` (retirement notice only) — no undocumented behavior-changing variable found; `BIOMCP_MONARCH_MAPPING_FIXTURES` is an `#[ignore]`d test hook.
- **No docs advise disabling security checks** (searched for insecure/verify-skip/trust-all patterns across docs and src).
- **Public-repo captures:** every licensed-source capture set is `-synthetic` (oncokb, orphadata-synthetic, seer-synthetic, gwas-synthetic, protein-synthetic); the only non-synthetic licensed-adjacent set is finding 3.

## Strengths

1. **Credential hygiene is structural, not conventional.** The `Secret` type cannot format its value, identities/caches/recordings/observations are all built from credential-free canonical URLs, keys attach only at the wire, and every reply (including cached reads, replayed exchanges and pagination headers) passes reflection containment against the effective credential. Leaking a key by accident requires defeating several type-level barriers.
2. **Error and diagnostic surfaces are fixed-vocabulary by construction.** Every error type that can reach stderr, logs or MCP replies (`SourceError`, `RequestError`, `FetchError`, `CacheError`, `RecordError`, `TlsCause`, gRPC status mapping) is a closed enum with fixed strings — provider text, URLs, SQL and paths are structurally excluded.
3. **The HTTP transport is defensively small and correct.** Strict authority admission with loopback defaults, Origin rejection, bounded/strict framing, reserved health readers, and a missing-use-header defaulting to the most restrictive tier show a threat model that was actually applied, not bolted on.

## What I skipped and why

- **Runtime verification of anything** — the rules forbid builds/tests/network; all findings are static traces and marked not reproduced.
- **Clinical/biological correctness** of any filtered content — clinical-reviewer question, out of this lane.
- **Wrapper (hosting repo) code** — not in this snapshot; stance 3 says the core is judged behind the wrapper, so wrapper-path findings (F5 issuer keys, header stripping) stay with the hosting lane and existing tickets.
- **Cache staleness/pool eligibility, GWAS totals, drug expansion, trial paging** — already dispositioned as F1–F4 with tickets.
- **Deep re-verification of hosted recording/continuation ownership and store privacy** — prior review traced "stored caller ownership, parsed maintenance restrictions"; I only confirmed the enforcement sites exist (`store/ownership.rs`, `privacy_tests.rs`).
- **DoS/load behavior beyond admission bounds** (thread-per-connection costs, quota fairness under load) — the prior review's load-benchmark and admission tests cover the design; I found no unbounded path but did not re-derive the analysis.
- **`Recordings::open` (Record mode) not running `safe_path` on the record dir** — `--record` is a local operator-only flag (refused on MCP), canonicalized before sidecar writes, so it is not a trust boundary.

---

# Lane D — release, docs and gate trust

Review of the pinned snapshot `the pinned snapshot` = origin/main `586c602a5cf54b8b2be65f9c6986979682b21fa6` (2026-11-14). Scope: release/, manifest.json, deny.toml, CHANGELOG.md, ROADMAP.md, docs/, Makefile, scripts/, skills/ embedded in the binary. Prior review F1–F8, the eight review stances and the no-backwards-compatibility ruling were read first; nothing already dispositioned is re-raised. No builds, tests or network were run: every claim below is source-traced and marked accordingly. No secret values are printed.

## Hunt questions answered up front

- **Does the landing gate hold real stages, and can each fail?** Yes, structurally. `scripts/landing_gate.py` runs nine real `make` goals through `scripts/timed_stage.py` (which defaults to exit 127 and prints `exit=` even on failure); `scripts/test_gate_stages.py:80-131` proves stage order, stop-on-failure and pre-job's keep-going failure reporting against a fake `make`. I found no stage that prints "passed" without running, and no check that exits zero when its tool is missing (`check_docs.py`'s git fallback widens rather than narrows; `secrets.py` refuses an empty `--since`; `make secrets` fails loudly if `release/secrets-main` is absent). The one real "no-op when unset" I found is finding 3.
- **Are release scripts no-ops before the tag?** Yes — stance 5 honored. `release/publication.py:26-38` (`identity`) requires `GITHUB_EVENT_NAME=push`, `GITHUB_REF=refs/tags/v<version>` and matching `GITHUB_SHA`; every publish step in `.github/workflows/release.yml` is conditioned on `github.event_name == 'push' && github.repository == 'genomoncology/biomcp'`, and rehearsal dispatches print explicit no-channel boundary summaries. Not a finding.
- **Links and anchors in packaged docs?** Checked every relative link and anchor in README, CHANGELOG, ROADMAP, docs/, release/notes.md, skills/ (31 files) and every `github.com/genomoncology/biomcp/blob/main/...` link tree-wide: zero broken files or anchors. Not a finding.
- **Secrets/private captures in the public tree?** Scanned the public tree (excluding `sdlc/`, `tests/`) for key/private-key/token patterns: only synthetic test values (`sentinel-…`, `synthetic_credential`, `synthetic-secret`). `release/secrets.ignore.jsonl` rows all carry reviewed reasons. No leak found. One naming question remains (finding 7).

## Findings

**[1] confirmed defect | medium** — The 1.0 release notes tell users a credential name that does not exist.
Evidence: `release/notes.md:63` — "The source credential names `ONCOKB_TOKEN`, `NCBI_API_KEY`, `S2_API_KEY`, `NCI_API_KEY`, `OPENFDA_API_KEY`, `ORCID_ACCESS_TOKEN`, `UMLS_API_KEY` and `ALPHAGENOME_API_KEY` remain recognized." The code has exactly seven credential names, none of them UMLS: `crates/biomcp-fetch/src/secret.rs:176-186` (`environment()` maps only Openfda/Ncbi/S2/Orcid/Oncokb/AlphaGenome/Nci) and `crates/biomcp-adapters/src/decl.rs:156-175` (`keys()`/`key_name` over the same `SecretName` set). No `UMLS` string exists anywhere in `crates/`. A commercial user following the primary migration document sets `UMLS_API_KEY`, silently gets nothing, and the licensing guidance ("Every source starts enabled; commercial users must disable…") sits in the same sentence.
Reproduced: confirmed by direct source tracing (code + doc); runtime not exercised.
Smallest fix: delete `UMLS_API_KEY` from that sentence (one-line edit; no code, no gate time — `check_docs.py` only checks the retired-tool table in notes.md, so nothing else must be regenerated).
Timing: before release — the notes are the no-compat contract ("The release notes list every removed or changed command, option, config key…", `sdlc/decisions/2026-11-01-no-backwards-compatibility.md`); after the tag this would need a patch release to correct.

**[2] design gap | medium** — No user-visible change from the recent (~60) landed tickets is reflected in CHANGELOG.md or release/notes.md.
Evidence: `CHANGELOG.md` is one paragraph pointing at the notes; `release/notes.md` (196 lines) has zero mentions of GTR specimens/certifications (ticket 0931; live fields in `crates/biomcp-types/schemas/diagnostic-card.json:347,774` — "`header.specimens` and `laboratory_certifications`"), ClinVar gene classification counts (0932/0946/0950; visible in `docs/reference/commands.md:3395`), the whole FDA recall/PMA/CDx surface (0897–0959: `--classification`, `--intended-use`, `--classification-name`, `--sort`, dated CDx rows), HPO ancestors in one call (0908; new schema `biomcp.phenotype-ancestors/1`), PGX phenotype filter (0936), article PDFs from Europe PMC/bioRxiv (0965; `docs/reference/commands.md:1766` "europepmc: words in open full text and preprints"), the new `sync_timeout_seconds` config key (0951; documented in `docs/configuration.md:123` but absent from the notes' configuration-migration list), and symlink-refusing record folders (0963). Grep for specimen/recall/pma/europepmc/biorxiv/ancestor in notes.md: 0 hits. Docs/configuration.md and the generated reference are current; only the user-facing change ledger is stale, and nothing enforces it (no script references CHANGELOG.md at all).
Reproduced: confirmed by grepping the snapshot; no runtime.
Smallest fix: one writing pass adding a short "Added in the final pre-release window" section to release/notes.md (new commands, filters, sources, config key) and either mirroring it in CHANGELOG.md or deleting CHANGELOG.md down to a pointer; no code change.
Timing: before release — the notes are frozen into the GitHub release (`release/workflow.py:145-148` stages them verbatim) and become the 0.9→1.0 contract at the tag.

**[3] design gap | medium** — The release schema-contract comparison only runs at rehearsal/release; the routine gate never executes it.
Evidence: `crates/biomcp-types/tests/schemas/release.rs:12-14` — `generated_schemas_preserve_release_contracts` returns `Ok(())` immediately when `BIOMCP_SCHEMA_BASELINE` is unset. The landing gate and full suite (`scripts/landing_gate.py:10-14`, Makefile `library-tests`) run biomcp-types schema tests without that variable, so the only enforcement is `make schema-release-check` (`Makefile:190`), invoked in the release workflow (`.github/workflows/release.yml:166`), the rehearsal (`release/rehearse.py:61`) and one ticket job (`scripts/label_names_proof_job.sh:24`). A ticket that narrows a reply schema without a version bump or a `release/schema-reviews.json` entry lands green through the normal gate and first fails at rehearsal or on the tag. That is exactly the "proof that only the release runs" pattern stance 5 moves into the routine gate (`sdlc/decisions/2026-11-06-review-stances.md`, stance 5). Not a stance-breaking defect (it does run before publication), but a late-discovery gap the stance is written against.
Reproduced: not reproduced (no build run); control-flow traced directly.
Smallest fix: add `schema-release-check` to the FULL suite and to the landing gate when the changed paths touch `crates/biomcp-types/` or `release/schema-reviews.json` (the selection pattern already exists in `scripts/changed_library_tests.py`); cost is one Makefile stage plus ~1–2 minutes of warm cargo test time, only on schema-touching tickets.
Timing: before release — cheap insurance while schemas are still moving weekly; after release it becomes a tag-time surprise.

**[4] likely defect | low** — Nine YAML case files under `tests/tool-cases/` are executed by no gate stage; they look like coverage but cannot fail.
Evidence: the runner discovers only `tests/cases` by default (`crates/biomcp/tests/runner/case_main.rs:61` — `directory.unwrap_or_else(|| …fixtures().join("cases"))`), nothing else selects `tests/tool-cases` (only mentions are `sdlc/records/2026-11-06-ticket-review.md:938` and tickets 0623/0765), and ticket 0765 already documented the failure mode: "Job j35187 ran `tests/tool-cases/drug/label-discovery.yaml`, a stale duplicate no gate runs … The tool-case copy is deleted." The nine remaining files (2 adverse-event, 3 article, 4 drug source-alias cases, each with typed-MCP assertions) rot invisibly the same way. `tests/retained-cases/` is explicitly documented as intentionally not run ("the default routine suite does not execute it", `tests/retained-cases/article/mariposa-2.yaml`), so only tool-cases are the gap.
Reproduced: confirmed by exhaustive reference search in the snapshot; no runtime.
Smallest fix: point a gate step at them (`scripts/trials.py --cases tests/tool-cases` after the main cases stage) or move their assertions into `tests/cases/`; either is a small scripts/cases change, minutes of gate time.
Timing: before release — dead cases are false trust, and the 0765 precedent shows they actively mislead during incident triage.

**[5] design gap | low** — The desktop/plugin release metadata is validated only by the full suite and the release workflow, never by the landing gate.
Evidence: `manifest.json` tool names are checked against server source declarations by `release/tools.py:31-34` (`check`), which runs only inside `make docs-check` (`Makefile:139`); the landing stages in `scripts/landing_gate.py:10-12` contain no manifest check, and `scripts/changed_tool_proofs.py` PROOF_INPUTS does not list `manifest.json`, `.claude-plugin/` or `server.json`, so a change there selects nothing. `release/workflow.py:120-125` does validate the plugin at staging. A tool rename that misses manifest.json lands through the routine gate and fails at full-suite or release time.
Reproduced: not reproduced (no build); traced through Makefile targets and selectors.
Smallest fix: read `manifest.json` in `scripts/check_docs.py` (already in pre-job) and compare tool names to the same source-declaration list, or add the files to the changed-selection rules; a few lines, seconds of gate time.
Timing: before release preferred (the manifest ships in the plugin payload); acceptable after, since the release workflow still catches it.

**[6] inconsistency | low** — Generated command pages speak two vocabularies: the clap `--help` block says "native"/"literal" and embeds "; help <command>" pointers, while the argument list below the same block rewrites those words to "source"/"exact" and strips the pointer.
Evidence: `docs/reference/commands.md:947-1010` (`search diagnostic`) — options block: "Native enforcement classification; FDA device recalls only." / "FDA recall class; help search diagnostic"; bullets one screen below: "Source enforcement classification…" with no pointer. Cause: the clap parser takes the raw schema description (`crates/biomcp/src/grammar/arguments.rs:33-38`), while the details renderer rewrites it (`crates/biomcp/src/help/discovery.rs:96-112` `plain()`: native→source, literal→exact) and drops the pointer (`crates/biomcp/src/help/argument_details.rs:130-145` `drop_pointer`). The raw multi-line descriptions live in `crates/biomcp-types/schemas/diagnostic-search.json` (`classification`, `source`, `type`, `name`). A terminal `--help` read therefore contradicts the packaged reference page beside it — the outcome tickets 0913/0923 (plain wording, say each thing once) were chasing.
Reproduced: not reproduced (binary not run); both renderings traced to their generators and visible in the checked-in generated file.
Smallest fix: apply the same `plain()` rewrite when setting clap help in `grammar/arguments.rs` (one call), then `make docs-generate`; minutes of gate time.
Timing: after release with reason — cosmetic wording only, no behavioral promise broken; bundle it with the next help-text ticket.

**[7] question | low** — Public `sdlc/` tickets name internal machines the repo's own checker classifies as internal everywhere else.
Evidence: `scripts/check_docs.py:26-31` stores hashed internal names, and I verified by recomputing the SHA-256 prefixes that three hashed words are internal host names (recomputed prefixes match) — yet `INTERNAL_SKIP` exempts `sdlc/` ("planning records are reviewed separately"), and public tickets say "evidence on an internal machine at `data/{repo}/…`" (`sdlc/tickets/0577…:12`), "on the the build machine at c821e374d" (`0750…:42`), "on the the build machine" (`0940…:42`). The whole `sdlc/` tree ships in the public repo (the release notes link into it as blob URLs). If that exemption is a reviewed acceptance, name it in a decision so reviewers stop re-deriving it; if not, scrub or generalize the machine names.
Reproduced: confirmed by hash recomputation and grep; the "reviewed separately" claim is asserted in a code comment only, not in a decision record I could find.
Smallest fix: one decision line accepting it (zero cost) or a small scrub pass before the history append (ticket 0294 freezes this text into public history permanently).
Timing: before the history append/publication — after that, the names are baked into public git history (stance 7 keeps history as it is).

**[8] risk | low** — The two changed-selection scripts disagree on the Makefile: a Makefile change runs every tool proof but no extra library test.
Evidence: `scripts/changed_tool_proofs.py:27` includes `Makefile` in `WORKSPACE_FILES` (selects every proof); `scripts/changed_library_tests.py:19-20` lists only `Cargo.toml`, `Cargo.lock`, `rust-toolchain*` and `.cargo/` — a change to the Makefile's test wiring (feature flags, target selection) selects no additional library tests in the landing gate. The gate does run the new Makefile itself, so a fully broken invocation still fails; the residual risk is a Makefile edit that changes how tests are invoked in a way that silently skips some.
Reproduced: not reproduced; selector tables compared directly.
Smallest fix: add `'Makefile'` to `WORKSPACE_FILES` in `changed_library_tests.py` and one expectation line in `scripts/test_gate_stages.py`; seconds of gate time only on Makefile changes.
Timing: after release with reason — low probability, and the full suite plus the release workflow's `make fmt lint test docs-check` still exercise the Makefile on a real runner.

## Strengths

1. **The gate tests itself.** `scripts/test_gate_stages.py` proves stage ordering, failure propagation, pre-job keep-going reporting, both changed-selection scripts against real temporary git repos with the true crate graph, coverage prerequisite reuse, and even the release checkout's history-scope guard against a planted credential fixture (`:214-262`). Very few projects meta-test their CI this seriously; the exit-trap/missing-tool failure modes I hunted for are explicitly covered.
2. **Secret hygiene is designed, not bolted on.** `release/secrets.py` pins and digest-verifies the scanner, lets only file/rule names cross the diagnostic boundary, enumerates reachable blobs (merge-only additions can't hide), refuses shallow checkouts, and the incremental baseline keeps removed/merge blobs (`scripts/test_gate_stages.py:262-296` proves it). Staged publication artifacts get a second scan before attestation.
3. **Generated docs cannot drift silently.** `reference_matches_grammar` (`crates/biomcp/src/grammar/reference.rs:291-301`) pins `docs/reference/commands.md`, `docs/recipes.md`, `docs/reference/source-pacing.md` and the skill's exit reference to the grammar; every example call in public docs is validated against the live server's tool schemas (`scripts/check_docs.py:validate_examples` + `first-read-check`), with first-read size ceilings measured per tool mode (`scripts/check_front_map.py`). The classic "help text lied about the flag" failure is structurally prevented.

## What I skipped, and why

- Running anything (cargo, python, network): forbidden by the review rules; every runtime-dependent claim above is marked "not reproduced".
- Deep audit of `release/gate.py` installed checks, `release/windows.py` VM flow, `rehearse.py` cold/mutation paths, and `notices.py` license assembly: adjacent-lane machinery, and the November 10 review already rehearsed the pipeline; time went to the landing gate and user-facing text instead.
- Enumerating the four unknown hashed internal words in `check_docs.py`: impossible without the plaintext; I verified only that three of the plaintexts match, which is enough for finding 7.
- Field-by-field verification of all 305 schemas against the 298-schema release baseline: the in-repo compatibility test owns that contract and I could not run it; my property-path diff found only additive changes under unchanged versions plus reviewed renames (e.g. `Reply_VariantPopulationQuery` → `…Echo`), which is compatible by the checker's rules (`crates/biomcp-types/tests/schemas/compatibility.rs:36-70`), so no version-bump finding is claimed.
- `check_release_channels.sh`/`check_hosted_section.sh` hardcoding `python3.13` against the Makefile's 3.11+ finder: they fail loudly when absent and are focused job scripts, not gate stages — not worth a finding.
- Clinical or biological correctness of any label, count or table: clinical-reviewer question, out of my lane.

## Acceptance

All twelve-cap slots unused beyond eight; volume held down deliberately. Nothing in the snapshot was modified.

---

# Lane E — Reliability and performance

Reviewed snapshot: `the pinned snapshot` = origin/main `586c602a`, pinned 2026-11-14. Read-only review: no build, no cargo, no tests, no network. **Every finding below is source-tracing only; none was reproduced at runtime.** Prior findings F1–F8 (2026-11-10 register), the 8 review stances (2026-11-06) and the no-backwards-compatibility decision (2026-11-01) were read first and are not re-raised.

## Answers to the lane's hunt questions (short form)

- **Does every outside call have a timeout?** Yes. Every HTTP wire call sets `timeout_global(Some(remaining))` (`crates/biomcp-fetch/src/wire_request.rs:163`), publisher downloads the same (`destination.rs:144`), gRPC calls set a deadline (`grpc/rpc_call.rs:32`), SQLite uses busy timeouts/absolute deadlines (`cache.rs:428`, `store_size.rs:37`), and DNS goes through ureq's resolver chain carrying the request timeout (`destination.rs:52-62`).
- **Which timeout wins?** Per-source leg = `min(now + spec.deadline, parent remaining)` (`lib.rs` `Fetch::leg`, `deadline.child(cap)`); command = `max(config budget, source command_budget)` (`declaration.rs:31-33`); hosted HTTP whole-call = `Cancellation::hosted` whose end lengthens to a slow source's declared budget (`cancellation.rs:82-88`, ticket 0912); plain HTTP and stdio have no whole-call scope — the command budget ends the work (documented, `docs/configuration.md:72`). `source_deadlines` is never populated by the product, so the per-source cap is fixed in code.
- **Is the NCBI rate-limit retry bounded, does it back off?** Yes. `wait_throttle` waits do not consume attempts but every wait is deadline-bounded, a `Retry-After` ≥ remaining budget exits early as `RateLimited`, and a zero/past Retry-After still waits `max(delay, backoff)` ≥ 250 ms (`request.rs:196-247`, `retry.rs:20-21,55`). Worst case a hostile/buggy server replying `429` + `Retry-After: 0` is asked roughly `budget / 250 ms` times (~50 within the 15 s ClinVar leg) — bounded, deliberate ("no refusal is hammered", `request.rs:236`).
- **Can one slow source starve others?** Slot-wise no starvation-loop: hosted HTTP admits `max_calls` (default 8), waits FIFO up to `queue_wait`, then returns a typed capacity error with retry advice (`mcp/http/call.rs:62-183`); a timed-out worker keeps its slot only until it actually stops (documented, `configuration.md:153`, and tested in `call.rs` tests). Fairness-wise see finding 4.
- **Panics on worker threads?** Contained: MCP tool work wraps `catch_unwind` (`mcp/worker.rs:4-6`), batch panics propagate only after all scoped workers join (`batch.rs:73-100`), gRPC worker panic closes channels to typed failures (`grpc/worker.rs:143-148`), release profile is `panic = "unwind"` (`Cargo.toml:20`). No caller-hang path found.
- **Does cancel stop descendants?** Cooperatively, yes: the flag is shared into every leg deadline (`Deadline::remaining()` → 0), quota condvar waits slice to 20 ms (`retry.rs` wait_slice, `limiter.rs:212`), and `cancel()` shuts down that call's isolated agents (`cancellation.rs:97-105`). Only an already-sent hedged read lingers until its own `timeout_global` (bounded, commented at `wire_request.rs:24`).
- **Cache growth/eviction?** Reply rows self-prune on every write (`cache.rs:301` deletes all rows past freshness+grace), limiter buckets expire after 300 s idle, bundles are capped (`MAX_BUNDLE_BYTES`, `auto_sync_max_bytes` 32 MiB). The gap is finding 1.
- **Cost caps?** Not applicable today: every source is a free public API whose keys are rate keys, not billing instruments (see Skipped).
- **Process-global mutation per request?** None found: no `set_var`/`set_current_dir` anywhere in `crates/`; the only signal handler is installed once at HTTP server start and startup refuses if it cannot be (`mcp/http.rs:107-116`).

## Findings (ranked)

**1. [design gap] Hosted reply cache has no byte ceiling, caches 28–32 MiB label pages, and hosted mode refuses `store clean`** — severity **medium** — A hosted operator cannot bound the disk the shared reply cache uses.
Evidence: `crates/biomcp-adapters/src/openfda_label/mod.rs:37-41` (`const PAGE_BYTES: u64 = 32 * 1024 * 1024` … `.fresh(Freshness::Days(1)).bytes(PAGE_BYTES)`); whole bodies are stored on every fresh write (`crates/biomcp-fetch/src/cache.rs:303-316`); expiry pruning happens only inside a write transaction or explicit cleanup (`cache.rs:301`, `cleanup.rs:10-11` "only when the caller explicitly requests cleanup"); hosted refuses `store clean` (`docs/configuration.md:139`, "Hosted mode refuses … `store clean`"). Each distinct drug-name search can pin ~30 MiB for a day; fresh entries are never evicted for size, and an idle hosted process retains even expired rows until the next write.
Reproduced: **not reproduced** (read-only; arithmetic from declared constants).
Smallest credible fix: a byte ceiling on the `reply` table enforced at write time (delete oldest-expiring rows over cap), or an operator-side hosted clean path; alternatively lower the label name-page bound. Code: one owner in `biomcp-fetch/src/cache.rs` (+ config knob); small; ordinary gate.
Timing: **before release** if the hosted/AWS rollout is still on the 1.0 path (plan says parked, not cancelled); otherwise first hosted scale-out, because the failure mode is silent disk exhaustion on a server that cannot self-clean.

**2. [risk] Label name-search holds several whole copies of a 32 MiB page per call; concurrent hosted calls can transiently allocate ~1 GiB** — severity **medium** — The process has a per-request body bound but no process memory budget.
Evidence: `openfda_label/mod.rs:19-37` documents 28 MiB live pages (quetiapine 15.0 MiB, 50-label pages); the wire read buffers the whole body (`wire_request.rs:11-22` `read_whole` → `Vec`), serde then materializes the parsed page, mapping clones rows into `NativeWindow`, and a cache read loads the body again (`cache.rs:191` `CASE WHEN length(body)<=?2 THEN body`). Hosted HTTP allows 8 concurrent calls (`config.rs:14` `DEFAULT_HTTP_CAPACITY: usize = 8`); stdio 32 (`config.rs:24`). 8 × (wire + parsed + mapped) of 30 MiB pages is ~0.7–1 GiB transient, plus WAL write amplification.
Reproduced: **not reproduced** (no build/run).
Smallest credible fix: cap concurrent large-page fetches (an admission class keyed on `byte_limit`, e.g. ≤2 in flight) or reduce the name page for uncached reads; alternatively stream-parse. Code: small-medium, confined to `biomcp-fetch` admission.
Timing: **after release**, with a measurement first — the bound is real but transient and per-request; measure RSS on a hosted load run (the existing `http/load_benchmark.rs` harness is the natural place) before adding machinery.

**3. [design gap] ClinVar gene counts scale with gene size against a fixed 15 s leg, sharing one 5-per-2 s bucket with PubMed** — severity **medium** — The shipped feature can honestly fail on genes it claims to count, and the user cannot raise the cap.
Evidence: `clinvar/mod.rs:39-40` (`deadline: Duration::from_secs(15)`, `command_budget: None`); `gene.rs:150-176` makes 1 einfo + 10 count searches + 5 status ID lists (+ extra pages per 10,000 IDs, `gene.rs:167`) + esummary pages of 200 (`gene_review.rs:15`); the NCBI keyless default is 5 requests per 2 seconds with `in_flight` 3 (`quotas.rs:65`), and PubMed shares the same bucket (`pubmed/mod.rs:35` `quota: &quotas::NCBI`). ~16 base requests ≈ 6.4 s alone; a status list over 10,000 records, or a concurrent PubMed search in the same process, pushes past 15 s → typed `DeadlineExceeded`. VAERS (60 s) and Monarch (30 s) declared `command_budget` for exactly this reason; ClinVar did not, and `source_deadlines` is never configurable in the product, so `command_timeout_seconds` cannot rescue it.
Reproduced: **not reproduced** (no network); pacing math from declared constants.
Smallest credible fix: give the ClinVar SPEC a `command_budget` (and matching leg deadline) sized to a 5-status worst case, or bound ID-list pages with explicit partial disclosure. Code: one declaration + tests; small.
Timing: **after release**, with measured page counts per gene — today's failure is a truthful typed error, not a wrong number, so it is a capacity/UX fix, not a correctness emergency.

**4. [question] Hosted HTTP admission has no per-caller limit: one caller's slow calls can occupy every slot** — severity **low** — Is per-caller fairness the wrapper's job or the core's?
Evidence: `mcp/http/call.rs:117-160` `turn()` admits strictly FIFO on a global `max_calls`; `State` carries no caller identity; per-key isolation exists only for rate pacing (`quota/admit.rs:16-21`, `Caller::Hosted(fingerprint)`), not for call slots. A single tenant issuing 8 concurrent VAERS calls (60 s budget each) capacity-blocks every other caller for up to a minute; they receive typed capacity errors with retry advice, so no hang — but no fairness either.
Reproduced: **not reproduced**.
Smallest credible fix: a per-fingerprint sub-cap (e.g. ceil(max_calls/2)) inside `Calls::turn`, or confirmation that the hosted front end already serializes per account (stance 3: judge hosted on the real path through the wrapper — if the wrapper enforces this, it is a docs line, not code).
Timing: **question first, before any hosted scale-out** — needs the hosting owner's answer; code cost is small if it lands in the core.

**5. [inconsistency] Stdio and plain-HTTP capacity refusals under-report when a slow-source call will actually free its slot** — severity **low** — The retry advice contradicts the declared slow-source budgets.
Evidence: `mcp.rs:305` sets every call's advertised end as `now + queue.budget` (the config `command_timeout_seconds`, default 20 s), and `refusal()` (`mcp.rs:248-257`) reports that as retry time; but a running VAERS/Monarch command executes up to its 60/30 s declared budget (`declaration.rs:31-33`, `vaers/mod.rs:24-25`), because stdio scopes have no `CallEnd` (`mcp.rs:217` uses `Cancellation::with_budget`, fixed end). A client following the advice retries up to 40 s early and can collect a second capacity error. The hosted HTTP path gets this right by reading the lengthening scope (`http/call.rs:162`).
Reproduced: **not reproduced**; confirmed by tracing.
Smallest credible fix: derive `ends` from `max(queue.budget, declared command budgets)` or record each call's actual start-time deadline. Code: a few lines in `mcp.rs` (+ `Calls` for plain HTTP); tiny.
Timing: **after release** — cosmetic timing advice; self-corrects on each retry.

**6. [design gap] `fan_out::run` spawns one thread per job with no cap, bypassing the MAX_BATCH_WORKERS discipline** — severity **low** — The codebase has a bounded batch primitive (`batch.rs:11` `MAX_BATCH_WORKERS: usize = 8`) and a second, unbounded one.
Evidence: `crates/biomcp-query/src/fan_out.rs:6-15` (`jobs.into_iter().map(|job| scope.spawn(job))`, no `min`). Current callers are bounded today (article search ≤6 sources `article/sources.rs:160-192`; normalize ≤3 services; health one thread per registered source, ~60 — `health.rs:12`), so this is exposure, not a live defect.
Reproduced: **not reproduced**; no runaway caller found.
Smallest credible fix: chunk jobs through a scoped worker pool (or reuse `Leg::batch` semantics) the next time a caller is added; alternatively an assert/limit on `jobs.len()`. Tiny.
Timing: **never as its own change** — fold it into whichever change next adds a fan-out caller; a lone refactor buys nothing measurable now.

## One-line notes on risks the docs/stances already accept (not findings)

- Keyless hosted buckets keep the source's full rate; `--rate-share` scales only keyed buckets — documented at `docs/configuration.md:91,153`.
- Plain HTTP has no whole-call deadline; the command budget ends the work — documented at `configuration.md:72` and `mcp/http/call.rs:234`.
- Hosted keeps timed-out workers until they actually stop (slots retained) — documented at `configuration.md:153` and deliberately tested (`call.rs` "expired running workers must retain all server slots").
- `attempts()` may send ~50 bounded requests within a budget when a source answers 429 with `Retry-After: 0` — deliberate, commented (`request.rs:236-237`).

## Hot path: one drug label read (`get drug <spl-set-id> --label-section indications_and_usage`)

1. Command budget 20 s default (`config.rs:171-177` `BUDGET`), `command_deadline` created (`drug.rs` selected_get → `openfda_label::get`).
2. Leg for `openfda_label::SPEC`, capped at 15 s from now (`Fetch::leg` `deadline.child(15s)`); OPENFDA quota admission (4/s keyless, `quotas.rs:141`).
3. Cache check on the credential-free identity (`retry.rs:73-80` canonical + sha256): a <1-day-old entry is loaded whole from SQLite, re-validated, filtered and decoded (`cache_request.rs:214-285`).
4. On miss: `GET api.fda.gov/drug/label.json?search=openfda.spl_set_id:"…"&limit=1&sort=effective_time:desc` with `timeout_global(remaining)`; on an empty result a second `set_id:` query (`openfda_label/mod.rs:96-131`). Single-label bodies are typically ≤ ~1 MiB (aripiprazole 574 KiB per the module comment), well under the 32 MiB endpoint bound.
5. `Record::label()` splits the heading's text into fixed 6,144-byte fragments and pages within the 28 KiB reply budget (`page.rs:19-52`); `label_bound.rs` trims the serialized reply under 32 KiB + 2 KiB.
6. Fresh result cached (upsert + global expired-row delete, `cache.rs:301-316`).

Work repeated per item that could happen once (all CPU-only, O(section bytes) each — cheap next to the network, but repeated): (a) `fragments()` re-splits the entire section text on every page continuation (`page.rs:63-91`); (b) `text_bytes()` re-serializes each part's selected text to JSON just to count escaped bytes (`query/drug/openfda_label.rs:107-118`); (c) each pagination step re-decodes the whole label page from cache. Multi-section requests share one read and one name resolution (`resolve.rs:24-115`, `selected_parts`), so no per-part network repetition — good.

## Strengths

1. **Timeout discipline is genuinely end-to-end**: one absolute `Deadline` flows into per-request `timeout_global`, publisher redirects, gRPC, SQLite busy handling, quota condvar waits and cooperative local parsing; the hedged second try is quota-admitted and the losing read is explicitly bounded by its own timeout. I found no outside call without a bound.
2. **Resource bounds are layered and mostly explicit constants**: 8 batch workers, 32/8 MCP capacities with bounded FIFO queues, HTTP admission slots separated from framing readers (health stays reachable), 64 KiB frames, 16 MiB/128-exchange recordings with atomic publish and symlink refusal, 1024/8 MiB observation reports, bounded gRPC spool/limits, bundle byte caps. This is unusually defensive for a CLI of this size.
3. **Failure containment is real, not aspirational**: panics are caught at the MCP worker boundary and after scoped batch joins, Busy cache contention degrades to `cache_problems` without failing answers, and stale fallback is surfaced (`report.stale`) rather than silently served.

## Skipped, and why

- **No runtime verification of anything**: the snapshot is read-only per the hard rules; every finding above is marked not reproduced. The existing `crates/biomcp-fetch/tests/retries.rs`, `http/load_benchmark.rs` and quota tests were read as evidence of intent but not run.
- **Cost caps before/after spend**: not applicable — all sources are free public APIs whose credentials are rate keys, not billed spend; the workspace api-key-risk decision (2026-11-11) already covers key risk. Revisit only if a paid source is ever added.
- **Clinical/biological correctness of counts, labels, PMA/recall/CDx tables**: clinical-reviewer question, not this lane's to judge.
- **Correctness items already dispositioned** (GWAS totals, pool snapshot eligibility, drug-name expansion, trial offset counting, key freshness, cache narrow readers/negative expiry, docs consolidation, nightly diagnostics): read, not re-raised.
- **DNS/proxy/bypass edge behavior** (`network/bypass.rs`), **gRPC streaming decode internals**, **bundle import verification**, and the **typed-tool schema/catalog path**: spot-checked bounds only; a dedicated pass would need more time than this lane had.
- **Git history**: not available in the snapshot, so "just landed" changes (GTR streaming, ClinVar counts, NCBI retry, recordings symlinks, PMA tables, label sections) were judged as the code stands.

---

# F — Maintainability and code quality

Lane of the adversarial review of BioMCP. Snapshot: the pinned snapshot = origin/main 586c602a5cf54b8b2be65f9c6986979682b21fa6 (2026-11-14), read-only. Nothing was built, run or network-tested; every runtime claim is marked not reproduced. Prior findings F1–F8 and the eight stances (2026-11-06) were read first and are not re-raised. Scanner skills deslop-code.md and structural-risks.md were read and their cheap checks were approximated with grep/python only (jscpd, cargo shear and cargo-machete could not run under the hard rules; findings below say so where it matters).

## Findings (ranked)

**1. [likely defect | medium] The live openFDA punctuation fix landed in one of six copies of the escaper, so four sibling sources still send characters openFDA rejects with HTTP 400.**
Broken promise: the same user input succeeds on the device-recall/PMA/510(k) name path but fails on drug recall, DrugsFDA, label and MAUDE paths.
Evidence: `crates/biomcp-adapters/src/fda_device_recall/mod.rs:44-50` — "openFDA answers HTTP 400 to these characters even inside a quoted phrase (live, 2026-11-13). Its tokenizer splits words at them, so a space finds the same words. `const REFUSED: &[char] = &[...]`" with `phrase()`. The old escaper that does NOT neutralize that set is still in force at `fda_drug_recall/mod.rs:36` (`fn literal(value: &Text)` — only backslash/quote), `drugsfda/mod.rs:88`, `openfda_label/mod.rs:53` (used on user text at `search.rs:33,49,104,118,127`), `device_search.rs:15`, and `maude/search.rs:4` (`fn phrase(field, text)`). All quote arbitrary user text against the same api.fda.gov backend. The missing shared home is visible: `device_search.rs:30,37` reaches into a sibling adapter (`crate::fda_device_recall::phrase`) because no shared owner exists.
Reproduced: not reproduced (needs live openFDA or a replay fixture; source-traced only).
Smallest credible fix: move `literal`/`phrase` into `crate::openfda` (the existing shared openFDA module) with the REFUSED set, adopt at all six sites, add one replayed 400 case per affected source. Cost: small code move, no new dependency; one normal gate.
Timing: before release — it is a user-visible failure on legal input, and the 2026-11-13 comment shows the behavior was already proven live for the sibling copy.

**2. [confirmed defect | low] The Python docs gate collects the case list and feature names but never compares them, while its success message claims agreement.**
Broken promise: a gate stage that says it validates "feature declarations" only checks format and duplicates; the real case-to-feature comparison lives in the Rust runner, so the Python side is sediment plus an overclaim.
Evidence: `scripts/check_docs.py:310-320` — `cases = {path.relative_to(root).as_posix() for path in (root / "tests/cases").rglob("*.yaml")}` is used only for a non-empty guard, and `names.add(match[1])` is filled but never compared to anything before the function ends; the closing print (line 337) says "Parity, feature declarations and architecture check references agree". The live comparison is `crates/biomcp/tests/runner/features.rs:4-33` (`validate(feature_list, trial names, complete)`).
Reproduced: confirmed by reading (dead variables are static facts); no execution.
Smallest credible fix: delete the unused `cases` set and either complete the case→feature comparison in Python or reword the success line to what the Rust runner owns. Cost: a few lines; gate time unchanged.
Timing: before release — trivial, and it is gate honesty.

**3. [confirmed defect | low] `winnow` is an unused workspace dependency, and the gate's scanner cannot see this class of unused dependency.**
Broken promise: "Every Rust routine gate fails on an unused dependency" (decision 2026-10-31, dependencies-are-few-chosen-and-used) — the gate runs cargo-machete, which checks member manifests against sources but not `[workspace.dependencies]` entries no member inherits.
Evidence: root `Cargo.toml:46` — `winnow = { version = "1.0.4", default-features = false, features = ["ascii", "alloc"] }`; no crate Cargo.toml mentions winnow and no source references it (tree-wide grep; only mention is a comment in `crates/biomcp/tests/ratchets/dependencies.rs:8`). The gate is `Makefile:225-227` — `unused-dependencies: install-machete; cargo machete`. The deslop skill prescribes `cargo shear` for exactly this. Related dead code: `dependencies.rs:11` — `("biodata", &["serde", "winnow", "schemars"])` is the first LAYERS entry, but `check()` skips it because `crates/biodata/Cargo.toml` does not exist (biodata is an external git dependency), so that layer is unreachable and its winnow mention suggests the declaration was believed necessary.
Reproduced: confirmed by static cross-check (all member manifests + sources); cargo shear itself not run (hard rule: no cargo).
Smallest credible fix: delete the winnow line and the unreachable biodata layer entry; optionally add cargo shear to `unused-dependencies`. Cost: one-line deletion plus a small Makefile edit; one gate run.
Timing: before release — zero-risk deletion that directly serves the few-chosen-used decision.

**4. [confirmed defect | low] Two public constructors in biomcp-types have no caller anywhere in the tree.**
Broken promise: with no backwards-compatibility promises (decision 2026-11-01), public API should be alive; dead constructors are unreviewed surface nothing gates (unreachable_pub and dead_code lints do not catch unused pub items in a library crate).
Evidence: `crates/biomcp-types/src/query/gene.rs:152` — `pub fn in_region(` ("Select ClinGen dosage regions overlapping a GRCh38 interval") has zero references in any crate src, test or tools; `crates/biomcp-types/src/response/usage.rs:123` — `pub fn argument_values(` likewise zero references (sibling constructors `new`, `filtered`, `regions_only` are used).
Reproduced: confirmed by tree-wide text search including tests and tools; no compile-based unused check run.
Smallest credible fix: delete both constructors (tests of siblings unaffected). Cost: trivial; one gate run.
Timing: before release — trivial hygiene, and Ian's ninth-call direction prioritizes cosmetic cleanups.

**5. [design gap | low-medium] The openFDA page-envelope contract exists in three copies with already-visible drift.**
Broken promise: one source's reply contract should have one owner (structural risk: truth restated); `crate::openfda::Envelope<T>` was built as that owner, but two endpoints re-declare the whole Page/Absent shape.
Evidence: shared owner `crates/biomcp-adapters/src/openfda.rs:12-58` (`enum Envelope<T>` with `data_date`, `check`, `check_status`, `Item::counts`); copies at `faers/counts.rs:26-54` and `adverse_event_sources/received.rs:27-56`, each re-declaring the untagged `Page/Absent` enum, `data_date`, and the identical `check_status` arm `(Self::Page { .. }, 200) | (Self::Absent { .. }, 404)`. Drift: both copies drop `Item::counts` (the shared envelope's row-count verification) — defensible for count endpoints, but now the contract "a valid openFDA page" is decided in three places; a future change to the absence or status contract must be fixed three times (the F2/F3 family of bugs in the prior review came from exactly this class of drift between adapter copies).
Reproduced: confirmed by reading; no behavioral reproduction claimed.
Smallest credible fix: parameterize `Envelope` with a per-item check hook (the `Item` trait already has one) and fold both copies into it. Shared owner: `crate::openfda`. Cost: small refactor, existing tests replay both endpoints; one gate run.
Timing: after release — no wrong output today; do it while touching these endpoints next (cluster 1's fix touches the same module and could carry it).

**6. [risk | low] XML attribute extraction is copy-pasted per source with semantic drift in empty-value and match-order handling.**
Broken promise: parsing the same kind of trusted-boundary fact (a source XML attribute) should mean the same thing everywhere.
Evidence: shared helper `crates/biomcp-adapters/src/xml_element.rs` (`Element::attr` — first match wins, no trim); `gtr/xml.rs:203-215` `fn attr` — trims the value and maps empty to `None`; `wikipathways/gpml_raw.rs:31-46` `fn attribute` — no trim, keeps empty strings, and iterates all attributes so the LAST duplicate wins where gtr takes the first.
Reproduced: not reproduced (behavioral difference inferred from code; no execution).
Smallest credible fix: expose one shared `BytesStart`-level attr helper in `crate::xml_element` and adopt in both streaming parsers, choosing one documented empty/duplicate policy. Shared owner: `crate::xml_element`. Cost: small; one gate run.
Timing: after release — drift is real but bounded to two sources' optional fields; bundle with the next parser change.

**7. [inconsistency | low] openFDA quota is shared through one const, but the identical openFDA Terms are restated inline in five SPEC declarations while a shared const already exists for a sixth.**
Broken promise: facts about the same publisher should be declared once (structural risk: truth restated); the codebase itself already demonstrates the right pattern two lines away in each file.
Evidence: `quotas::OPENFDA` is referenced by 8 SPECs (`quotas.rs:141`), but the identical `name: "openFDA terms and license; third-party exceptions"` + `url: "https://open.fda.gov/terms/"` pair is restated at `fda_device_recall/mod.rs:16-18`, `fda_510k/mod.rs:13-15`, `fda_pma/mod.rs:12-14`, `fda_drug_recall/mod.rs:16-18`, `adverse_event_sources/mod.rs:12-14`; meanwhile `drug_sources/mod.rs:20-24` already defines a shared `pub(crate) const FDA_TERMS` (with its own label-rights wording) — the pattern exists but is not applied.
Reproduced: confirmed by grep.
Smallest credible fix: hoist one shared FDA terms const beside `quotas::OPENFDA` and reference it from the five identical SPECs. Cost: trivial; one gate run.
Timing: before release with any nearby edit, else after release — pure restatement risk (a terms-URL change currently needs five edits).

**8. [risk | low] The wire literal "recall-records" is defined in two independent enums across the two FDA recall domains.**
Broken promise: one native fact (what a recall search's total counts) should have one wire definition; today two typed enums own the same literal.
Evidence: `crates/biomcp-types/src/drug/recall.rs:60` — `DrugRecallCountUnit ... RecallRecords => "recall-records"` (canonical_enum, drug domain) and `crates/biomcp-types/src/response/diagnostic.rs:255-257` — `DiagnosticCountUnit { ... RecallRecords => "recall-records" }` (wire_enum, diagnostic domain, used by the device-recall search). A future rewording of one literal diverges the two recall tables' schemas silently.
Reproduced: confirmed by grep.
Smallest credible fix: have one enum reuse the other's variant (or share a consts module) so the literal exists once. Cost: trivial; one gate run.
Timing: after release — cosmetic dedup with no behavior change; note the schemas themselves stay distinct.

**9. [question | low] FDA table field typing still diverges across the just-unified tables: PMA/510(k) dates stay raw strings while recall dates are parsed.**
Broken promise (candidate): "field names across the FDA tables were just unified" (help.rs:301, "(0959)") — names now agree, but the types behind same-named native dates do not.
Evidence: `crates/biomcp-types/src/response/diagnostic.rs:347-349` — `decision_date: Option<Box<str>>` ("including explicit empty") on `DiagnosticPmaHeader`, and `DiagnosticClearance` the same; while `response/device_recall.rs:24-25` — `initiated: Option<PartialDate>` and `drug/recall.rs:38-40` also `Option<PartialDate>`. Same publisher, same kind of native date field, two representations. The "explicit empty" doc comment suggests it is deliberate for openFDA sheet cells, but nothing records that ruling against the recall tables' opposite choice, and it changes what schema consumers can assume.
Reproduced: not applicable (design question, no runtime claim). Judging whether raw-string dating is clinically right is a clinical-reviewer question, not mine.
Smallest credible fix: either a one-line recorded reason beside the PMA typing, or unify on one representation. Cost: a decision plus at most a small refactor.
Timing: before the next schema cut if the answer is "unify"; otherwise record the reason now.

**10. [risk | low] Ticket-number narrating comments in product code contradict the workspace deslop rule, and no ruling records an exception.**
Broken promise: deslop-code.md — "Narrating comment: does a comment ... record history, such as a ticket number? Delete it. Keep comments that say why."
Evidence: 37 occurrences of `(0NNN)` ticket references in `crates/*/src` (e.g. `biomcp-types/src/response/envelope.rs:38` "// The page comes before the result (0939)", `biomcp-fetch/src/record.rs:323` "(0954)", `biomcp-adapters/src/gtr/bundle.rs:33` "(0931)"). The comments' why-parts are good and should stay; only the history token is at issue.
Reproduced: confirmed by grep count.
Smallest credible fix: strip the parenthesized numbers (keep the rationale), or add a one-line ruling that traceability numbers are wanted. Cost: mechanical sed-level edit across ~32 files; one gate run.
Timing: after release — pure hygiene; safe to batch. If these numbers are an intentional convention, the missing piece is the ruling, not the edit.

## Strengths

1. **Layering and dependency discipline are real, not aspirational.** biomcp-types' runtime deps are exactly `biodata serde schemars`; adapters add only parsing crates; clap exists only in the top biomcp crate; no crate imports the binary crate. The LAYERS ratchet (`ratchets/dependencies.rs`) denies any dependency outside each layer's allowlist, and deny.toml pins licenses and git sources. The one violation found (winnow, finding 3) is a declaration with zero reach into any build.
2. **Restatable truths are generated and gated, unusually consistently.** Command reference, recipes, source-pacing doc and the SKILL.md exit block are generated from the grammar and compared by `make docs-check` (`grammar::reference::reference_matches_grammar`, Makefile:146); ROADMAP's parity ledger and the upgrade block are generated and compared by check_docs.py; error codes and messages live in one macro-defined file (`response/error.rs`); paged replies come from one `page!` macro (35 uses) that enforces totals-before-rows, with a dedicated test (biomcp-types/tests/response.rs:366).
3. **Test quality is genuinely outside-in where it matters, with one shared isolation helper.** Sampled: `runner/label_priority.rs` drives both MCP tool modes through replayed fixtures and asserts observable envelope fields; `adapters/tests/ctgov_refusals.rs` mutates recorded native bodies at the trust boundary to prove refusal semantics; `runner/diagnostic_cdx.rs` seeds a synthetic stored corpus. All child-process tests isolate through `biomcp_test_support::environment` (structural-risks #3 satisfied); the required-surface coverage ratchet derives its inventory from shipped help and tools/list rather than a hand-kept list, and its pending-gap ratchet has closed to empty (shrink-only enforced).

## Skipped, and why

- **Exact ratchet arithmetic (parents-with-logic 170 cap, module-docs 410 cap):** my read-only approximation puts reality at roughly 147 and 347, i.e. ceilings hold with headroom and no offender exists; the gate's own tokenizer was not run (no cargo), so exact numbers are not reproduced. Not a finding: caps match reality.
- **jscpd, cargo shear, cargo-machete, clippy:** could not run under the hard rules (no cargo, no network, no builds). Clone detection was approximated with a local python n-gram scan over the adapters (12- and 8-line windows); unused-dependency checking was reproduced with manifest/source cross-greps, which found the winnow entry a machete-shaped gate would miss.
- **File-size caps:** the only enforced file cap found is 600 lines for `scripts/*.py` (check_docs.py:322); Rust file size is convention only (largest src file 600 lines; functions capped at 80 by clippy with a documented threshold). No violation to report.
- **Clinical or biological naming/typing judgments** (e.g. whether raw-string dates on PMA are right): named as a clinical-reviewer question in finding 9 rather than judged.
- **Runtime behavior of the openFDA 400 drift (finding 1) and any live-source verification:** hard rule forbids network; kept as source-traced, not reproduced.
- **Prior findings F1–F8 and stance-accepted risks** (loopback placement, synthetic captures, README-vs-docs policy split, etc.): read and excluded per instructions; no compat code found that breaks the no-backwards-compatibility stance (all "legacy/deprecated" hits are upstream native field names).
- **Error-text duplication sweep:** user-facing error literals are macro-centralized; the worst internal duplication found was a 3x short io::Error string — below finding threshold, dropped per deslop verdict rules.
