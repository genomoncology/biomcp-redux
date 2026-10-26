# Author investigation

This investigation proposes an author surface through steps 1 to 4 of the slice method. It treats an author identifier as one source's record. A shared name, affiliation, paper or research topic never establishes that two records represent one person. Bibliographic questions below ask what a source asserts and make no clinical judgment.

## Questions and accepted answer boundaries

The questions are derived from the cited publications and public reports. Each accepted answer names the source evidence required, rather than substituting a plausible researcher name. Publication identity does not prove professional identity across sources.

| Case | Question | Harvest source | Accepted answer |
| --- | --- | --- | --- |
| A01 | Which author records does the source attach to the AKT1 E17K paper? | PMID 17611497 | Its exact Semantic Scholar paper byline, with every returned author ID. |
| A02 | What else does that paper's first author record list? | PMID 17611497 | The first byline record's own papers in native order, with exact paper identifiers. |
| A03 | What affiliation and publication counts does that author record state? | PMID 17611497 | Fields on that exact record, with missing fields stated as unavailable. |
| A04 | Can I open that paper from its author page? | PMID 17611497 | An exact PMID or DOI supplied on a returned paper, without title matching. |
| A05 | Which source records authored the AKT1 functional study? | PMID 33123537 | The source paper's complete returned byline, retaining separate IDs. |
| A06 | What papers does its first author record contain? | PMID 33123537 | A bounded native author page, with its continuation and no inferred corpus. |
| A07 | Does the source establish that similarly named records are the same author? | PMID 33123537 | No automatic link from a similar name; source records remain separate. |
| A08 | Can I follow a paper without a PMID? | PMID 33123537 and its author corpus | Preserve its source paper ID and native DOI when present; never invent a PMID. |
| A09 | Which investigator records appear on the sotorasib paper? | PMID 34096690 | Exact byline records returned for the checked PMID. |
| A10 | What other papers belong to its first author record? | PMID 34096690 | The named record's native publication page. |
| A11 | Does its citation count establish who leads this topic? | PMID 34096690 | A source count with scope and date; no clinical expertise or topic rank inferred. |
| A12 | Can I retrieve the full metadata of a listed paper? | PMID 34096690 | A named disclosure layer or exact article pivot, preserving source identifiers. |
| A13 | Which records authored the KRAS inhibitor paper? | PMID 32955176 | Source-owned byline and author IDs. |
| A14 | Does an author have more papers than the default page? | PMID 32955176 | Native total where provided, explicit continuation, and bounded default rows. |
| A15 | Can I continue without silently changing the query? | PMID 32955176 | The next action keeps source, author ID, page size and metadata selection. |
| A16 | Are article author IDs globally resolved people? | PMID 32955176 | They identify provider records only; no cross-provider claim. |
| A17 | Which public works does Charles Swanton's ORCID record claim? | PMID 37046094; ORCID 0000-0002-4299-3018 | Exact public work groups on the requested ORCID, with assertion provenance. |
| A18 | What public name does that ORCID record state? | PMID 37046094; ORCID 0000-0002-4299-3018 | Public person name from the exact requested record. |
| A19 | Which claimed works carry identifiers I can pass to article? | PMID 37046094; ORCID 0000-0002-4299-3018 | Native PMID and DOI assertions, preserving identifier type and relationship. |
| A20 | Are several assertions in one ORCID work group several papers? | PMID 37046094; ORCID 0000-0002-4299-3018 | Preserve group scope and summaries; do not count each assertion as a separate publication. |
| A21 | Which candidate records does a search for Louis Williams return? | Legacy author identity investigation, PMIDs 42146891 and 42056076 | Separate native candidates, including apparent splits; no automatic choice. |
| A22 | What public works does Atul Butte's ORCID record claim? | Legacy identity investigation; ORCID 0000-0002-7433-2740 | Exact ORCID works; never merge with Semantic Scholar 1716151 by name. |
| A23 | Which authors are most prominent on AKT1 E17K? | PMID 17611497 | Native article discovery can show bylines and counts. Neither author API proves a topic-wide author ranking; say unavailable. |
| A24 | Why does an ORCID person fail when its works succeed? | Public issue 289 | A public lowercase visibility value is accepted; other visibility values stay excluded. |

Technical controls cover an empty candidate query, missing exact record, optional key omitted, source disabled, source 429, malformed body, mismatched identity, and repeated work assertions. A source failure never becomes an empty answer.

## Prior behavior and public issues

The closed public issue inventory was read on 2026-10-26: all 57 closed issues were retrieved, and full title/body matching for author, ORCID, Semantic Scholar and S2_API_KEY found 289 and 225. Issue 290 also applies through shared source-group controls.

- [289](https://github.com/genomoncology/biomcp/issues/289): retain exact ORCID detail and claimed works, accept the recorded lowercase public visibility, and keep anonymous access. OAuth credential creation remains outside the command; existing configuration accepts a pre-issued optional token. The tutorial's scope encoding is configuration documentation, not an OAuth implementation.
- [225](https://github.com/genomoncology/biomcp/issues/225): retain anonymous Semantic Scholar with optional `S2_API_KEY`; omit the header when absent. A rate limit remains a source failure with a key-help action where appropriate.
- [290](https://github.com/genomoncology/biomcp/issues/290): ORCID API terms restrict commercial use. It belongs in the shared noncommercial group; disabling a source also prevents cached delivery.

| Legacy command or option | Proposed disposition | Reason |
| --- | --- | --- |
| `search author -q/--query TEXT` | Keep query capability as `search author TEXT` | Native Semantic Scholar candidates, unchanged text; no disambiguation guess. |
| `--source semanticscholar` | Keep as `--source semantic-scholar` | Existing source spelling convention; do not silently query ORCID name search. |
| Search `-l/--limit`, `--offset` | Keep long flags and add shared `--pool` | Native bounded page in source order, with stable local continuation. |
| `get author semanticscholar:ID` | Keep, accept canonical source spelling and existing qualified alias | One exact provider record. |
| `get author orcid:ID` | Keep | Exact public ORCID person record, checksum validated before request. |
| `author papers ID` | Keep | Provider-qualified corpus, with explicit source provenance. |
| Papers `-l/--limit`, `--offset` | Keep long flags and shared pool | Semantic Scholar pages natively; ORCID work summaries may require one bounded whole reply. |
| Papers `--full` | Keep rich disclosure, proposed named `details` section | Preserve existing source facts; heavy abstracts and bylines stay below compact defaults. PM must approve spelling. |
| `article authors ID` | Article slice already retires the separate command | Author slice must ensure article bylines carry provider-qualified pivots where source supplies them; no new hidden enrichment call. |
| `list author` | Shared typed help | Explain exact provider identity, sources, limits and sections. |
| Global JSON/Markdown, fresh/cache and source-disable options | Shared behavior | No entity-specific duplicate configuration. |

No coauthor aggregation, topic scoring, affiliation filtering or cross-provider merging shipped in the legacy author surface. They are not retained merely because an earlier design proposed them. Its rich paper page did ship and therefore needs equivalent named access.

## Source boundaries

[ORCID Public API terms](https://info.orcid.org/public-client-terms-of-service/) distinguish public registry data from the API service. Public data does not grant commercial API-service use. The source should be labeled noncommercial and carry these terms on every outcome. Native work assertion provenance and external-identifier relationships must survive normalization. ORCID reads use the user's optional `ORCID_ACCESS_TOKEN` by environment name; the investigation never performs token issuance or inspects a token.

Semantic Scholar author APIs use an optional `S2_API_KEY`. Existing article source terms and source-group membership apply. Publisher text in rich papers stays outside Git unless its license permits inclusion. No capture is evidence of a right to redistribute the corpus.

The investigation has not called either author API. Current source terms require a product ruling before new calls. The coordinator paused both candidate API captures while routing that ruling.

ORCID's Public API terms section 1 covers Anonymous API and Public API alike. Section 2 restricts service use in revenue-generating products or services. An optional token changes quota, not those rights. The Member API is a separate membership route; this investigation has no membership authorization. Historical repository fixtures can establish an old reply shape but cannot establish current access rights or current facts.

[The annual Public Data File policy](https://info.orcid.org/public-data-file-use-policy/) grants CC0 for the released snapshot with no access or reuse conditions. It excludes rights in linked articles and does not waive individuals' rights. [The bulk-data guide](https://info.orcid.org/documentation/integration-guide/working-with-bulk-data/) lists a 2025 XML summary archive and eleven activity archives, organized by identifier suffix. A snapshot could preserve exact ORCID identity and claimed works while disclosing its release date. Its cost and individual-record extraction remain unmeasured; no archive download or conversion tool is authorized by this investigation.

[The current Semantic Scholar API agreement](https://api.semanticscholar.org/license/), section 2(a), limits its default grant to internal use for model training/evaluation and dataset transformation for noncommercial research or education. Section 2(b) offers an expanded license through the provider. The existing adapter labels the source noncommercial. That label alone does not establish eligibility for a new author API investigation. The proposed capture purpose would be bounded internal evaluation of entity retrieval, without publishing source bodies, reselling the API, model training or user-facing commercial delivery. The PM must determine whether an existing expanded permission covers it or choose an eligible route. No key is proof of expanded rights.

No whole Semantic Scholar dataset is proposed. ORCID snapshot metadata can be investigated separately from the restricted API service.

## Proposed command and disclosure shape

`search author TEXT [--source semantic-scholar] [--limit N] [--offset N] [--pool ID]` returns source candidates. Each header keeps the qualified ID, full source name, source-provided affiliations, and native paper/citation counts when available. Names never become IDs.

`get author ID... [SECTION]...` returns one exact card per input. The card keeps source identity, professional display name, source counts and a link to the provider. Public contact details, biography, homepages and demographic inference are unnecessary. A candidate never gains an ORCID from a matching name.

`author papers ID [details] [--limit N] [--offset N] [--pool ID]` returns source paper headers. Compact rows keep native paper ID, exact PMID/DOI where supplied, whole title, year and venue. Details preserve source-provided abstract, dates, counts, access metadata, fields, publication types and byline. The source's own order stays intact. ORCID claims remain separately typed from Semantic Scholar indexed papers.

Proposed types are `AuthorId` with explicit source-qualified variants, `AuthorHeader`, `AuthorCard`, `AuthorPaperHeader`, and a separately typed `OrcidWorkGroup` with ordered assertion summaries. Source-derived counts are optional and state their scope. ORCID identifiers retain their original type and relationship before any exact article pivot. No field promises a global publication total, current employment or verified person identity.

Each search candidate gets a `disambiguate` action before the page action. Author cards offer the papers pivot. A paper row offers an exact article pivot when its native identifier is supported. More rows offer one page action retaining all selectors. Failed sources offer `fresh`; optional-key throttling can offer `key`. All actions follow architecture 6.10 and use exact IDs.

## Delight questions

1. A name search should show separate candidates and enough source evidence to choose one without pretending to identify a person.
2. An ORCID input should open its exact public record without a name search or an unnecessary Semantic Scholar request.
3. A publication page should provide complete titles and usable article identifiers before the agent asks for abstracts.
4. A same-name or apparent split should remain visible; source disagreement should never be silently repaired.
5. A whole ORCID works response should expose its download and page scope rather than pretend the provider paginates it.
6. A topic-wide author rank should be stated unavailable when the source only offers article search and per-author counts.
7. Missing identifiers and optional metadata should remain unavailable; empty public works and provider failure should have distinct statuses.
8. An author-paper pivot should retain source identity and assertion provenance, allowing an agent to stop without guessing.

## Evidence and budgets

Live evidence and measured compact/default/details budgets will replace this paragraph before PM review. No 0.9 execution has been authorized without verified network containment; its runtime comparison is unmeasured until that condition is met. Legacy source and CLI contracts above are read-only code observations.

## Ian's questions

The PM must rule on the proposed `details` spelling for the shipped `--full` capability, the alias for existing `semanticscholar:` IDs, and the bounded ORCID group/assertion disclosure after captures establish its native shape. No user decision is requested before those measurements exist.
