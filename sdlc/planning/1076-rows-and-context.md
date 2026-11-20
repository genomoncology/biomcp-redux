# Which reply fields are worth moving to the list?

## PM decision, 2026-11-20

Use the existing `regulatory` section for FDA context from an Open Targets drug. The indications reply points to it. Move only provenance and identity shared by construction for the selected drug, UniProt entry or versioned GTR test. The rankings and modeled alternatives below remain evidence for later work.

## Original scan and proposal

Sharing the fields below saved 214,441 bytes across 106 recorded replies in a reversible model. Each original row could be reconstructed exactly. FDA regulatory context needs a separate decision about how an application identifies its products.

| Modeled list and shared fields | Replies / distinct cases | Net bytes saved | Bytes per reply |
|---|---:|---:|---:|
| Variant conservation scores, subject and attribution within each candidate allele | 15 / 3 | 71,685 | 4,779 |
| Cell-line cross-references, category and namespace | 48 / 1 | 69,840 | 1,351–1,663 |
| FDA submissions, event record and attribution | 12 / 5 | 41,512 | 873–13,943 |
| Protein features, attribution, protein ID and source coordinates | 5 / 1 | 17,368 | 1,870–9,888 |
| Article search, open-access source and licence-not-checked flag | 26 / 5 | 14,036 | 496–1,636 |

## What the scan found

Job j43664 passed all 1,119 public replay cases and runner tests on main `2e63f2cd8`. Of 8,056 structured replies, 7,109 were BioMCP envelopes. Of 1,311 object lists with at least ten rows, 862 contained a constant field. The inventory gives each reply's size, matched paths, row count and bytes saved. It includes lists nested in rows and operational replies. [Full ranking](1076-ranked.tsv) · [Per-reply measurements, gzipped TSV](1076-measurements.tsv.gz)

The largest result-list totals, moving *every* constant, were gene search (73,131 bytes), conservation scores (71,685), cell-line cross-references (69,840), diagnostic search (52,152), FDA submissions (48,476), trial results (38,205), FDA application search (31,971), article search (19,162), pathway search (19,104) and protein features (17,998). The narrower recommendation keeps individual match, location, status and clinical facts explicit. Two of the four gene cases repeatedly exercise queues or reply size. All 48 cell-line replies are from the K-562 page walk. MCP discovery had 436,366 potential bytes across 280 replies, but moving fields out of individual tool definitions would change the MCP client contract.

Outside the rows, 2,130 of 3,460 row-bearing replies crossed 25%. Another 4,596 had no object rows. Requested drug label text (up to 27,723 bytes per reply) and trial source outcomes (up to 15,796) are examples of facts worth keeping. The one replay where FDA context exceeded a quarter used 989 of 2,657 bytes. The larger live pages below show its cost.

## Ticket examples on Linux build `5db190f8a783`

“After” moves only the named fields to a sibling `row_defaults` object. Sizes exclude the CLI newline. The GTR result was marked stale. The replay corpus had no ten-row example of the GTR gene or indication-row savings measured here.

| Command after `biomcp` | Rows | Shared fields | Bytes before → after |
|---|---:|---|---:|
| `protein features P15056 --limit 50` | 50 | `attribution`, `feature.protein`, `source_coordinates` | 21,833 → 11,523 |
| `get diagnostic GTR000509442.12 genes --source gtr --limit 50` | 50 | `test_id`, `test_type` | 13,804 → 11,322 |
| `search article 'AKT1 E17K' --limit 50` | 50 | `open_access_source`, `license_not_checked` | 24,795 → 22,019 |
| `get drug imatinib indications --source opentargets` | 10 | `drug`, `attribution` | 18,506 → 16,480 |
| Same indications command with `--limit 50` | 50 | Same two fields | 64,809 → 53,703 |

**FDA context model.** The default indications reply spent 12,566 bytes (68%) on FDA context. At 50 rows it spent 42,086 (65%). Moving constant application fields alone saves 2,242 and 4,537 bytes respectively. The model stored each distinct FDA product once inside `regulatory_context`, with ordered product indices per application and shared defaults for applications, products and submissions. The caller pays one in-reply lookup per product. The dated events, submitters and notice that the indications are unlinked stay in the same reply, as the source-age comparison case requires.

Applied only to FDA context, the model reduced it to 5,275 of 11,215 bytes (47%) and 21,995 of 44,718 (49%). Both expanded to the exact original FDA object. Shortening the indication rows would change those percentages. [FDA size model](1076-regulatory-context.py)

## Original proposal for reply shape and proof

Put defaults before their list, scoped to that page. A row inherits an approved field by its exact path and carries its own differing value. If any row lacks the field, leave that column explicit so absence, false and an unchecked licence stay distinct. Use defaults only where they save bytes. Apply `--fields` to effective rows, keeping required attribution, identifiers and pivots.

Bump each changed reply schema's major version and document inheritance. For each list, start with a replay case that fails on the base. Assert the shared field appears once, overrides survive, expanded rows match in order, and the validated reply shrinks by the measured amount. Cover a later page and a narrowed reply.

## Reproduce the measurements

Run `make cases` with `BIOMCP_TRANSCRIPT_DIR` set to an empty directory, then run the commands below. The scanner measures compact UTF-8 JSON. A constant object includes its children. Each field estimate pays for its own defaults object. The combined estimate shares that cost. The quarter test subtracts the outermost requested section lists, or the outermost result lists, and counts independent FDA context outside indication rows. Overlapping estimates must be read separately.

```sh
python3 sdlc/planning/1076-measure.py "$BIOMCP_TRANSCRIPT_DIR" > measurements.tsv
python3 sdlc/planning/1076-measure.py --rank < measurements.tsv > ranked.tsv
```

Coverage is limited to captured structured replies. Of 1,118 transcript markers, 106 flagged missing or partial capture. One passing test wrote no marker. There were 69 process outputs without parseable JSON. Seven retained cases requiring external fixtures are outside the public suite.
