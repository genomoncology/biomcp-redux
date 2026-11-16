Synthetic: orphadata

The two Orphadata files are authored, not captured. They keep the shape of Orphadata's product 9 XML (header, release date and version, `Licence` element, one `Disorder` element per code) and state `dbserver="synthetic"`.

Invented: every disorder name (each starts with "Synthetic"), Orphanet code 900001, every prevalence estimate and geography, and PubMed IDs 90000001-90000003, which are above any ID PubMed has issued. Code 2322 is the code MONDO:0016512 links to, kept so the MONDO join can be tested; its name, estimates and geographies are invented.

Kept from real cases for their shape: disorder 900001's sources cite the real PubMed IDs 18442953 and 33080481, in the two source spellings Orphadata uses. Code 2322's disorder type (Malformation syndrome), its three onset ages and its two inheritance modes are copied from Orphanet's real entry for that code, Kabuki syndrome.

The two MyDisease exchanges for MONDO:0016512 and MONDO:0007254 are unchanged public replies recorded live on 2026-11-13 by `biomcp --record` (MONDO CC BY 4.0, Disease Ontology CC0).

Real Orphadata bytes do not belong in this repo (decision `sdlc/decisions/2026-11-16-review-follow-ups.md`, item 1); `crates/biomcp/tests/ratchets/noncommercial_replies.rs` fails on an Orphadata recording without this marker, or on any Orphadata file, inline or in a sidecar `.bin`, that lacks `dbserver="synthetic"` or names a disorder that does not start with "Synthetic".
