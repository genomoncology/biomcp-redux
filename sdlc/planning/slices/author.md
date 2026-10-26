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
| A05 | Which source records authored the AKT1 E17K review? | PMID 33123537 | The source paper's complete returned byline, retaining separate IDs. |
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
| `--source semanticscholar` | Keep as an alias of canonical `--source semantic-scholar` | Accepted compatibility preserves existing calls; do not silently query ORCID name search. |
| Search `-l/--limit`, `--offset` | Keep long flags and add shared `--pool` | Native bounded page in source order, with stable local continuation. |
| `get author semanticscholar:ID` | Keep as an alias of canonical `semantic-scholar:ID` | One exact provider record; the existing ID spelling remains accepted. |
| `get author orcid:ID` | Keep | Exact public ORCID person record, checksum validated before request. |
| `author papers ID` | Keep | Provider-qualified corpus, with explicit source provenance. |
| Papers `-l/--limit`, `--offset` | Keep long flags and shared pool | Semantic Scholar pages natively; ORCID work summaries may require one bounded whole reply. |
| Papers `--full` | Keep as an alias of the named `details` section | Accepted spelling preserves existing rich metadata; heavy abstracts and bylines stay below compact defaults. |
| `article authors ID` | Article slice already retires the separate command | Author slice must ensure article bylines carry provider-qualified pivots where source supplies them; no new hidden enrichment call. |
| `list author` | Shared typed help | Explain exact provider identity, sources, limits and sections. |
| Global JSON/Markdown, fresh/cache and source-disable options | Shared behavior | No entity-specific duplicate configuration. |

No coauthor aggregation, topic scoring, affiliation filtering or cross-provider merging shipped in the legacy author surface. They are not retained merely because an earlier design proposed them. Its rich paper page did ship and therefore needs equivalent named access.

## Source boundaries

[ORCID Public API terms](https://info.orcid.org/public-client-terms-of-service/) distinguish public registry data from the API service. Public data does not grant commercial API-service use. The source should be labeled noncommercial and carry these terms on every outcome. Native work assertion provenance and external-identifier relationships must survive normalization. ORCID reads use the user's optional `ORCID_ACCESS_TOKEN` by environment name; the investigation never performs token issuance or inspects a token.

Semantic Scholar author APIs use an optional `S2_API_KEY`. Existing article source terms and source-group membership apply. Publisher text in rich papers stays outside Git unless its license permits inclusion. No capture is evidence of a right to redistribute the corpus.

[Ian's new-source ruling](../../decisions/2026-10-26-new-sources-follow-open-by-default.md) authorizes development captures and implementation under the open-by-default policy. Semantic Scholar and ORCID remain `noncommercial`. Commercial users disable that group. Neither source needs a separate expanded-license prerequisite for this work. An optional key changes access quota and does not change the source's use category.

Exact public ORCID person and works API requests are the selected route. The annual Public Data File, bulk downloads, hosted queries, OAuth exchanges and account creation are outside this slice. Historical fixtures establish historical reply shapes only. They do not establish current source facts.

## Proposed command and disclosure shape

`search author TEXT [--source semantic-scholar] [--limit N] [--offset N] [--pool ID]` returns source candidates. `semantic-scholar` is canonical; `--source semanticscholar` and `semanticscholar:` IDs remain accepted aliases. Each header keeps the qualified ID, full source name, source-provided affiliations, and native paper/citation counts when available. Names never become IDs.

`get author ID... [SECTION]...` returns one exact card per input. The card keeps source identity, professional display name, source counts and a link to the provider. Public contact details, biography, homepages and demographic inference are unnecessary. A candidate never gains an ORCID from a matching name.

`author papers ID [details] [--limit N] [--offset N] [--pool ID]` returns source paper headers. The shipped `--full` flag remains an alias for `details`. Compact rows keep native paper ID, exact PMID/DOI where supplied, whole title, year and venue. Details preserve source-provided abstract, dates, counts, access metadata, fields, publication types and byline. The source's own order stays intact. ORCID claims remain separately typed from Semantic Scholar indexed papers.

Proposed types are `AuthorId` with explicit source-qualified variants, `AuthorHeader`, `AuthorCard`, `AuthorPaperHeader`, and a separately typed `OrcidWorkGroup` with ordered assertion summaries. Source-derived counts are optional and state their scope. ORCID identifiers retain their original type and relationship before any exact article pivot. No field promises a global publication total, current employment or verified person identity.

The shared biodata `ScientificPublication` holds checked PMID/PMCID/DOI identities and opaque `AuthorshipAssertion` values. Those assertions retain source order and repeated display names while declaring completeness unknown; they deliberately establish no person identity. Reuse that bibliographic meaning without forcing a Semantic Scholar author ID, source paper ID or ORCID work assertion into a publication identifier slot. Source-specific records must retain identifiers that the shared publication value cannot represent. The existing `Envelope`, `SourceOutcome`, `PageInfo`, status and typed `Step` contract remain the outer schema; candidate, card, compact papers and rich papers each need generated closed schemas once captures establish their fields. No biodata or schema edit belongs to this investigation.

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

## Measurement scope and decision boundary

The [accepted author rulings](../../decisions/2026-10-26-protein-pathway-study-rulings.md#author) accept this metadata-only boundary. [Ian's later ruling](../../decisions/2026-10-26-new-sources-follow-open-by-default.md) lifts the capture pause for both sources. The bounded API capture below establishes exact source replies. It does not establish accepted types, command output ceilings or warm-cache behavior. Existing article code and its replays stay intact. No bulk download, conversion tool, OAuth exchange or account creation ran.

All 24 questions above are harvested questions with acceptance criteria. They are not 24 verified source answers. A01 has a current exact byline capture. The other Semantic Scholar capabilities remain deferred after its recorded 429. Exact public ORCID person and works captures support A17–A20/A22 and the lowercase-public visibility case A24; they do not establish normalized command answers. A24 additionally has the historical recorded lower-case reply and public bug report. Unsuccessful documentation retrieval is not a source failure witness for a proposed command.

| Evidence | Observation and limitation |
| --- | --- |
| Legacy CLI and source files | Read-only inventory of all author commands and flags, exact source IDs, rich paper fields and source assertions. No baseline execution. |
| Closed public issues | All 57 closed issues read; entity/source matches 289 and 225, with shared licensing switch request 290 considered separately. A search hit about trial semantic queries is unrelated. |
| ORCID service policy | Current anonymous and Public API restrictions verified from the primary policy. Member API terms and entitlement remain unverified. |
| Source use policy | Both sources retain `noncommercial` under Ian's open-by-default ruling. The capture pause and separate expanded-license prerequisite are superseded. |
| Documentation limits | Swagger retrieval returned 429, and a separate incorrect license page returned 403; neither was retried. The correct primary API license page was readable. These are documentation observations only. |

| Command or layer | Measurement required before implementation | Current budget claim |
| --- | --- | --- |
| `search author` default page | Exact candidate order, source total, full names/affiliations, request count, compact JSON bytes/tokens, empty and failure outcomes | Unmeasured. No numeric ceiling accepted. |
| `get author` Semantic Scholar card | Matching returned author ID, optional field/multiplicity inventory, cold/warm timings and bytes/tokens | Unmeasured. No numeric ceiling accepted. |
| `get author` ORCID card | Exact checksummed ID, public name visibility, public API provenance, matching returned identity | Unmeasured for the public API route. Historical fixtures are not current proof. |
| `author papers` compact | Native order, page size/next/total scope, every identifier type and exact article join, same-question bytes/tokens | Unmeasured. No numeric ceiling accepted. |
| `author papers details` | Same page as compact, absent/optional abstract and access fields, complete bylines and counts, source terms | Unmeasured. No numeric ceiling accepted. |
| ORCID groups and assertions | Group cardinality, representative selection without silent loss, external-ID relationship, bounded assertion drill-down | Unmeasured for the proposed route. No type accepted from a guessed shape. |
| Whole help, entity help and MCP listing | Exact command spelling, examples and tool-description growth on approved implementation | Existing caps remain unchanged; this design requests no cap increase. |
| Same-question 0.9 comparison | Verified network containment, exact request graph and replay of eligible captures | Unmeasured; no legacy binary was executed. |

## Bounded source captures

The approved recipe is `scripts/author_capture_plan.json`; `scripts/capture_author_witnesses.py` executes it through the existing source-capture credential mechanism. The default paper and search page requests use five rows at offset zero, with offset five allowed only when the preceding reply returns that native continuation. Compact papers request `paperId,externalIds,title,year,venue`. Details add abstract, publication date, citation/reference/influential counts, open-access metadata, fields, publication types and author IDs/names. Cards and search request `authorId,name,affiliations,paperCount,citationCount,hIndex,url`. These selectors define the proposed product requests for the downstream tickets.

Job j15202 ran revision `c4c266b69277cd963f4d4d55921d98da802fd465` on the build machine. It made seven physical requests from the approved maximum of twenty. Each attempted row ran once, without retries, redirects, fallback hosts or altered selectors. The bounds were 25 seconds and 8 MiB per body, with at least one second between Semantic Scholar requests and half a second between ORCID requests. The machine supplied `S2_API_KEY` and no `ORCID_ACCESS_TOKEN`; the archive records those presence booleans only. Semantic Scholar returned 429 on the second byline and stopped. The independent fixed ORCID plan continued anonymously. A passing capture job proves evidence retention; it does not turn the Semantic Scholar failure into a successful capability.

Evidence archive A is Yellow `data/biomcp-redux/0264-author-captures/2290-author-captures-redacted.tar.gz`, SHA-256 `0a873520c1dacb33c93a22f00868ebae58401f21ad0b768026ac8c52b75c3964`. Its `ledger.json` names every attempted exact URL, status, body digest/bytes, elapsed time, returned author IDs and native paging fields. The numbered `.body`, `.headers` and `.receipt.json` files preserve each request's source reply. The archive includes the fixed recipe, runner, source helper, revision, credential-presence booleans and `deferred.json`. The nested `phase2-inventory.tar.gz` was created and verified before phase two; it contains the five exact ORCID URLs. No Semantic Scholar dependent URL was instantiated because phase one did not supply all required IDs. An offline correction removed five ORCID transport `Set-Cookie` response headers without making another request or changing any body. `evidence-redaction.json` identifies the original archive digest and the affected header files. Every one of the 35 outer members and both nested inventory members in the corrected archive was verified independently against its manifest on Yellow and from the coordination machine. The original archive passed independent member verification and cross-machine SHA comparison before the capture job deleted its own experiment folder and archive. The offline correction then created the archive cited here and verified its members and cross-machine SHA independently. That correction held its working data in memory and created no scratch files. The original archive, other experiments and job logs remain intact.

| Planned witness | Result | Capability and evidence in archive A |
| --- | --- | --- |
| PMID 17611497 byline | Proven, HTTP 200 | A01 exact byline: 24 returned author IDs, first `2060978`; `0000.body`, SHA-256 `47c460ab4001d4c40463da395d7c7412070a4ccf4e4056a0cfb8c6b167b06a6a`. |
| PMID 33123537 byline | Failed, HTTP 429 | Exact current A05 request failed; `0001.body`, SHA-256 `65ab993d12c5c2cc9b68e6da2bb79b326930283e520363ffcdc145f88cd5a148`. Earlier 0061 native bylines remain historical evidence and do not replace this exact request. |
| PMID 34096690 byline | Failed prerequisite; not sent | A09 current exact byline remains deferred after the source stop. |
| PMID 32955176 byline | Failed prerequisite; not sent | A13 current exact byline remains deferred after the source stop. |
| Louis Williams search, offset 0 | Failed prerequisite; not sent | A21 candidate order, native total and page remain deferred. |
| Louis Williams search, offset 5 | Failed prerequisite; not sent | A21 continuation and nonoverlap remain deferred; no native next was obtained. |
| First PMID 17611497 author card | Failed prerequisite; not sent | A03 card fields/count scope remain deferred. |
| First PMID 34096690 author card | Failed prerequisite; not sent | A11 card fields/count scope remain deferred; required ID missing. |
| First PMID 17611497 compact papers, offset 0 | Failed prerequisite; not sent | A02/A04 papers and exact article pivots remain deferred. |
| First PMID 34096690 compact papers, offset 0 | Failed prerequisite; not sent | A10 compact papers remain deferred. |
| Same PMID 34096690 papers, details | Failed prerequisite; not sent | A12 same-page details and optional fields remain deferred. |
| First PMID 32955176 compact papers, offset 0 | Failed prerequisite; not sent | A14 native paging remains deferred. |
| Same PMID 32955176 compact papers, offset 5 | Failed prerequisite; not sent | A15 continuation remains deferred; required ID and native next missing. |
| First PMID 33123537 compact papers, offset 0 | Failed prerequisite; not sent | A06/A08 papers and native identifiers remain deferred. |
| Author `999999999999999999` exact card | Failed prerequisite; not sent | Semantic Scholar absence remains unproved; absent-versus-failed cases must be labeled synthetic. |
| ORCID `0000-0002-4299-3018` person | Proven, HTTP 200 | A18 exact `/0000-0002-4299-3018/person`, lowercase `public` name visibility; `0002.body`, SHA-256 `e9a8fda15f08a696c94386638ff4cdc3e5dad23902a3b9ddf6c6c44968f146dc`. |
| ORCID `0000-0002-4299-3018` works | Proven, HTTP 200 | A17/A19/A20: 380 groups, 453 assertions, up to two summaries per group; `0003.body`, SHA-256 `7fb1493eda38b38eb1e5fc341e725cbf50e761330f3ab828ce7b2224ec171553`. |
| ORCID `0000-0002-7433-2740` person | Proven, HTTP 200 | Exact `/0000-0002-7433-2740/person`, lowercase `public` name visibility; `0004.body`, SHA-256 `4ad727a35bb96431616d1c81a5de18cb74e0ea53c8747aa30dca825612ff0bcb`. |
| ORCID `0000-0002-7433-2740` works | Proven, HTTP 200 | A22: two groups and two assertions; `0005.body`, SHA-256 `242b829be3b7ee63ba65113af20ea21fab45298ba019a152568279b29b925710`. |
| ORCID `0000-0000-0000-0001` person | Empty, HTTP 404 | Exact checksum-valid absence witness; `0006.body`, SHA-256 `0670f59cceb60cd03fa9d4b9feaaa6627a5da264cc1001919da66fa06688da2b`. |

The Semantic Scholar byline body was 1,350 bytes; its failed second byline was 174 bytes. The two ORCID person replies were 3,513 and 964 bytes. The two ORCID works replies were 886,277 and 2,568 bytes. The ORCID absence reply was 219 bytes. Every returned work assertion had lowercase `public` visibility. The first works reply preserved `doi`, `pmid`, `wosuid`, `eid` and `pmc` identifier types with 922 `self` relationships; the second preserved two `doi` identifiers with `self` relationships. Both retained source and assertion-origin identifiers. No returned external-ID value was missing. Ledger timing records cover only source requests and do not establish command latency. ORCID works replies have whole-record scope and no native total or continuation fields; group and assertion counts above are observed reply cardinalities, not a global publication total. Person replies include fields outside the proposed card; those fields remain outside the product card. No raw person reply, paper body or publisher abstract enters Git.

Before type acceptance, the capture set must cover each source field's absence, multiplicity and collection scope. The eventual owning cases assert measured whole replies plus the agreed allowance, excluding only the shared source-term comparison fields according to the existing policy. No fixture may replace current source evidence with invented positive data. Licensed source bodies and unpublished question sets remain outside Git.

## Command and option case list

| Surface | Required cases and harvest links |
| --- | --- |
| Search positional text and legacy `-q/--query` compatibility ruling | A21 same-name candidates, A23 topic rank refusal, whitespace/empty usage, unchanged native text. |
| Search `--source` and legacy spelling | A21 explicit source, unsupported source usage, disabled source group, eligible keyless reply, 429, malformed body. |
| Search `--limit`, `--offset`, `--pool` | A21 first page and exact continuation, min/max/invalid bounds, empty page, warm page makes no request, expired pool offers restart. |
| `get author` one/many qualified IDs and ID alias | A01/A05/A09/A13 exact provider identities; A18/A24 exact ORCID; input order; invalid checksum before request; missing record; mismatched source ID refused; no name-based join. |
| Card optional fields and output formats | A03/A11 counts/affiliations unavailable versus absent, compact JSON schema, Markdown facts, CLI/MCP parity. |
| `author papers` compact | A02/A04/A06/A08/A10/A14/A16/A17/A19/A20/A22 native papers and exact article pivots, no enrichment or corpus merge. |
| Papers `--limit`, `--offset`, `--pool` | A14/A15 native continuation or disclosed whole snapshot scope, stable source order, warm page, expired pool, incomplete source window. |
| Papers rich layer / legacy `--full` | A12 same paper IDs and page, retained native abstract/date/count/access/field/type/byline capabilities, absent fields remain optional; continuation retains rich selection. |
| ORCID assertion disclosure | A17/A19/A20 same-group multiple summaries, exact external ID type/value/relationship, nonpublic assertion exclusion, lower/upper public visibility, stale snapshot disclosure. |
| Global source disable and fresh | Issue 290 prevents requests and cache restoration; failure differs from an empty public record and offers a typed next action. |
| Schema/help | Documented source-qualified IDs, section names, page bounds, identity limits and license groups match generated command grammar; existing shared help caps apply. |

## Structural risks

Source IDs and their aliases should have one checked type and one parser. Every article pivot should consume a native exact identifier rather than repeat normalization or infer identity from a display name. Public ORCID work groups and Semantic Scholar indexed papers need separate raw records because their collection scopes differ. The shared transport owns credentials, quotas, source groups and caching; this slice adds no second key scanner, cache or pagination framework. Tests isolate configuration, cache, home and secrets while preserving required operating-system environment. Real-size reply measurements must precede defaults so a prolific author cannot print an entire corpus by accident.

## Ian's questions

Ian resolved source eligibility through the open-by-default ruling. Both sources retain their noncommercial use category. Exact capture results must establish which capabilities can proceed.

The accepted rulings keep provider-qualified identity, native candidate order, compact papers and the named `details` section, with shipped `--full` as its alias. `semantic-scholar` is canonical; existing `semanticscholar:` IDs and `--source semanticscholar` remain aliases. Topic-wide author ranking and cross-provider merging are deferred. Types, output ceilings and question answers wait for exact public API captures. No capability retirement or new source cost is accepted by this proposal.
