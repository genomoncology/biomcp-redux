# Article search keeps every source one option away

Date: 2026-10-24

Decided by Ian, answering the article slice's questions.

BioMCP 1.0 keeps every article source and capability that 0.9 had. Plain PubMed search in PubMed's relevance order stays the default, because it does a good relevance search. Every other source and search method sits one option or subcommand away, so an agent working a hard question can walk a hierarchy of strategies to find the needle in the haystack.

- LitSense2 and PubTator3 stay as named sources. They do a different job from PubMed, not a worse one: LitSense2 finds sentences that resemble the query, and PubTator3 searches normalized genes, diseases and chemicals. Help text describes each source's job and its known failure modes, such as LitSense2 returning resemblances for any query, without ranking sources against each other.
- Semantic Scholar stays, with its search, TLDR summaries, citations and references with citation contexts, citation evidence, recommendations, and open-access PDF full text. Semantic Scholar is free. Its optional key raises the request limit and unlocks more requests; it is not a paid service.
- A paper's related or recommended papers may not share its gene or topic. That is fine: they serve edge-case discovery.
- The AKT1 E17K bar: the recent paper (33123537) is on the default page one, and the 2007 discovery paper (17611497) is first on the most-cited order of the same term, which the default page's `next` names.

COSMIC is out of BioMCP's plans. BioMCP uses free gene sources. The only COSMIC data BioMCP shows is the old pre-license COSMIC copy inside MyVariant.info, which belongs to the variant slice.
