# 0113 — Match a hotspot row only on the same transcript and residue

Status: OPEN.

Milestone: 2

Depends on: 0105

## Outcome

Clinical source review found no retained consequence tied to a hotspot row's same versioned transcript. This ticket closes without code under the approved review-first outcome. Hotspot rows keep their residue and alternate counts as candidates; 0110 can prove a positive candidate row.

- Retained source facts: `0009-hotspots.body` states BRAF, `ENST00000288602`, V600, alternate E count 833 and total 897. `0099-hotspots.body` states EGFR, `ENST00000275493`, L858, alternate R count 144 and total 144. Both transcripts lack versions. Independent MyVariant transcript, HGVSp and position arrays do not establish a consequence association.
- Evidence: Yellow `data/biomcp-redux/0078-variant-investigation/`, with request provenance in `requests.json`. The bodies were untruncated HTTP 200 captures on 2026-10-24. BRAF SHA256 `e234c470f630767bb9c086d6c71a6226c83ef8655b1485a0a25f307b92a4279a`; EGFR SHA256 `126f2190b047e33e4fa8b333699ab2f10636f26c7933295621e3d315b85b575d`.
- Source model: the publisher's [HotspotMutation](https://github.com/cBioPortal/cancerhotspots/blob/master/model/src/main/java/org/cmo/cancerhotspots/model/HotspotMutation.java) and [Transcript](https://github.com/cBioPortal/cancerhotspots/blob/master/model/src/main/java/org/cmo/cancerhotspots/model/Transcript.java) expose a transcript string without a separate version. [MyVariant object documentation](https://docs.myvariant.info/en/latest/doc/data.html) supplies no missing association. [Ensembl identifier rules](https://jun2026.archive.ensembl.org/info/genome/stable_ids/index.html) do not justify inventing a historical version.
- Scope: no code, builds or provider requests were made for this review. An exact match would need retained source evidence for the same versioned transcript, gene, reference residue, position and alternate. No transcript version or array pairing is inferred.
