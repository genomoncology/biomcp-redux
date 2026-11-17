# Article cards

`live/` contains one EFetch capture and two ESearch captures from the approved CC BY articles 33123537, 35440569, 28944238 and 35061708. The unknown PMID 99999999 contributes no article. Job j9228 captured the source replies once.

Folders prefixed `synthetic-` derive request identities and selected XML records from that capture, or replace the reply with explicit synthetic absence, mismatched identity, shared DOI or failure examples. They are synthetic recordings. No provider error body is treated as a checked cache answer.

The checked first search item 33377972 has no objects under its PMC7774047 prefix in the public OA bucket. Its abstract remains outside the repository.

`book-chapter/` uses the original request identities for PMID 21882399 alone and in a mixed request with journal article PMID 33123537. The November 17 replies have been reduced to source citation metadata. The book chapter prose and all abstracts are excluded from the fixtures.

`unbound-book-chapter/` is a response-only control. It removes the chapter PMID from the mixed citation replay, so the chapter cannot claim an identity while the journal citation remains usable.
