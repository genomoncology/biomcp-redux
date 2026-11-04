# BioMCP's biodata builds its own parts

Date: 2026-10-24

Decided by Ian.

BioMCP keeps and builds its own biodata crate. Another team builds a biodata crate of its own in parallel, on different parts, including full variant notation (HGVS, alterations and cytogenetic notation). BioMCP takes no dependency on that crate now. Merging the two into one public crate comes later and is not planned soon.

BioMCP builds only its own parts. When a slice needs a part the other team owns, the PM raises it before building a copy. For the variant slice this means BioMCP passes HGVS and other variant notation to its sources as checked text, lets each source resolve it, and keeps only the small protein-change parse its article readers already use. A full notation parser needs a new ruling.
