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
| `list author` | Retire with fixed guidance to `help author`, `help search author` and `help get author` | Typed help retains provider identity, sources, limits and sections. |
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

No shared publication or authorship record exists in biodata, and this slice adds none. Author-owned checked records hold native byline assertions, which retain source order and repeated display names while declaring completeness unknown and establish no person identity. They reuse the existing checked `Pmid`, `Pmcid`, `Doi`, `SemanticScholarId` and `Curie` types without forcing a Semantic Scholar author ID, source paper ID or ORCID work assertion into a publication identifier slot. Source-specific records must retain identifiers that the shared publication value cannot represent. The existing `Envelope`, `SourceOutcome`, `PageInfo`, status and typed `Step` contract remain the outer schema; candidate, card, compact papers and rich papers each need generated closed schemas once captures establish their fields. No biodata or schema edit belongs to this investigation.

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

All 24 questions above are harvested questions with acceptance criteria. They are not 24 verified source answers. A01, A05 and A09 have current exact byline captures. The final dependent capture below establishes current Semantic Scholar search, cards, compact/details papers, native continuation and author absence. The exact PMID 32955176 paper lookup returned 404 and supplied no author ID. Exact public ORCID person and works captures support A17–A20/A22 and the lowercase-public visibility case A24; they do not establish normalized command answers. A24 additionally has the historical recorded lower-case reply and public bug report. Unsuccessful documentation retrieval is not a source failure witness for a proposed command.

| Evidence | Observation and limitation |
| --- | --- |
| Legacy CLI and source files | Read-only inventory of all author commands and flags, exact source IDs, rich paper fields and source assertions. No baseline execution. |
| Closed public issues | All 57 closed issues read; entity/source matches 289 and 225, with shared licensing switch request 290 considered separately. A search hit about trial semantic queries is unrelated. |
| ORCID service policy | Current anonymous and Public API restrictions verified from the primary policy. Member API terms and entitlement remain unverified. |
| Source use policy | Both sources retain `noncommercial` under Ian's open-by-default ruling. The capture pause and separate expanded-license prerequisite are superseded. |
| Documentation limits | Swagger retrieval returned 429, and a separate incorrect license page returned 403; neither was retried. The correct primary API license page was readable. These are documentation observations only. |

Native source reply measurements and exact identity/paging proofs appear below. Command output budgets, warm-cache measurements and normalized type acceptance remain downstream work.

| Command or layer | Measurement required before implementation | Current budget claim |
| --- | --- | --- |
| `search author` default page | Exact candidate order, source total, full names/affiliations, request count, compact JSON bytes/tokens, empty and failure outcomes | Unmeasured. No numeric ceiling accepted. |
| `get author` Semantic Scholar card | Matching returned author ID, optional field/multiplicity inventory, cold/warm timings and bytes/tokens | Unmeasured. No numeric ceiling accepted. |
| `get author` ORCID card | Exact checksummed ID, public name visibility, public API provenance, matching returned identity | Public API identity and visibility proved below. Normalized card size remains unmeasured. |
| `author papers` compact | Native order, page size/next, unavailable native total, every native identifier and exact article pivot | Retained native measurements below; each owning case holds its measured ceiling. |
| `author papers details` | Same page as compact, optional abstract and access fields, native bylines and counts, source terms | Retained native measurements below; each owning case holds its measured ceiling. |
| ORCID groups and assertions | Group cardinality, representative selection without silent loss, external-ID relationship, bounded assertion drill-down | Native groups and assertion provenance proved below. Representative selection and normalized output remain unmeasured; no type is accepted here. |
| Whole help, entity help and MCP listing | Exact command spelling, examples and tool-description growth on approved implementation | Existing caps remain unchanged; this design requests no cap increase. |
| Same-question 0.9 comparison | Verified network containment, exact request graph and replay of eligible captures | Unmeasured; no legacy binary was executed. |

## Bounded source captures

The approved recipe is `scripts/author_capture_plan.json`; `scripts/capture_author_witnesses.py` executes it through the existing source-capture credential mechanism. The default paper and search page requests use five rows at offset zero, with offset five allowed only when the preceding reply returns that native continuation. Compact papers request `paperId,externalIds,title,year,venue`. Details add abstract, publication date, citation/reference/influential counts, open-access metadata, fields, publication types and author IDs/names. Cards and search request `authorId,name,affiliations,paperCount,citationCount,hIndex,url`. These selectors define the proposed product requests for the downstream tickets.

Job j15202 ran revision `c4c266b69277cd963f4d4d55921d98da802fd465` on the build machine. It made seven physical requests from the approved maximum of twenty. Each attempted row ran once, without retries, redirects, fallback hosts or altered selectors. The bounds were 25 seconds and 8 MiB per body, with at least one second between Semantic Scholar requests and half a second between ORCID requests. The machine supplied `S2_API_KEY` and no `ORCID_ACCESS_TOKEN`; the archive records those presence booleans only. Semantic Scholar returned 429 on the second byline and stopped. The independent fixed ORCID plan continued anonymously. A passing capture job proves evidence retention; it does not turn the Semantic Scholar failure into a successful capability.

Evidence archive A is Yellow `data/biomcp-redux/0264-author-captures/2290-author-captures-redacted.tar.gz`, SHA-256 `0a873520c1dacb33c93a22f00868ebae58401f21ad0b768026ac8c52b75c3964`. Its `ledger.json` names every attempted exact URL, status, body digest/bytes, elapsed time, returned author IDs and native paging fields. The numbered `.body`, `.headers` and `.receipt.json` files preserve each request's source reply. The archive includes the fixed recipe, runner, source helper, revision, credential-presence booleans and `deferred.json`. The nested `phase2-inventory.tar.gz` was created and verified before phase two; it contains the five exact ORCID URLs. No Semantic Scholar dependent URL was instantiated because phase one did not supply all required IDs. An offline correction removed five ORCID transport `Set-Cookie` response headers without making another request or changing any body. `evidence-redaction.json` identifies the original archive digest and the affected header files. Every one of the 35 outer members and both nested inventory members in the corrected archive was verified independently against its manifest on Yellow and from the coordination machine. The original archive passed independent member verification and cross-machine SHA comparison before the capture job deleted its own experiment folder and archive. The offline correction then created the archive cited here and verified its members and cross-machine SHA independently. That correction held its working data in memory and created no scratch files. After independent review, the coordinator authorized deleting only the original archive that retained transport cookies. The corrected archive preserves every source body and the original digest/redaction record. Other experiments and job logs remain intact.

| Initial attempt witness | Initial result | Capability and evidence in archive A |
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

### Approved Semantic Scholar follow-up

PM ruling m7310 authorized one follow-up with the fourteen remaining request recipes in `scripts/author_followup_plan.json`. It reused archive A's successful PMID 17611497 byline and every ORCID reply. The source key remained inherited by name. The shared source-capture helper now records `credential_header` and `credential_header_attached` from the assembled curl configuration at the request boundary. Those fields contain only the header name and a boolean. The synthetic source-capture contract checks actual receiver attachment with a synthetic key, checks keyless omission, and rejects the synthetic value from every evidence file. The original attempt's environment-presence record does not retrospectively prove its header attachment.

Job j15244 ran revision `12da159c6ac39d009952d581f715b70c990a188a` and sent three requests. It enforced five-second gaps, the existing 25-second/8 MiB bounds, zero retries and redirects, and the stop rules. The first two bylines succeeded. PMID 32955176 returned 404, so phase one could not supply every required first-row ID. The dependent phase sent no request; its exact inventory archive is empty and eleven recipes are explicitly deferred. No replacement PMID, provider host, author ID, page, absence probe, 17611497 byline or ORCID request followed.

Evidence archive B is Yellow `data/biomcp-redux/0264-author-captures/2290-author-followup.tar.gz`, SHA-256 `89725953630ed01f691d219d814fc5b822001256b9eebba3906a554c3d3a1c42`. Its 27 outer members include the native ledger/replies, exact deferred recipes, reused archive path/hash, actual window timestamps and two independently verified nested archives. `prelaunch-snapshot.tar.gz` preserves the coordinator's unchanged snapshot, SHA-256 `c23ea544a076836adac512d00eaa16d7c251757a0a71fb37109c8143dd14b369`; it was archived and verified before the first provider call. `phase2-inventory.tar.gz` records that no dependent request was instantiated. Both nested archives contain two members. The follow-up archive was renamed after retention to distinguish it from the original unsanitized archive that the coordinator had authorized deleting. Its bytes and hash did not change.

| Follow-up witness | Result | Capability and evidence in archive B |
| --- | --- | --- |
| PMID 33123537 byline | Proven, HTTP 200 | A05: seven author IDs, first `2118426986`; `0000.body`, 562 bytes, SHA-256 `ff026d53fe524e09a057028bcc5852a006359741b1d2dc6a7dbf5d47c79f2c4c`. |
| PMID 34096690 byline | Proven, HTTP 200 | A09: 26 author IDs, first `9845886`; `0001.body`, 1,448 bytes, SHA-256 `403f9d766dd8002c633670bd8c0cae2afe52996ac8305c463281277e1f5f7867`. |
| PMID 32955176 byline | Failed prerequisite, HTTP 404 | The exact paper lookup supplied no author ID; A13 and its dependent papers remain unproved. `0002.body`, 50 bytes, SHA-256 `4d638311dd5d95c82a6bbadfacecd07aa61634033fbd1edec7820b6b623ea7c7`. This is not an absent-author witness. |
| Both search pages, both cards, six papers pages and absent author | Not sent after failed prerequisite | Eleven recipes remain deferred. No native search/papers continuation or positive card/papers shape was captured. Absent-versus-failed Semantic Scholar author cases still require explicit synthetic labels. |

All three attempted receipts record `credential_header: x-api-key` and `credential_header_attached: true`. No key value, credential header value or value-derived digest is retained. The named-key reflection guard and shared response-header sanitizer remain active. The source window was `2026-10-26T15:15:19.076411+00:00` through `2026-10-26T15:15:29.653814+00:00`. These are capture bounds, not command latency measurements.

The existing SSH route from the build machine could not resolve the coordination host. The coordinator therefore supplied the approved metadata fallback. Its prelaunch snapshot was observed at `2026-10-26T15:12:07.527012+00:00`; it showed the named 0259 and 0260 continuations terminal and recorded their declared local/replay recipes. Its post-completion snapshot was captured at `2026-10-26T15:19:24.626653+00:00`. The coordinator prohibited every other owned live Semantic Scholar job during this window. These records establish the owned-job declarations and bounded metadata snapshots only. Other teams, global jobs and external key holders remain unknown; the records do not prove exclusive use of the key or explain the original 429.

The unchanged post-completion snapshot, SHA-256 `11f0f96a8681ab73d4b7e9466692867a625b41292046a1f548ed200cde72a318`, is in the three-member offline supplement Yellow `data/biomcp-redux/0264-author-captures/2290-author-followup-concurrency.tar.gz`, SHA-256 `6d3fc8ad8795ad3e87e6ab1c1bd5f3e98b56416928b03afa2f9978883b4f2f01`. Its provenance points to archive B and preserves the actual window and visibility limit. It adds no provider request. Every outer and nested member of archives A and B and every supplement member passed independent verification from Yellow and the coordination machine; cross-machine archive hashes matched. Archive A remains unchanged. The capture job removed only its own experiment, archive, unique temporary directory and metadata staging after verified retention. The authorized root pre/post snapshot temporary folders and the supplement's own Yellow staging were removed only after verified copies/archive retention. Source bodies, PM logs and unrelated archives remain intact.

The accepted routine gate j15211 remains applicable to the earlier shared sanitizer change. Affected source-capture, attachment, documentation and syntax contracts passed in j15229; j15244 also checked snapshot-runner syntax and documentation before capture. No full Rust case collection was repeated for this follow-up. At the end of that follow-up, Semantic Scholar search, author cards, papers, details, continuation and author absence remained unproved. The separately approved dependent capture below addresses those prerequisites.

### Approved dependent captures

PM ruling m7360 authorized one job with eleven exact URLs in `scripts/author_dependent_plan.json`. It reused the three proved first-row author IDs and all earlier bylines and ORCID replies. The first and next compact pages for PMID 33123537's record replaced the unresolvable PMID 32955176 papers requests. Both the original native 404 and its lack of an author ID remain visible; no alternate identity was guessed. The job also captured compact/details pairs for both 17611497 and 34096690.

Job j15278 ran `edc39952a810b6b2a3a4248059a1c09f9bedb51e` and sent exactly eleven requests, with five-second gaps, the established 25-second/8 MiB bounds, zero retries and redirects, and the same source-stop rules. Ten replies returned 200 and the exact absent-author probe returned 404. Every receipt records actual `x-api-key` attachment as true. No earlier byline or ORCID request was repeated, and no source alternative ran. The exact eleven-row inventory and the immutable prelaunch snapshot were archived and independently verified before the first dependent request. The source window was `2026-10-26T15:29:25.831572+00:00` through `2026-10-26T15:30:18.677990+00:00`.

Evidence archive D is Yellow `data/biomcp-redux/0264-author-captures/2290-author-dependents.tar.gz`, SHA-256 `d8521464f4f2525a13450efe352abcaa97170a8d3232fde02e9b299202f86eb3`. It has 51 outer members and two nested archives with two members each. The archive was renamed after retention without changing its bytes. The ledger retains all exact URLs, statuses, safe attachment witnesses, native paging/IDs, byte sizes, source timings and body hashes. It records zero deferred rows.

| Dependent witness | Result and source scope | Evidence in archive D |
| --- | --- | --- |
| Louis Williams search, offset 0 | Proven: five candidates, native total 61, next 5. | `0000.body`, 953 bytes, SHA-256 `b6d9d61bfd0ca3d4a5f80dd918359cafaf4511dd4c9d26b7aa90296ba3a2746d`. |
| Same search, offset 5 | Proven: five separate candidate IDs, same total, next 10. | `0001.body`, 965 bytes, SHA-256 `6e4276bf7fb51b829b7b284dafa3d04fbaf5ca3b25733f65aabec6a60fdbdbb1`. |
| Author `2060978` card | Proven: returned ID matches. Native count fields present; affiliations is an empty array. | `0002.body`, 180 bytes, SHA-256 `ade062b9402ef9ef5f6d9f41d2d2594eee02bdbf2683dda563cad1b3dbede5e9`. |
| Author `9845886` card | Proven: returned ID matches. Native count fields present; affiliations is an empty array. | `0003.body`, 182 bytes, SHA-256 `5a7ca10879cc2cdc46dab8683114b7657ba0a209798a62008c44dbe40d4cd445`. |
| Author `2060978` compact papers, offset 0 | Proven: five rows in native order, next 5. Every row has a native PMID and DOI. | `0004.body`, 1,753 bytes, SHA-256 `2312f221faecd7f900ab2e7e15304d5fd9d281819503f980eb863d693dfa2675`. |
| Same author/page, details | Proven: identical ordered paper IDs; four abstracts present and one null. | `0005.body`, 24,844 bytes, SHA-256 `e9bb06ba54046ae8750092f01292ba85d07b4063375ed5561294fd95b8bd4705`. |
| Author `9845886` compact papers, offset 0 | Proven: five rows, next 5; two lack a PMID and all five retain a native DOI. | `0006.body`, 1,764 bytes, SHA-256 `f7ea65a40f81fac6f5514bec9522d46f55aa7403f407612c800085a4928b7011`. |
| Same author/page, details | Proven: identical ordered paper IDs and five abstracts; `fieldsOfStudy` is null in some rows. | `0007.body`, 21,691 bytes, SHA-256 `c22183840366b625e7ad57d6ff776262ccda305b5a292e5c3c573b9a0a06faac`. |
| Author `2118426986` compact papers, offset 0 | Proven: five rows, next 5; one lacks a DOI and all five retain a native PMID. | `0008.body`, 1,773 bytes, SHA-256 `64b85e4dd58794bdc8707ad7f633e86bba39b36b84eb65cd9bc70c886cd59398`. |
| Same author, offset 5 | Proven: five nonoverlapping paper IDs, no next; one lacks a PMID and all five retain a native DOI. | `0009.body`, 1,863 bytes, SHA-256 `5b148691e0e1603922f7edb38bac6c39426e9cc66dc4e8556883bea36e107100`. |
| Author `999999999999999999` card | Empty: native 404 names the exact requested author ID. This establishes author absence separately from the paper lookup failure. | `0010.body`, 56 bytes, SHA-256 `d60c1d562e64c83e96fe9684e8fc3c41b903d1ff583a374e37d8dc8b0271ac6e`. |

Both card replies contain `authorId,url,name,affiliations,paperCount,citationCount,hIndex`; native counts describe the provider record. Compact papers contain `paperId,externalIds,title,venue,year`. Details also contain dates, citation/reference/influential counts, open-access metadata, study fields, publication types, authors and abstract. Missing PMIDs, DOI, abstract and study fields remain explicit source absences. No papers reply supplies a native total, so continuation and observed page size must not be presented as a global publication count. These native reply measurements do not set normalized command-output ceilings or prove cross-provider identity. No abstract, title, byline body or source reply enters Git.

The coordinator's unchanged prelaunch snapshot has SHA-256 `5cd98a25a451a8ba027489328bba67021ae9f7afb206151febcdcdd2784444ea` and is preserved inside archive D. It records the declared local/replay gate for 0259/0260 and the no-provider diagnosis for 0272. The coordinator prohibited other owned live Semantic Scholar jobs during the window. The post-completion snapshot was captured at `2026-10-26T15:31:30.705331+00:00`; its unchanged bytes have SHA-256 `1f7478dbfc90cc792dd0e504b83eaad34498d71b25ccaa317f81c61934378bac`.

That post snapshot and its binding to archive D/window are preserved in the three-member offline supplement Yellow `data/biomcp-redux/0264-author-captures/2290-author-dependents-concurrency.tar.gz`, SHA-256 `e0f6c47caf5c786309dcd0399c80f929826daa6377ff487132da3bcb40d5e0b6`. No provider request was added. These bounded metadata snapshots and declared recipes do not establish global job visibility or exclusive use of the key; other teams and external key holders remain unknown.

Every outer and nested member of archive D and every supplement member passed independent verification on Yellow and from the coordination machine. Cross-machine hashes matched, and the hashes of archives A, B and the earlier concurrency supplement remained unchanged. The job removed only its own experiment, generated archive, unique temporary directory and supplied metadata staging after verified retention. The coordinator-authorized dependent pre/post temporary folders and the supplement's own Yellow staging were deleted only after archive/member/cross-machine verification. Source bodies and other archives remain intact.

The affected script, source-capture, actual-header-attachment and documentation contracts passed in j15271, including actual local HTTP cases for 17611497 details identity, 33123537 continuation and both required stop conditions. No full Rust suite was repeated. All approved current request forms now have positive or native-absence evidence. The exact PMID 32955176 byline remains absent, and its author/papers identity remains unavailable. Topic-wide ranking, coauthor aggregation and cross-provider merging remain deferred. Type acceptance and normalized command measurements remain the downstream tickets' work.

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
| Papers `--limit`, `--offset`, `--pool` | A14/A15 native continuation or disclosed whole public-record scope, stable source order, warm page, expired pool, incomplete source window. |
| Papers rich layer / legacy `--full` | A12 same paper IDs and page, retained native abstract/date/count/access/field/type/byline capabilities, absent fields remain optional; continuation retains rich selection. |
| ORCID assertion disclosure | A17/A19/A20 same-group multiple summaries, exact external ID type/value/relationship, nonpublic assertion exclusion, lower/upper public visibility, stale snapshot disclosure. |
| Global source disable and fresh | Issue 290 prevents requests and cache restoration; failure differs from an empty public record and offers a typed next action. |
| Schema/help | Documented source-qualified IDs, section names, page bounds, identity limits and license groups match generated command grammar; existing shared help caps apply. |

## Structural risks

Source IDs and their aliases should have one checked type and one parser. Every article pivot should consume a native exact identifier rather than repeat normalization or infer identity from a display name. Public ORCID work groups and Semantic Scholar indexed papers need separate raw records because their collection scopes differ. The shared transport owns credentials, quotas, source groups and caching; this slice adds no second key scanner, cache or pagination framework. Tests isolate configuration, cache, home and secrets while preserving required operating-system environment. Real-size reply measurements must precede defaults so a prolific author cannot print an entire corpus by accident.

## Ian's questions

Ian resolved source eligibility through the open-by-default ruling. Both sources retain their noncommercial use category. Exact capture results must establish which capabilities can proceed.

The accepted rulings keep provider-qualified identity, native candidate order, compact papers and the named `details` section, with shipped `--full` as its alias. `semantic-scholar` is canonical; existing `semanticscholar:` IDs and `--source semanticscholar` remain aliases. Topic-wide author ranking and cross-provider merging are deferred. Types, output ceilings and question answers wait for exact public API captures. No capability retirement or new source cost is accepted by this proposal.

## Semantic Scholar execution contract

`search author TEXT` preserves the original nonempty query, including surrounding and repeated whitespace. The source controls candidate order, offsets and continuation. A qualified author ID names a provider record. A shared display name does not link records. The search's retained pool keeps its published identity and expiry while later native pages extend its observed window. Restored pages retain the original source provenance. A changed selector rejects the pool; an expired pool offers a restart. Source disable checks precede pool and raw-cache access.

`get author ID...` accepts one through twenty checked inputs and preserves one result slot for each input in input order. Each slot carries the qualified input, its independent status and its checked card when available. A native missing record is absent. A transport or decode error is failed. The adapter rejects a returned author ID that differs from the request before cache admission. Optional counts carry the `provider_record` scope. Missing metadata is unavailable; an empty source affiliation collection remains distinct from an unavailable collection. Cards offer typed `author papers` pivots. That deferred command returns its explicit deferred response until its owning implementation lands.

The response schemas are `biomcp.author_page/2` and `biomcp.author_card/3`. The shared request schema is `biodata.request/26`. `Step` still carries checked action arguments in its existing wire shape; generic `Envelope` has no independent versioned schema identity. The changed author response identities carry the wrapper and count-scope changes.

Semantic Scholar resolves its credential at an admitted adapter/transport request. This includes raw-cache and replay validation because reflected-credential containment still checks the current credential. A disabled source and a restored query pool do not resolve it. The deferred credential uses the configuration key first, then `S2_API_KEY`; a missing key omits the header. Other credentials retain their existing resolution behavior.

The first-row record for PMID 33123537 has one additional exact card witness. Job j15451 made the approved single request for record `2118426986`, received a native 200 and retained its unchanged 181-byte body with SHA-256 `84dab7a910afdf55a49545196b35116603960ca89d2ceff57a7b6d3a2c8ecd4e`. Yellow `data/biomcp-redux/0264-author-captures/0266-author-card-xowlaemk.tar.gz` has SHA-256 `c077d787ebf31f8308c90a22a9c1085d8fff3ea8c50626afae9d5984f73a4d2a`. The receipt records actual `x-api-key` attachment as a boolean and never records its value. The approved source window was `2026-10-26T17:03:16.126796+00:00` through `2026-10-26T17:03:21.307162+00:00`.

The unchanged prelaunch snapshot is archived inside that capture. Its SHA-256 is `10e4667852be1b6a7a2d952ff963e135eb302ad2baa0ee64b3b0ee841a616504`. Yellow `data/biomcp-redux/0264-author-captures/0266-author-card-concurrency.tar.gz`, SHA-256 `f70e3cdf9b36ce51d60c054608ef30a5c3d75f65a6120a39760e4f78fad86464`, binds the post-completion snapshot to the same archive and window. The post snapshot has SHA-256 `17d1ba9a57d812754723345f71c611708cbda87d4ed8809fc6681b65b4a065d2`. Both archives passed outer and member verification and independent cross-machine hash comparison. These bounded owned-job snapshots do not establish visibility of other teams, global jobs or external key holders.

The exact card cases use the first-row records from the retained complete bylines for PMIDs 17611497, 33123537 and 34096690. They do not claim that every byline member's card was fetched. PMID 32955176 supplied only the retained native paper 404, so its byline and first-row author identity remain unavailable. No replacement identity is inferred.

### Normalized author output measurements

The retained native replay measures the whole emitted envelope, including shared source terms. The bare column measures only `result`; it is not the enforced command ceiling. CLI includes its terminating newline, MCP text is the actual `content[0].text`, and RPC measures the complete retained MCP frame. Native response bodies remain outside Git. Each owning canonical YAML step enforces its raw byte and token ceiling with the agreed 15 percent allowance. Source-body sizes above are separate measurements. The original design supplied no numeric normalized search/card ceiling, so these values do not replace an earlier numeric bound. The same-question 0.9 comparison remains unmeasured.

| Native owner | CLI | Bare result | MCP text | Markdown | Full RPC |
|---|---:|---:|---:|---:|---:|
| Search offset 0 | 1824 | 836 | 1823 | 1597 | 4126 |
| Search offset 5 | 1836 | 845 | 1835 | 1609 | 4150 |
| First card from PMID 17611497 | 703 | 304 | 702 | 744 | 1722 |
| First card from PMID 33123537 | 713 | 308 | 712 | 754 | 1742 |
| First card from PMID 34096690 | 705 | 306 | 704 | 746 | 1726 |
| Exact absent card | 376 | 77 | 375 | 399 | 1016 |

The approved help ceilings for CLI/MCP text/Markdown are search 1262/1261/1137, get 646/644/708 and list 1786/1785/1748. Canonical help cases own raw CLI/MCP ceilings; mandatory Markdown shape cases own the three Markdown ceilings. Existing global discovery ceilings remain unchanged.

The final help metadata includes the twenty-input bound. The measured get-help surfaces are CLI 574, bare help 492, MCP text 573, Markdown 636 and full RPC 1420 bytes. List-author measures 1566/1490/1565/1541/3510 bytes on those same surfaces. Search help remains 1097/1012/1096/988/2536 bytes. Whole help measures 87594/87526/87593/78299/185140 bytes. The MCP description measures 3426 bytes and tools-list measures 8014 compact bytes or 8015 with its terminating newline. All fit the existing approved ceilings; the measurement does not reset their headroom.


## ORCID execution contract

The adapter validates the returned person and works paths against the checked requested ORCID iD before admitting a reply to cache. A name path retains that same exact iD. Every work assertion path retains its record and native put-code. Only the exact lowercase `public` visibility enters a card or work group. The adapter does not reconcile names or call Semantic Scholar for an ORCID input.

`author papers` retains one bounded whole public works reply and pages its groups locally. `collection_scope` states `whole_public_works_reply`; `observed_groups` and page totals describe those public groups, never publications. Each group retains its ordered public assertion summaries, source and assertion-origin identities, and native external identifier type, value and relationship. Only a checked native self PMID or DOI offers an article pivot. An expired collection offers a restart. Source disable checks precede collection and raw cache reads. The response schema is `biomcp.orcid_works/2`.

The optional ORCID credential uses `orcid_access_token` from configuration, then `ORCID_ACCESS_TOKEN`. The shared deferred loader resolves it only when an admitted ORCID source request needs it. The source registry supplies the noncommercial category, public API terms and native record URL. No token issuance or account operation exists.

The accepted capture inventory covers every requested person, works and absent-record URL without query fields. Implementation made no new registry request. Native bodies and expected source text stay outside Git under the retained capture preparation helper. The [ORCID v3 translated-title model](https://github.com/ORCID/orcid-model/blob/main/src/main/java/org/orcid/jaxb/model/v3/release/common/TranslatedTitle.java) defines its `content` field; the adapter preserves it through a typed native structure. The synthetic translated-title case failed with `bad_reply` before this decoder change.
### Native papers API and output measurements

The author papers endpoint binds identity through its exact credential-free request URL. The accepted replies expose `offset`, optional `next`, and `data`; they supply neither a top-level author ID nor a total. The adapter checks the requested offset, row limit, native paper IDs and typed external identifiers before reply-cache admission. A different author's body cannot be detected when those native facts agree. Names and byline membership never establish identity. PM ruling m7735 replaces the impossible body-author check with this request-bound contract. A14 and A15 use record `2118426986`; the unresolved PMID `32955176` supplies no replacement identity.

Compact and details use separate exact native field selectors. Details and `--full` normalize to the same selector and retained pool identity. The pool key retains the qualified author, disclosure selection and page size; the native next offset selects its window. Checked warm pages make no request. Source policy applies before stored-pool access. Source faults retain publisher terms, and optional-key throttling keeps the deferred key action. A native empty access URL has no checked URL value; other supplied access metadata remains present. Every supplied byline row remains in source order, including missing IDs and repeated names. Its completeness remains unknown.

Existing article replies retain only the displayed native first author's optional Semantic Scholar ID. The exact author pivot reads that checked ID from the ordinary stored article header, so it survives warm pages without an enrichment request. PubMed and Europe PMC do not infer this field. The additive field changes the header, article page, edge page and recommendation output contracts; their generated schema identities advance together.

Job j15660 measured the unchanged accepted source captures on M5 with isolated homes and caches. The columns below separate the CLI newline, bare result, MCP text, Markdown and complete MCP frame. Each canonical case stores its raw CLI/MCP-text ceiling at the measured size plus fifteen percent in `max_bytes` and `max_tokens`; no group maximum or extra command flag owns a second ceiling. Publisher abstracts and derived native expectations remain outside Git.

| Native command selection | CLI bytes | Bare result bytes | MCP text bytes | Markdown bytes | Complete MCP frame bytes |
| --- | ---: | ---: | ---: | ---: | ---: |
| `2060978` compact | 2403 | 1690 | 2402 | 2538 | 5312 |
| `2060978` details and full | 23066 | 22316 | 23065 | 20683 | 48808 |
| `9845886` compact | 2494 | 1727 | 2493 | 2609 | 5486 |
| `9845886` details and full | 21041 | 20237 | 21040 | 20036 | 43842 |
| `2118426986` first compact page | 2461 | 1742 | 2460 | 2577 | 5424 |
| `2118426986` exact continuation | 2475 | 1825 | 2474 | 2591 | 5448 |
| `2118426986` warm continuation | 2433 | 1825 | 2432 | 2547 | 5360 |
| Author papers help | 1037 | 952 | 1036 | 876 | 2432 |

The same discovery measurement returned 3426 description bytes, 8014 compact tools-list bytes and 8015 raw tools-list bytes. The native paper pages supply no total, and all continuations preserve the requested record and selectors. The source archives cover every exact request used here; the missing-request inventory is empty and the implementation made no provider calls.


### ORCID normalized output measurements

The Swanton row measurements come from j15682. The remaining native and control measurements come from j15720. CLI includes its terminating newline. Bare result, actual MCP text, Markdown and full RPC remain separate measurements. Each owning canonical YAML step enforces `full_output_bytes`, `max_bytes` and `max_tokens` with the agreed rounded allowance. Publisher data and derived native expected strings remain outside Git.

The exact Swanton collection has 380 public groups with 453 ordered assertions. Its local page walk covers every native put-code once in its original group. The Butte collection has two groups and two assertions. The native group at offset 42 supplies a checked PMID pivot with native provenance. A native DOI resolver URL that is not a checked bare DOI stays unchanged as an external-ID claim and offers no inferred article pivot.

Current help measurements from j15720 are listed separately below. `list author` includes the executable papers route. Its canonical CLI/MCP owners and the existing Markdown shape own the changed entity ceilings. Other entity and global discovery ceilings remain unchanged. Whole help fits the established ceiling. The MCP description measures 3426 bytes; compact tools/list measures 8014 bytes and the raw frame measures 8015 bytes.

| Help selection | CLI | Bare | MCP text | Markdown | Full RPC |
| --- | ---: | ---: | ---: | ---: | ---: |
| Search author | 1097 | 1012 | 1096 | 988 | 2536 |
| Get author | 574 | 492 | 573 | 636 | 1420 |
| Author papers | 1116 | 1031 | 1115 | 955 | 2590 |
| List author | 2583 | 2507 | 2582 | 2419 | 5668 |
| Whole help | 88766 | 88698 | 88765 | 79356 | 187626 |

The scoped synthetic works HTTP 404 control measures 295 CLI bytes, 4 bare result bytes, 294 MCP text bytes, 308 Markdown bytes and 846 full RPC bytes in j15767. Its canonical owner enforces 340 raw bytes and 86 estimated tokens. The native person absence remains distinct from this synthetic works status control. Current papers help owns 1284 raw bytes and 321 estimated tokens; its mandatory Markdown shape owns 1099 bytes. List author owns 2971 CLI bytes, 2970 MCP text bytes, 743 estimated tokens and 2782 Markdown bytes.


## Retirement and source-failure contract

`list author` returns `usage.retired` and names `help author`, `help search author` and `help get author`. `help author` retains the complete author route list, including papers. Topic ranking, coauthor aggregation and cross-provider merging return fixed unsupported guidance to `search author TEXT` or `search article`. An ORCID name search returns its own fixed unsupported guidance. Neither route rewrites query text, chooses a candidate or joins records by name. The existing article-owned `article authors` retirement remains unchanged. Legacy `-q` and `--query` author searches remain supported.

Synthetic controls distinguish a successful empty candidate page, an absent exact record and a failed source. Both sources retain source identity and publisher terms on HTTP 429, HTTP 500, deadline, broken JSON and HTTP 200 error-body failures. An anonymous Semantic Scholar 429 retains the key-help action. The existing source failure handling required no adapter change for these controls.

The zero-I/O command owner reads the canonical retired spelling table. CLI and MCP run against a local proxy listener and an unusable cache path. Disabled author search, exact cards and papers check each source name and the noncommercial group before cache and network. Existing settings-boundary tests separately observe deferred credential reads because subprocess output cannot expose whether a credential was resolved. Warm-cache canonical cases reject disabled delivery for both selectors. Publisher bodies remain outside Git; the added replies are synthetic controls.
