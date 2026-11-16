Synthetic: orphadata

The two Orphadata files are authored, not captured. They keep the shape of Orphadata's product 9 XML (header, `Licence` element, one `Disorder` element per code) and state `dbserver="synthetic"`; every name, estimate, source and PubMed ID in them is invented. Orphanet code 900001 is invented. Code 2322 is the code MONDO:0016512 links to, kept so the MONDO join can be tested; its name and rows are invented and say nothing about the real disorder. PubMed IDs 90000001-90000003 are above any ID PubMed has issued.

The two MyDisease exchanges for MONDO:0016512 and MONDO:0007254 are unchanged public replies recorded live on 2026-11-13 by `biomcp --record` (MONDO CC BY 4.0, Disease Ontology CC0).

Real Orphadata bytes do not belong in this repo (decision `sdlc/decisions/2026-11-16-review-follow-ups.md`, item 1); `crates/biomcp/tests/ratchets/noncommercial_replies.rs` fails on an Orphadata recording without this marker or without `dbserver="synthetic"`.
