# Source-stated drug aliases

All replies here are authored synthetic provider-shaped examples. Names, cross-source identities, counts and timestamps do not describe an actual drug. They distinguish native ChEMBL synonym matching from alias authority and independent selected-source discovery, and preserve actual continuation spelling.

The `ddinter/` files copy the synthetic DDInter partitions from `drug/ddinter-synthetic` and add one invented pair, DDInter9001 with DDInter9030 "Canonical therapy", so a DDInter partner given as EX-66 resolves through the same aliases.

The openFDA label name lookups for "Orphan code" and "Ambiguous therapy" return the HTTP 404 no-match envelope.
A label lookup that cannot read MyChem's names has every other source current (1087).
The synthetic Coumadin lookup returns no match from MyChem and the case disables the other alias sources to exercise the unresolved DDInter partner notice.

Synthetic: ddinter
