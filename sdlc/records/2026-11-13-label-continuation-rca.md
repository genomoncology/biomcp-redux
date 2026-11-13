# Why agents read the first page of a label section as the whole section

November 13, 2026. Root-cause note for ticket 0882 (label section pages say they continue), as `skills/rca.md` asks when a failure repeats across tickets.

## The cases

1. pv-scientist, bench run 0040: `get drug pembrolizumab label --label-section warnings_and_cautions` returned fragment 0 of 4. The agent printed only the first 2,500 characters of `result.label` and, for the offset call, the first 400 bytes of the reply, so it never saw `total` or `next` (`sdlc/records/2026-11-12-persona-rerun-failures.md`, regression (a)).
2. reg-affairs, bench run 0041 on b87cb3b3e: `get drug KEYTRUDA label --label-section indications_and_usage` returned fragment 0 of 4 (6,128 characters). The agent listed `label.data` as id, version, revised, url, heading and text, and reported "subsections 1.1 … 1.10" as the section (`personas/reg-affairs/runs/2026-11-13/log.md:192-201` in biomed-agent-bench). The raw-tools agent read the same section whole from DailyMed, through 1.21.

Both are the same failure: the reply was right and carried its paging, and the agent stopped reading at the end of the text.

## Why

- Why did the agents miss the continuation? Every signal that the section goes on came after the text: in `label.data` the fields `offset` and `total` followed `text`, and the envelope's `page` and `next` follow the whole result. An agent that reads or prints the text first, as both did, reaches a long passage that looks complete before it reaches any of them.
- Why does that matter for label sections in particular? Label text is cut into 6,144-byte fragments, and many sections are larger. Measured on 2026-11-13 from openFDA (compact JSON-escaped bytes of the whole field):

| Label | Indications and Usage | Sections over 6,144 | Largest section |
| --- | --- | --- | --- |
| KEYTRUDA 9333c79b v229 | 23,683 | 7 of 27 | clinical_studies 209,576 |
| OPDIVO f570b9c4 v68 | 12,307 | 9 of 26 | adverse_reactions 135,472 |
| GLEEVEC 211ef2da v41 | 3,803 | 7 of 25 | clinical_studies 40,315 |
| XARELTO 10db92f9 v65 | 4,281 | 8 of 30 | clinical_studies 42,774 |
| TAGRISSO 5e81b4a7 v37 | 3,210 | 8 of 26 | adverse_reactions 29,230 |
| HUMIRA 608d4f0d v2154 | 3,902 | 10 of 29 | clinical_studies 52,568 |
| ZESTRIL 838c2d78 v2 | 6,450 | 7 of 27 | clinical_studies 14,798 |

So a quarter or more of every label's sections are paged. A break can fall mid-sentence wherever the byte budget ends, or, worse, at a clean sentence end: openFDA stores Zestril's 6,450-byte indications as four array elements, a reply returns one fragment by default, and the first ends "… beta-blockers [see Clinical Studies (14.3) ] ." with three to go (jobs j39041 and j39048). Nothing in the text says it stopped early.

## Fixes considered

1. **A larger fragment.** The label section allocation in a reply is 8,192 bytes (`crates/biomcp-query/src/drug/label_bound.rs`, `LABEL_BYTES`; drug slice design, "Dosing, indications or full warnings page"). The most the allocation can grow without a PM ruling is 50 percent (`sdlc/decisions/2026-10-26-measured-entity-caps-need-no-ask.md:7`), about 11 KB of text. That would cover Zestril's indications but not Opdivo's (12,307) or Keytruda's (23,683), and it would do nothing for the sections of 30 KB to 210 KB that must page anyway. It does not remove the cause.
2. **Put the continuation before the text (chosen).** One optional field, `remaining`: how many fragments of the section follow this reply, absent when the reply ends the section. It sits before `text` in the command-line reply, and its name sorts before `text` in the MCP reply, whose keys are alphabetical. An agent that reads `label.data` from the top meets `"remaining": 3` before the first word of label text. No wording is added to the text, which stays verbatim, and the reply grows by about 16 bytes. Measured in job j39041 on cb204d5ca: Keytruda's indications give `remaining` 3, 2, 1 and then no field across offsets 0 to 3, the last fragment ending at 1.21; replies are 7,838 bytes at most, inside the existing allocation.

A smaller follow-up is not done here: the default of one fragment a reply means a section split into short native elements, like Zestril's, takes four calls although all of it fits the 8,192-byte allocation.

## Reach

The same layout, paging fields after the text, is used for FDA PDF text (SSED, approval letters) through `crates/biomcp-adapters/src/pdf/text.rs`. Those replies were not part of either failure; the ticket defers them. Nothing here lives in a shared skill, pm or a machine, so there is nothing for Jarvis.
