# 1066 — label-section-change-history

Status: COMPLETE.
Landed: b0c519523f7f6cb0395e0c850cdd51c4d57a4066

Milestone: 7

## Outcome

A caller can ask BioMCP when a named section of a drug's US label changed and what the change added, in one call per drug: for example, which approved labeling change first put major adverse cardiovascular events in a boxed warning, with FDA's approval date, the supplement number, and the sentence that was added. The answer comes from FDA's own archived labels and letters, and each fact names the document it came from.

## Why

Agents asked "when did this warning enter the label" spend most of their tokens downloading archived label PDFs and approval letters and reading them by hand. In one traced run on 2026-11-19, 16 of 42 turns and about half of 1.8 million tokens went to this one job, after BioMCP's approval history had already given the dates. A second task, dating when three risks entered three labels, shows the same pattern. BioMCP's approval history says when a labeling change was approved but not what changed.

## Proof

- Proof: replayed cases where the call returns, for Olumiant, Xeljanz and Rinvoq, that major adverse cardiovascular events entered the boxed warning in the labeling change FDA approved on 2021-12-02; for Aimovig, that Raynaud's phenomenon entered warnings and precautions in the change approved 2025-03-21; and for Livalo, that myasthenia gravis entered the postmarketing adverse reactions in the change approved 2024-03-14. Each fails on the base commit, where no such call exists.
- Proof: a term that never appears replies that plainly, and a label version BioMCP could not read is named as unread, never skipped silently.

## Starts from

- Starts from: `get drug NDA... approvals --source drugsfda`, which already returns each supplement's date, class, `supplement_purpose` and the links to FDA's letter and label PDF (`application_docs`). The archived label PDFs are at accessdata.fda.gov under those links. FDA's site answers some requests with an "excessive requests" page unless a browser user agent is sent; agents hit this on 2026-11-19.
- This ticket starts with a design, not code. The developer first reports to the PM, in at most two pages in the ticket's branch under `sdlc/planning/`: the sources tried (Drugs@FDA label PDFs, FDA letters, DailyMed's label version history) and what each can and cannot give; how section text is taken from a PDF and how reliable that was on the five drugs in the Proof and on ten others picked blind; the command's shape and its reply; the reply's size; what is fetched and stored, and how much; and the cases where it will be wrong or silent. The PM rules on the design before the build.
- PM decision, 2026-11-19: the question the command answers is "in which approved label version does this text first appear in this section". It reports what the documents say and does not judge why a change was made.
- PM decisions, 2026-11-19, on the design note `sdlc/planning/label-section-change-history.md`:
  1. Source and shape: Drugs@FDA label PDFs with the existing lopdf reader, the halving search, the sentence-only reply, and the command `get drug NDA... label-history --source drugsfda --label-section S --find TEXT`. DailyMed and letter reading stay out of this version.
  2. First step: run BioMCP's own reader over the design's 80 PDFs and compare with the design's figures (sections found, text order, time per label). If it reads materially worse, or cannot meet the per-call time limit, stop and report with the numbers before building further.
  3. Page limit: raise or lift the 100-page refusal for label PDFs so the 202-page Keytruda and 191-page Opdivo labels are read; say what bound was chosen and why.
  4. The reply says how many label versions exist, which were read, and that the answer assumes text once added stays. `--every-version` is in this ticket if small; otherwise help says it is absent. An unread version is named with its reason, and if one sits just before the first match the reply says "on or before".
  5. `--find` may repeat and matches any of its values in any letter case. Help says matching is literal text.
  6. Fixtures: no 50 MB of PDFs; new recorded data stays under about 10 MB. Test PDF reading on a few small real label PDFs, and the search and reply logic on recorded section text for the Proof drugs, Aimovig included, with provenance naming each source PDF and its checksum. If Aimovig cannot be shown end to end within that size, show it live in a pm job.
  7. Reply size is a requirement: the design's 1.5 KB mock is the target, and a case fails if a Proof drug's reply passes 3 KB.
- PM decision, 2026-11-19: reply size matters as much as correctness here, because the caller is an agent paying per token. The reply gives the version, date, supplement and the matching sentence, not whole label sections.

## Keeps

- Keeps: BioMCP downloads and reads documents; it installs and runs no other tool automatically. If reading PDFs needs a library, the design names it and its license.
- No fact is given without the document it came from. A document that could not be fetched or read is reported as such.
- `approvals` keeps its present reply.
- Label PDFs are fetched only from https://www.accessdata.fda.gov, as approval letters are: the download's source declaration admits no other origin, so a PDF the reader parses can come only from FDA's own site, and each read runs in the capped child process (1075) (PM, 2026-11-19).

## Defers

- Defers: labels outside the US. Whole-label diffs between two versions. FDA's drug safety communications, which have their own ticket.

## Changes

- Changes: a design note under `sdlc/planning/` first; then the drug commands, a Drugs@FDA document reader, cases and help.
