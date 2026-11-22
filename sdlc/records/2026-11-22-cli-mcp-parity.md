# Recent CLI and MCP parity

This inventory covers the tickets landed on main from November 19 through November 22, 2026, including tickets landed in another ticket's batch. The surface column records what the original fix affected. “Both” includes shared command code. The parity check compares the JSON that an ordinary CLI caller and the default `biomcp` MCP tool receive. Each replay prepares the two callers independently. The typed cases also verify that the tool translates the corrected next call into typed arguments.

```sh
cargo test --locked -p biomcp --test cases mcp-parity/recent
cargo test --locked -p biomcp --test cases mcp-parity/recent/live -- --ignored
```

The second command makes two live ClinicalTrials.gov reads. It exercises one optional witness. A source change between the reads can change the result.

| Landed fix | Surface | MCP status | Witness or reason |
|---|---|---|---|
| [1055 companion list first read fetches itself](../tickets/1055-companion-list-first-read-fetches-itself.md) | Both | Same | Local FDA fixture, first read on each surface. |
| [1056 companion list biomarker filter misses rows](../tickets/1056-companion-list-biomarker-filter-misses-rows.md) | Both | Same | `diagnostic/native-cdx` step 6 |
| [1057 refusals name the real cause](../tickets/1057-refusals-name-the-real-cause.md) | Both | Same | `drug/markets` step 9 |
| [1058 absent label section says so](../tickets/1058-absent-label-section-says-so.md) | Both | Same | `drug/label-printed-names` step 1 |
| [1059 status words match what happened](../tickets/1059-status-words-match-what-happened.md) | Both | Same | `drug/approvals-incomplete-top` step 1 |
| [1060 label search rows show drug name](../tickets/1060-label-search-rows-show-drug-name.md) | Both | Same | `drug/label-names-from-text` step 1 |
| [1061 lists page one way](../tickets/1061-lists-page-one-way.md) | Both | Same | `pathway/genes-fields` step 2 |
| [1062 orphanet id get and next step work](../tickets/1062-orphanet-id-get-and-next-step-work.md) | Both | Same | `disease/orphanet-id` step 1 |
| [1063 accept registry spellings and application ids](../tickets/1063-accept-registry-spellings-and-application-ids.md) | Both | Same | `trial/printed-phase` step 1 / `drug/label-application` step 1 |
| [1064 usage refusals say missing or invalid](../tickets/1064-usage-refusals-say-missing-or-invalid.md) | Both | Same | `grammar/usage-names-the-cause` step 1 |
| [1065 companion list biomarker filter handles not](../tickets/1065-companion-list-biomarker-filter-handles-not.md) | Both | Same | `diagnostic/native-cdx` step 8 |
| [1066 label section change history](../tickets/1066-label-section-change-history.md) | Both | Same | `drug/label-history-rinvoq` step 1 |
| [1067 fda drug safety communications source](../tickets/1067-fda-drug-safety-communications-source.md) | Both | Same | `drug/fda-dsc-navigation` step 1 |
| [1068 pdf reader opens form objects and flags garbled text](../tickets/1068-pdf-reader-opens-form-objects-and-flags-garbled-text.md) | Both | Same | `diagnostic/fda-letter-partial` step 1 |
| [1069 pdf reader decodes fonts without character maps](../tickets/1069-pdf-reader-decodes-fonts-without-character-maps.md) | Both | Same | `diagnostic/fda-letter-map-read-again` step 1 |
| [1070 first read fetch backs off and says what happened](../tickets/1070-first-read-fetch-backs-off-and-says-what-happened.md) | Both | Same | Local FDA fixture, refusal of a first read over the byte limit on each surface. |
| [1071 fields choose columns in sections](../tickets/1071-fields-choose-columns-in-sections.md) | Both | Same | `drug/approvals-fields` step 1 |
| [1072 article open access check answers in time or is asked for](../tickets/1072-article-open-access-check-answers-in-time-or-is-asked-for.md) | Both | Same | `article/search/pubmed-access-timeout` step 1 |
| [1073 unknown words after a command are refused](../tickets/1073-unknown-words-after-a-command-are-refused.md) | Both | Same | `gene/unknown-words` step 3 |
| [1074 hosted server caps pdfs read at once](../tickets/1074-hosted-server-caps-pdfs-read-at-once.md) | Hosted MCP | Different by design | Hosted PDF concurrency guard. The CLI has no shared request pool. |
| [1075 pdf reads run in a capped child process](../tickets/1075-pdf-reads-run-in-a-capped-child-process.md) | Both | Same | `diagnostic/fda-letter-damaged` step 1 |
| [1076 rows do not repeat what the reply can say once](../tickets/1076-rows-do-not-repeat-what-the-reply-can-say-once.md) | Both | Same | `drug/label-shared-budget` step 1 |
| [1077 update rustix past its advisory](../tickets/1077-update-rustix-past-its-advisory.md) | Build and checks | Different by design | Dependency audit. No application request changed. |
| [1078 fields covers the remaining sections](../tickets/1078-fields-covers-the-remaining-sections.md) | Both | Same | `gene/sections/constraint-fields` step 1 |
| [1079 pdf symbol font characters read as standard characters](../tickets/1079-pdf-symbol-font-characters-read-as-standard-characters.md) | Both | Same | `diagnostic/pdf-symbol-characters` step 1 |
| [1080 pdf reader keeps numbers whole](../tickets/1080-pdf-reader-keeps-numbers-whole.md) | Both | Same | The same Rinvoq history request as 1066 reads the affected archived PDF. The PDF reader cases assert the numbers directly. |
| [1081 two small reply faults from the hand check](../tickets/1081-two-small-reply-faults-from-the-hand-check.md) | Both | Same | `drug/approvals-default-page` step 3 |
| [1082 label history reads old format labels](../tickets/1082-label-history-reads-old-format-labels.md) | Both | Same | `drug/label-history-lamictal` step 1 |
| [1083 label history help uses a neutral example](../tickets/1083-label-history-help-uses-a-neutral-example.md) | Both | Same | `drug/label-history-example` step 2 |
| [1084 companion list rows are small and searchable by name](../tickets/1084-companion-list-rows-are-small-and-searchable-by-name.md) | Both | Same | `diagnostic/native-cdx` step 4 |
| [1085 refusals agents hit most say what to change](../tickets/1085-refusals-agents-hit-most-say-what-to-change.md) | Both | Same | `drug/label-history-refusals` step 1 |
| [1086 drug search finds a class by mechanism](../tickets/1086-drug-search-finds-a-class-by-mechanism.md) | Both | Same | `drug/label-mechanism` step 1 |
| [1087 stale source hides a drugs other names](../tickets/1087-stale-source-hides-a-drugs-other-names.md) | Both | Same | `drug/source-aliases-review-evidence` step 1 |
| [1088 fda potential safety signals source](../tickets/1088-fda-potential-safety-signals-source.md) | Both | Same | `drug/fda-safety-signals` step 1 |
| [1089 who lookups on stale data do not say absent](../tickets/1089-who-lookups-on-stale-data-do-not-say-absent.md) | Both | Same | Expired WHO PQ snapshot, compared in the existing source-age case. |
| [1090 search current label sections for phrases](../tickets/1090-search-current-label-sections-for-phrases.md) | Both | Same | `drug/label-find` step 1 |
| [1091 select numbered label subsections](../tickets/1091-select-numbered-label-subsections.md) | Both | Same | `drug/label-subsections` step 1 |
| [1093 search withdrawn cancer accelerated approvals](../tickets/1093-search-withdrawn-cancer-accelerated-approvals.md) | Both | Same | `drug/fda-withdrawn-approvals` step 1 |
| [1094 query the purple book roster and interchangeability](../tickets/1094-query-the-purple-book-roster-and-interchangeability.md) | Both | Same | `drug/purple-book-age` step 2 |
| [1095 search trials by acronym and official title](../tickets/1095-search-trials-by-acronym-and-official-title.md) | Both | Same | `trial/acronym-and-official-title` step 1 |
| [1096 filter trial sites by status state and radius](../tickets/1096-filter-trial-sites-by-status-state-and-radius.md) | Both | Same | `trial/sites/filters` step 1 |
| [1097 return compact trial arms for an intervention](../tickets/1097-return-compact-trial-arms-for-an-intervention.md) | Both | Same | `trial/matched-arms` step 1 |
| [1098 look up clingen and gencc curations by disease](../tickets/1098-look-up-clingen-and-gencc-curations-by-disease.md) | Both | Same | `disease/gene-validity` step 2 |
| [1099 find diseases by a set of hpo terms](../tickets/1099-find-diseases-by-a-set-of-hpo-terms.md) | Both | Same | `disease/hpo-release` step 1 |
| [1100 show orphanet to omim mapping relations](../tickets/1100-show-orphanet-to-omim-mapping-relations.md) | Both | Same | `disease/orphadata-xrefs` step 2 |
| [1101 choose release and mane transcript for gnomad constraint](../tickets/1101-choose-release-and-mane-transcript-for-gnomad-constraint.md) | Both | Same | `gene/sections/constraint-release` step 3 |
| [1102 show reactome release and distinct genes in pathway genes](../tickets/1102-show-reactome-release-and-distinct-genes-in-pathway-genes.md) | Both | Same | `pathway/member-count` step 1 |
| [1103 make multi id syntax consistent](../tickets/1103-make-multi-id-syntax-consistent.md) | Both | Same | `gene/multi-id` step 1 |
| [1104 find companion diagnostics by regimen partner](../tickets/1104-find-companion-diagnostics-by-regimen-partner.md) | Both | Same | `diagnostic/fda-cdx-regimen-partner` step 2 |
| [1105 show approval letter ids in compact histories](../tickets/1105-show-approval-letter-ids-in-compact-histories.md) | Both | Same | `drug/approval-document-fields` step 1 |
| [1106 keep drug label fallback on the same application](../tickets/1106-keep-drug-label-fallback-on-the-same-application.md) | Both | Same | `drug/label-fallback-application` step 1 |
| [1107 find the holder label when brand search only finds repackagers](../tickets/1107-find-the-holder-label-when-brand-search-only-finds-repackagers.md) | Both | Same | `drug/label-holder-recovery` step 1 |
| [1108 rank exact drug names ahead of incidental matches](../tickets/1108-rank-exact-drug-names-ahead-of-incidental-matches.md) | Both | Same | `drug/ranked-names` step 1 |
| [1109 show the scope of trial text search totals](../tickets/1109-show-the-scope-of-trial-text-search-totals.md) | Both | Same | `trial/acronym-and-official-title` step 4 |
| [1110 size trial arm pages after field selection](../tickets/1110-size-trial-arm-pages-after-field-selection.md) | Both | Same | `trial/get/arm-projection-bytes` step 1 |
| [1111 read explicit supplement purpose in fda letters](../tickets/1111-read-explicit-supplement-purpose-in-fda-letters.md) | Both | Same | `drug/supplement-purpose-changes` step 1 |
| [1112 name the cause of drug get refusals](../tickets/1112-name-the-cause-of-drug-get-refusals.md) | Both | Same | `drug/get-refusals` step 1 |
| [1113 name the source limit in gene search refusals](../tickets/1113-name-the-source-limit-in-gene-search-refusals.md) | Both | Fixed here | `gene/search-source-refusals` step 2 / typed calls reuse the source refusal |
| [1114 name the source and section limits in gene get refusals](../tickets/1114-name-the-source-and-section-limits-in-gene-get-refusals.md) | Both | Fixed here | `gene/sections/source-refusal-clingen` step 1 / typed calls reuse the source refusal |
| [1115 name the source and section limits in disease get refusals](../tickets/1115-name-the-source-and-section-limits-in-disease-get-refusals.md) | Both | Same | `disease/get-refusals` step 1 |
| [1116 name the hpo limit in phenotype get refusals](../tickets/1116-name-the-hpo-limit-in-phenotype-get-refusals.md) | Both | Same | `phenotype/get-refusals` step 1 |
| [1117 name the source limit in pathway get refusals](../tickets/1117-name-the-source-limit-in-pathway-get-refusals.md) | Both | Fixed here | `pathway/source-refusal` step 1 / typed calls reuse the source refusal |
| [1118 teach efficient biomcp reads and recovery](../tickets/1118-teach-efficient-biomcp-reads-and-recovery.md) | CLI skill | Fixed here | `trial/skill-examples` step 1 / `trial/skill-examples` step 2 / `pgx/skill-example` step 1 / the same guide is served by both MCP modes |
| [1119 ema first read reads the referrals file](../tickets/1119-ema-first-read-reads-the-referrals-file.md) | Both | Same | Local EMA fixture, first read on each surface. |
| [1120 disease id says retired record or gene entry](../tickets/1120-disease-id-says-retired-record-or-gene-entry.md) | Both | Same | `disease/omim-gene-entry` step 1 |
| [1121 trim drug help back under its old size](../tickets/1121-trim-drug-help-back-under-its-old-size.md) | Both | Same | `drug/help` step 1 |
| [1122 purple book says how old its snapshot is](../tickets/1122-purple-book-says-how-old-its-snapshot-is.md) | Both | Same | `drug/purple-book-age` step 1 |
| [1125 get drug help lists only sources get accepts](../tickets/1125-get-drug-help-lists-only-sources-get-accepts.md) | Both | Same | `drug/help` step 2 |
| [1126 label section limit matches what help advertises](../tickets/1126-label-section-limit-matches-what-help-advertises.md) | Both | Same | `drug/label-section-limit` step 1 |
| [1127 restore test guards weakened by recent landings](../tickets/1127-restore-test-guards-weakened-by-recent-landings.md) | Both | Same | `drug/fda-dsc-invalid-page-dates` and `drug/fda-withdrawn-approvals-invalid-date` step 1. Other changes restored test guards. |
| [1128 partial gene reply names what is missing](../tickets/1128-partial-gene-reply-names-what-is-missing.md) | Both | Same | `gene/batch-mygene-disabled` step 1 |
| [1129 skill names label indication search and fda lists](../tickets/1129-skill-names-label-indication-search-and-fda-lists.md) | CLI skill | Fixed here | `drug/indication` step 6 / `drug/fda-withdrawn-approvals` step 4 / `drug/fda-safety-signals` step 5 / the same guide is served by both MCP modes |

The comparison notes also propose trial enrollment and acronym display, a default label-text fallback, and changing the layout of label text and pagination. Those proposals had no landed change in the period. The landed work covers label indication and mechanism search, label history, FDA lists, companion diagnostics, trial sites and flows, disease curations and HPO queries, and pathway member paging. The guide carries the usage advice for these through `help skill render`. The original CLI `help skill render` still describes the CLI renderer.

Typed tools take explicit sections and return typed next calls. A CLI word whose meaning depends on its position has no equivalent ambiguity in an explicit MCP field. Other unknown typed options keep their own list of allowed argument names. The hosted PDF concurrency limit and hosted refusals have their own tests.
