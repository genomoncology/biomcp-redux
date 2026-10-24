# Biodata variants and annotations: a common model

This plan sets one model for variants and annotations that BioMCP's `biodata` crate and another biodata consumer share until the two crates merge. The other consumer projects HGVS between genomic, transcript and protein forms. BioMCP adapts public sources for agents. Each team builds its own crate under `decisions/2026-10-24-biodata-builds-its-own-parts.md`. Copied code is allowed. Both crates follow the names, wire spellings and rules below, so the later merge is a move and not a rewrite.

The plan covers three crates:

- **BioMCP biodata**, this repo's `crates/biodata`, including the variant identity work on its open ticket branch.
- **The projecting crate**, the other consumer's `biodata`. It holds the HGVS grammar and its parsed form, alteration and cytogenetic notation, coordinates and an allele key. Its sibling crate holds normalization, projection and the annotation readers.
- **The standalone crate**, the public `biodata` repository. It is the likely merge home.

## 1. What each crate has today

| Topic | BioMCP biodata | Projecting crate | Standalone crate |
| --- | --- | --- | --- |
| One-based position | `Position1(NonZeroU32)`, serializes as an integer ([coord.rs](../../crates/biodata/src/coord.rs)) | `Position1(u32)` with zero rejected and no serde | None |
| Zero-based span | `Span0 { start, end }` as u32, half-open, empty allowed, wire `{"start","end"}` ([coord.rs](../../crates/biodata/src/coord.rs)) | `Span0 { start, end }` as u32, half-open, empty allowed, no serde | `GenomicInterval { start, end }` as u64, half-open, capped at 2^53 − 1, with a constant label `zero_based_half_open` in its schema |
| Region on a chromosome | `GenomicInterval { build, contig, span, strand? }` ([coord/genomic.rs](../../crates/biodata/src/coord/genomic.rs)) | `GenomicInterval { build, contig, span }`, no strand, no serde | `GenomicLocus { assembly, sequence, interval, strand? }` |
| Build | `Build`: `GRCh37`, `GRCh38`, `unstated` | `Build`: `GRCh37`, `GRCh38`, with no unstated case and no hg19 alias | No enum. Assembly is free text with an authority |
| Chromosome | `Contig(Box<str>)`: the source spelling, kept as given | `Contig(u8)`: 1 to 22, X, Y, MT; reads an optional `chr`; writes `7`, `X`, `MT`; maps to the `NC_` accession per build from a fixed table | `sequence: String`, kept as given |
| Strand | `+`, `-` | `Plus`, `Minus` on the wire | `+`, `-`, `.`, `?` |
| Sequence accession | `RefSeqTranscript` and `EnsemblTranscript` with an optional version; MANE requires a version ([id/transcript.rs](../../crates/biodata/src/id/transcript.rs)). The ticket branch holds the variant reference as plain text | `Accession`: RefSeq (`NC_`, `NG_`, `NM_`, `NP_`, `NR_`, `NT_`, `NW_`, `XM_`, `XP_`, `XR_`), Ensembl (`ENSG`, `ENST`, `ENSP`) or LRG, with a required version and exact digit widths | Opaque text |
| Bases | None. The ticket branch holds alleles as `String` | `Base` enum of the IUPAC codes plus U; `Bases` is nonempty | Uppercase IUPAC text in `SequenceReplacement` |
| Allele | Ticket branch: `VariantRepresentation { source, notation, reference, assembly?, coordinate?, reference_allele?, alternate_allele? }` | `AlleleKey { interval, reference, alternate }`, where the reference length equals the span length. It is built only after normalization, at the rightmost placement | `SequenceReplacement { reference_id, interval, reference_state, alternate_state }`, nonempty, with the alternate required to differ from the reference |
| HGVS | Ticket branch: `VariantInput` checks the published form (accession, colon, `c.`, `g.`, `n.`, `p.`, `r.` or `m.`) and keeps the text unchanged. A private protein-change reader serves the article readers | Full winnow grammar with a parsed form: `Hgvs { predicted, reference?, change }`, nucleotide and protein edits, typed residues and a `Parsed { source, outcome }` envelope. It refuses alleles, mosaics, repeats and fusions as unsupported | Hand-written scanners for HGVS 21.1.4, with digits kept as text and no serde on the parsed form |
| Variant identifiers | Ticket branch: rsID, CAid and ClinVar Variation ID recognized by grammar in `VariantInput`; record keys and aliases are plain text | None in the crate. The readers hold rsID, CAid and ClinVar accessions as `String` and the Variation ID as `u32` | Opaque text; ClinVar Variation ID as `u64` |
| Annotations | Gene level: `OncokbCard`, `ClinGenValidity` and `CivicGene` with source labels as `Text` ([response/gene](../../crates/biodata/src/response/gene)). Ticket branch: `VariantFact { source, field, value }`, all text | In the reader crate: ClinVar records with per-submission classification kind (germline, somatic, oncogenicity, none), review status and origin; ClinGen interpretations with criteria codes; hit records with `Provenance { source, release, row_id }` and a match relation. Every label is publisher text | Adapter projections for ClinVar, MyVariant, OncoKB, cancer hotspots and gnomAD, with labels as text |
| Clinical frameworks | No enum; labels are text | No enum; labels are text | No enum, by its scope rule |
| Provenance | `Source` enum with short names; `Section` names its source; `SourceRecord { id, url? }` ([response/section.rs](../../crates/biodata/src/response/section.rs)) | Source family as text, release as read from the data file, publisher row ID | `source`, `locator`, `source_release` on each value |
| Option meaning | The source has no value | The publisher did not supply it | The source did not state it |
| Enums | `canonical_enum!` and `wire_enum!` declare each spelling once; snake_case; unknown values become a problem and leave the field absent ([macros.rs](../../crates/biodata/src/macros.rs)) | snake_case in the crate; PascalCase on several reader enums; refusal codes for unknown input | Per-type spellings |
| Serde | Both directions, through the same checks; internally tagged enums; `deny_unknown_fields`; no flatten | Serialize behind a feature, some Deserialize; externally tagged enums; flatten in writers | No derives on core types; hand-written documents |
| Schemas | `schemars`, checked in under `schemas/`, named `biodata.<entity>/<major>`, with drift and compatibility tests ([tests/schemas.rs](../../crates/biodata/tests/schemas.rs)) | None | Hand-written JSON Schema 2020-12, unversioned by rule |
| Error codes | Dotted stable codes such as `build.unknown` ([error.rs](../../crates/biodata/src/error.rs)) | Dotted stable codes such as `coordinate.zero_position` and `hgvs.edit.invalid` | Flat snake codes such as `hgvs_routing_unrecognized_prefix` |
| Toolchain | Edition 2024, Rust 1.95.0; serde, winnow and optional schemars | Edition 2024, Rust 1.95.0; winnow and optional serde | Edition 2024, Rust 1.86; serde, serde_json, XML readers, sha2 |

## 2. Agreement, conflict and merge risk

### Where the crates already agree

- Coordinates name their counting rule in the type. All three use zero-based half-open spans as the stored form, and the two working crates share the names `Position1` and `Span0` with the same meaning.
- Build spellings are `GRCh37` and `GRCh38`. Neither crate infers a build.
- No crate does I/O. Each `clippy.toml` bans file, network, process and clock access.
- Every crate keeps source labels as given and never classifies a variant itself.
- `Option` means the source has no value.
- Unknown input is refused with a stable code that never holds the input text.
- Each crate keeps the input text beside any parsed or resolved result.

### Where they conflict

1. **One name, three meanings.** `GenomicInterval` means a strand-bearing chromosome region here, a strandless region in the projecting crate, and bare bounds in the standalone crate. `Contig` means a kept source spelling here and a checked human chromosome number there.
2. **Strand on the wire.** `+` and `-` here and in the standalone crate; `Plus` and `Minus` in the projecting crate.
3. **Unstated build.** This crate puts `unstated` inside `Build`. The projecting crate has no such case.
4. **Accessions.** This crate allows unversioned transcripts and accepts six or more digits. The projecting crate requires a version and an exact digit width. The standalone crate keeps text.
5. **Integer width.** u32 in the working crates; u64 in the standalone crate.
6. **Wire contract.** This crate checks both directions and versions every schema. The projecting crate mostly serializes and has no schema. The standalone crate writes documents and schemas by hand and versions nothing.
7. **Enum tagging.** Internally tagged here; externally tagged in the projecting crate's HGVS form.
8. **Annotation shape.** Each crate has its own record per source with free-text labels. None has a shared statement type.
9. **Source identity.** An enum here; text in both others.
10. **Error code style.** Dotted in the working crates; flat snake in the standalone crate.

### Conflicts that would make a merge hard

Conflicts 1, 2, 3 and 6 change public JSON or public Rust names, so each one costs a schema major version or a breaking release if both crates ship first. Conflict 8 grows with every source either team adds. These five are settled by rules in section 4 now. Conflicts 4, 5, 7, 9 and 10 are local and can be settled at the merge.

## 3. The common model

The model follows GA4GH VRS 2.x for alleles, GA4GH Cat-VRS 1.x for protein-level and compound subjects, and GA4GH VA-Spec 1.x for statements. It follows HGVS 21.1 for notation and NCBI SPDI for the interbase allele form. It keeps the existing crates' conventions for Rust names, snake_case JSON, checked newtypes and versioned schemas. Ian chose snake_case as the native wire spelling (section 5).

### 3.1 Sequence references and coordinates

| Type | Fields and wire form | Rule | Standard |
| --- | --- | --- | --- |
| `Accession` | Canonical text, such as `"NC_000007.13"`, `"NM_004333.4"`, `"NP_004324.2"`, `"ENST00000288602.11"`, `"LRG_199p1"` | Version required. RefSeq numbers have six or nine digits, Ensembl numbers eleven. Human Ensembl prefixes only. Parse and display round-trip. The projecting crate's grammar is the one copied | HGVS reference sequence rules; RefSeq and Ensembl formats |
| `Build` | `"GRCh37"`, `"GRCh38"` | `#[non_exhaustive]`. No `unstated` case: an absent build is `Option<Build>` omitted from JSON. No hg19 or hg38 alias on the wire; adapters map provider spellings | GRC names |
| `Chromosome` | `"1"` to `"22"`, `"X"`, `"Y"`, `"MT"` | The checked human primary set, never with `chr`. Adapters read `chr7`, `chrM` and `23` and map them. This is the projecting crate's `Contig(u8)`, renamed | GRC primary assembly |
| `Contig` | Source spelling text | The source's own name, kept only where a source gives a name outside the primary set or where the source spelling matters | None |
| `Strand` | `"+"`, `"-"` | Absence is `Option<Strand>` | GFF3 and BED |
| `Position1` | Positive integer | One-based, as today | HGVS, VCF |
| `Span0` | `{"start": n, "end": n}` | Zero-based half-open, empty allowed, as today | VRS interbase, SPDI |
| `SequenceLocation` | `{"sequence": Accession, "span": Span0}` | A location on one versioned sequence. This is the variant location type | VRS `SequenceLocation` with an accession in place of a refget digest |
| `GenomicInterval` | `{"build", "chromosome", "span", "strand"?}` | A region labeled by build and chromosome, for gene spans and region search. Strand is optional | None |

The pure map between (`Build`, `Chromosome`) and the `NC_` accession is a fixed table with no I/O. The projecting crate owns it, and BioMCP copies it.

### 3.2 Alleles

| Type | Fields and wire form | Rule | Standard |
| --- | --- | --- | --- |
| `Base` | One uppercase IUPAC DNA letter | The projecting crate's enum, copied. `U` is refused; RNA notation stays in `HgvsText` | IUPAC |
| `Sequence` | A string of bases, such as `"A"`, `"GA"` or `""` | May be empty | VRS literal sequence, SPDI |
| `Allele` | `{"location": SequenceLocation, "reference": Sequence, "alternate": Sequence}` | Interbase and unpadded. The reference length equals the span length. Both sides empty is refused. A reference equal to the alternate is allowed and states the reference allele. The constructor never trims, shifts or checks the reference genome | SPDI `S:P:D:I`; VRS `Allele` with the deleted sequence kept |
| `AlleleKey` | `{"build": Build, "allele": Allele}` | Only the projecting crate's normalizer builds it. It holds the normalized placement and the build, because an accession does not always name its build: `NC_012920.1` is the mitochondrial sequence in both GRCh37 and GRCh38. It is the join key for that crate's annotations | The projecting crate's existing rule |

Equality of two `Allele` values is equality of what the sources stated. It is not variant equivalence. Equivalence needs normalization against the reference sequence, which only the projecting crate does. A VCF row converts to an `Allele` by dropping the padding base and subtracting one from the start. A gnomAD `chrom-pos-ref-alt` ID converts the same way.

### 3.3 Notation and representations

| Type | Fields and wire form | Rule | Standard |
| --- | --- | --- | --- |
| `Hgvs` (text) | Canonical text, kept exactly as given, such as `"NM_004333.4:c.1799T>A"` | Checked only for the published form: an `Accession`, a colon, one of `g.`, `c.`, `n.`, `r.`, `p.`, `m.`, `o.`, and a nonempty printable body. Name in Rust: `HgvsText` | HGVS 21.1 |
| `HgvsParsed` | The projecting crate's parsed form | Built only by the projecting crate's grammar. It never replaces the text: any value that holds a parsed form also holds the `HgvsText` it came from. Its JSON is not a public contract until the merge | HGVS 21.1 grammar |
| `ExpressionSyntax` | `"hgvs.g"`, `"hgvs.c"`, `"hgvs.n"`, `"hgvs.r"`, `"hgvs.p"`, `"hgvs.m"`, `"hgvs.o"`, `"spdi"`, `"gnomad"`, `"vcf"`, `"iscn"` | VRS spellings, plus `vcf` for `chrom:pos:ref:alt` text and `hgvs.o` for HGVS circular genomic text, which `HgvsText` accepts. A later GA4GH export leaves out `hgvs.o` and `vcf` expressions, which VRS does not list | VRS `Expression.syntax` |
| `Expression` | `{"syntax", "value", "syntax_version"?}` | The value is checked against its syntax's published form at construction | VRS `Expression` |
| `Representation` | `{"source", "expression", "build"?, "allele"?}` | One source's own form of a variant. `allele` is present only when the source gave reference, alternate and interbase bounds, or a padded VCF row the adapter converted under section 3.2. `build` is the source's stated build | VRS `expressions`, ClinGen Allele Registry genomic and transcript alleles |

A MyVariant `_id` such as `chr7:g.140453136A>T` is not reference-qualified HGVS. It becomes a `Representation` with syntax `hgvs.g`, the value as given and `build` set to GRCh37 only when the request named that build. A ClinGen Allele Registry allele becomes one `Representation` per genomic and transcript allele, each with its `allele` and its stated build.

### 3.4 Variant identifiers

`VariantId` is one enum. Each variant serializes as its published text, and parsing picks the variant from that text alone.

| Variant | Wire text | Check | Standard |
| --- | --- | --- | --- |
| `Rs` | `"rs113488022"` | `rs` and a positive number, stored as u64 | dbSNP |
| `Ca` | `"CA123643"`, `"CA000434"` | `CA` and a positive number written with at least six digits, zero-padded to six as the registry publishes it. Text with fewer than six digits, or more than six digits starting with zero, is refused. It serializes as that canonical text and compares by its number | ClinGen Allele Registry nucleotide allele |
| `Pa` | `"PA..."` | `PA` and a positive number | ClinGen Allele Registry protein allele; confirm the form against the registry when built |
| `ClinVarVariation` | `"clinvar:13961"` | Positive number with the `clinvar:` prefix | identifiers.org and Bioregistry `clinvar` |
| `ClinVarAllele` | `"clinvar.allele:..."` | Positive number | ClinVar AlleleID |
| `ClinVarAccession` | `"VCV000013961.98"`, `"RCV..."`, `"SCV..."` | Three-letter kind, nine digits and an optional version | ClinVar accessions |
| `Vrs` | `"ga4gh:VA.<32 characters>"` | Prefix and a 32-character base64url digest; kept as given by a source until a crate computes it | VRS 2.x computed identifier |

BioMCP's `VariantInput` stays a BioMCP query type. It reads `VariantId` text, `HgvsText` and BioMCP's command conveniences, such as a bare ClinVar number.

A VRS identifier depends on the refget digest of the reference sequence, which needs sequence data. The common crate defines the type. The projecting crate computes it later, after the merge, when the crate can depend on a hash library.

### 3.5 The variant record

`Variant` is the biodata entity for one variant. It holds every fact its identity sources give and no disclosure logic.

```text
Variant {
    ids: [VariantId],                // every identifier the sources state, never inferred
    representations: [Representation], // every source-qualified form
    genes: [GeneSymbol],             // as the sources state them
}
```

The projecting crate's normalized view extends this with `AlleleKey` and its projections in its own types. A `Statement` may be built as a view from a crate's own reader records, which stay the source for that crate's other writers. BioMCP's card, header and sections pick from `Variant` in BioMCP response types, as architecture section 3.10 says.

### 3.6 Annotations as statements

Every variant assertion whose source classification belongs to a proposition this model defines becomes one `Statement`. A statement never merges two sources and never maps one framework onto another. A source classification outside those propositions, such as ClinVar's drug response, risk factor, protective, affects, association, association not found, confers sensitivity, other or not provided, stays in the consumer's native source record with its label, IDs, domain, condition, review status, dates and provenance, and carries `statement_unmapped: true` instead of a `Statement`. The same holds for a source row whose subject the model cannot reference without inventing an identifier, such as an OncoKB implication's alteration group, which carries no source profile record. A new proposition or subject kind for such rows needs both PMs' agreement before either crate builds it.

```text
Statement {
    subject: VariantRef,            // what the statement is about
    proposition: Proposition,       // what kind of claim
    predicate: Predicate?,          // the claim's direction of effect, when stated
    direction: Direction?,          // supports, disputes, neutral
    classification: Term,           // the source's call, label always kept
    strength: Term?,                // evidence level, tier level or review status
    condition: Concept?,            // disease or tumor type
    therapies: [Concept],           // for therapeutic response
    allele_origin: Term?,           // germline, somatic and the source's other values
    gene_context: GeneSymbol?,
    criteria: [Text],               // for example ACMG codes met, as stated
    citations: [Pmid],
    attribution: Attribution,       // source, record, version, release, submitter, date
}
```

| Type | Fields and wire form | Rule | Standard |
| --- | --- | --- | --- |
| `VariantRef` | Internally tagged by `"level"`: `allele` with `id` or `hgvs`; `protein` with `gene` and `hgvs` or `label`; `profile` with `record`, `label` and member refs, each an `allele` or `protein` ref; a profile never contains a profile | A reference by ID and label, never an embedded entity. A protein-level claim such as BRAF V600E never becomes a genomic allele. A compound profile stays a profile | Cat-VRS `CategoricalVariant`, its protein-consequence recipe, and CIViC molecular profiles |
| `Proposition` | `pathogenicity`, `oncogenicity`, `clinical_significance`, `therapeutic_response`, `diagnostic`, `prognostic`, `functional_impact` | `#[non_exhaustive]` | VA-Spec proposition classes |
| `Predicate` | snake_case forms of the VA-Spec predicates, such as `is_causal_for`, `predicts_sensitivity_to`, `predicts_resistance_to` | Confirm the full list against VA-Spec 1.1 when coded | VA-Spec |
| `Direction` | `supports`, `disputes`, `neutral` | CIViC `DOES_NOT_SUPPORT` maps to `disputes` | VA-Spec `direction` |
| `Term` | `{"label": Text, "coding"?: Coding}` | `label` is the source's whole text, always kept. `coding` is present only when the label is a documented value of a closed vocabulary below | VA-Spec `MappableConcept` |
| `Coding` | Internally tagged by `"system"`, with a checked `"code"` | Each system is one Rust enum of its documented codes | VA-Spec `Coding` |
| `Concept` | `{"id"?: Curie, "label": Text}` | `Curie` is checked `prefix:local` text, such as `MONDO:0005105` or `NCIT:C3224` | VA-Spec `MappableConcept` |
| `Attribution` | `{"source", "record"?, "version"?, "release"?, "submitter"?, "evaluated"?, "url"?}` | `record` is the source's own ID for this assertion. `version` is that record's version. `release` is the dataset release as the source labels it. `evaluated` is a `PartialDate` | VA-Spec `contributions` and `reportedIn`; the projecting crate's `Provenance`; the standalone crate's `source_release` |

The first coding systems are those with a published closed vocabulary. Each code below is a snake_case spelling. Export to VA-Spec uses the VA-Spec code text, such as `likely pathogenic`.

| `system` | Codes | Source |
| --- | --- | --- |
| `acmg_amp_2015` | `pathogenic`, `likely_pathogenic`, `uncertain_significance`, `likely_benign`, `benign` | Richards 2015; VA-Spec ACMG 2015 profile |
| `clingen_low_penetrance_2024` | `pathogenic_low_penetrance`, `likely_pathogenic_low_penetrance`, `established_risk_allele`, `likely_risk_allele`, `uncertain_risk_allele` | VA-Spec ACMG profile |
| `amp_asco_cap_2017_tier` | `tier_i`, `tier_ii`, `tier_iii`, `tier_iv` | Li 2017; VA-Spec AAC 2017 profile |
| `amp_asco_cap_2017_level` | `level_a`, `level_b`, `level_c`, `level_d` | Same |
| `clingen_cgc_vicc_2022` | `oncogenic`, `likely_oncogenic`, `uncertain_significance`, `likely_benign`, `benign` | Horak 2022; VA-Spec CCV 2022 profile |
| `clinvar_review_status` | ClinVar's review status values | ClinVar XML schema |
| `civic_evidence_level` | `a`, `b`, `c`, `d`, `e` | CIViC |
| `oncokb_level` | `level_1`, `level_2`, `level_3a`, `level_3b`, `level_4`, `level_r1`, `level_r2`, `level_dx1` to `level_dx3`, `level_px1` to `level_px3` | OncoKB API |
| `oncokb_oncogenicity` | `oncogenic`, `likely_oncogenic`, `likely_neutral`, `inconclusive`, `resistance`, `unknown` | OncoKB API |

A ClinVar germline label such as `Likely pathogenic` gets `acmg_amp_2015` coding only when the submission states that method. An aggregate label such as `Conflicting classifications of pathogenicity` keeps its label and has no coding. A CIViC assertion's `Tier I - Level A` becomes `amp_asco_cap_2017_tier` `tier_i` in `classification` and `amp_asco_cap_2017_level` `level_a` in `strength`, both with the CIViC label.

### 3.7 Population frequency and predictor scores

| Type | Fields | Standard |
| --- | --- | --- |
| `PopulationFrequency` | `subject`, `dataset`, `build`, `group`?, `allele_count`?, `allele_number`?, `homozygotes`?, `frequency`?, `filters`, `attribution` | VA-Spec cohort allele frequency study result. A missing dataset is absent, never zero. No frequency is computed from counts |
| `PredictorScore` | `subject`, `method`, `score` as a finite number, `label`?, `attribution` | Source values as given. No score blending |

### 3.8 Conventions

- Rust names are the type names above. JSON field names are snake_case.
- Enums with data are internally tagged with a field named for the choice: `level`, `system`, `resolution`, `state`.
- Closed sets parse only their canonical spellings. Provider spellings map in each adapter. An unknown provider value leaves the field absent and records a problem, or keeps the source label in a `Term` with no coding.
- `Option` means the source has no value. JSON omits absent fields and empty lists and never prints `null`.
- Every identifier and checked text value serializes as its canonical text and parses back to an equal value.
- Lists hold every stated value in source order. No list is cut in a biodata type.

### 3.9 Who builds what now

| Part | Builder | The other team |
| --- | --- | --- |
| `Accession`, `Base`, `Chromosome`, the build-to-accession table | Projecting crate, already built | BioMCP copies them, renaming nothing |
| `HgvsParsed`, alteration and cytogenetic notation, normalization, `AlleleKey`, projection, VRS digest computation | Projecting crate | BioMCP uses none of them and passes `HgvsText` to its sources |
| `HgvsText`, `Expression`, `Representation`, `VariantId`, `Variant` | BioMCP, on its variant identity ticket | The projecting crate copies them when its readers need them |
| `Statement`, `Term`, `Coding`, `Concept`, `Attribution`, `VariantRef` | BioMCP, with its ClinVar, ClinGen and CIViC sections | The projecting crate maps its ClinVar and ClinGen readers onto them when it next touches those readers |
| `PopulationFrequency`, `PredictorScore` | BioMCP, with its population and prediction sections | The projecting crate maps its gnomAD and predictor readers |
| `Position1`, `Span0`, `Strand`, `Build`, `GenomicInterval`, `SequenceLocation`, `Allele` | Both, identical | Each change goes to both PMs before it lands |

BioMCP's variant identity branch changes before it lands: the allele strings become `Sequence`; the assembly text becomes `Option<Build>`; the reference text becomes `Accession`; record keys and aliases become `VariantId`; the generic `VariantFact` becomes `Statement`, `PopulationFrequency` and `PredictorScore`. BioMCP's gene schema moves its `build` field to `Option<Build>` and its `contig` to `chromosome`, raising `biodata.gene` to major version 2.

## 4. Rules both teams follow now

1. **Names.** Both crates use the type names in section 3. A type with one of these names has the meaning given here. A new public variant or annotation type gets a name both PMs accept before it lands.
2. **Wire spellings.** JSON fields are snake_case. Enum values are snake_case, except published spellings that are themselves the standard: `GRCh37`, `GRCh38`, `+`, `-`, chromosome names, VRS expression syntaxes and identifier text.
3. **Coordinate labels.** A field holding a coordinate has a type that names its counting rule: `Position1` or `Span0`. No bare integer holds a coordinate. Interbase is the stored form for alleles.
4. **Identifier formats.** Identifiers use the wire text in section 3.4 and the accession rules in section 3.1. Accessions in variant values carry their version.
5. **Schemas.** Both crates name schemas `biodata.<entity>/<major>`, such as `biodata.allele/1` and `biodata.statement/1`. The projecting crate adds generated schemas when it first serializes a common type in output. Removing, renaming or retyping a field raises the major version.
6. **Serde.** Deserialization runs the same checks as construction. No `flatten`. Unknown fields are refused.
7. **No disclosure logic in biodata.** Headers, cards, sections, caps, pages and next steps are consumer response types, outside the shared model.
8. **No interpretation in biodata.** No type ranks, votes on or merges classifications. No constructor fetches, normalizes or checks a reference base.
9. **Error codes.** Codes are dotted, `area.thing.problem`, such as `allele.reference_length` or `id.rs.invalid`. Messages never hold input text.
10. **Shared public fixtures.** Both crates carry the same small fixture file of public variants under `tests/fixtures/variant/`, with identical bytes. Each row gives the input text and its expected values in each form: `Allele` JSON, SPDI, VCF, gnomAD ID, HGVS text and identifiers. Each expected value is checked against a public source when the row is added. Both crates run every row through their copies. The first rows are:
    - BRAF c.1799T>A: `NC_000007.13:g.140453136A>T`, `NC_000007.14:g.140753336A>T`, `NM_004333.4:c.1799T>A`, `NP_004324.2:p.Val600Glu`, `CA123643`, `clinvar:13961`.
    - rs113488022 with its three alternate alleles.
    - EGFR L858R as MyVariant's `chr7:g.55259515T>G` on GRCh37.
    - An EGFR exon 19 deletion: `NC_000007.13:g.55242469_55242471del`.
    - An insertion, with its empty-reference `Allele` and padded VCF form.
    - TP53 `NM_001126114.2:c.1015G>C`, which resolves.
    - TP53 `NC_000017.10:g.7576633A>G`, which the registry rejects for a wrong reference base.
    - CHEK2 `NM_007194.4:c.1100del`.
    - TERT `NM_198253.3:c.-124C>T`.
    - One mitochondrial variant on `NC_012920.1`, whose `AlleleKey` rows for GRCh37 and GRCh38 differ only by build.
    - One reference allele whose reference equals its alternate, which every crate accepts.
    - One allele text containing `U`, which every crate refuses.
    - One ClinVar germline statement, one CIViC assertion with a tier and level, and one ClinGen expert panel interpretation, each with source labels.
11. **Copies.** A copied file keeps the owner's spelling and tests. A fix goes to the owner first, through the PMs, and the copy follows.
12. **Public text.** Shared types and docs name no consumer project.
13. **Legacy writers.** A writer that must reproduce an existing product's output byte for byte, including its nulls, empty rows and chromosome prefixes, is outside rules 2, 5 and 8 of this list and outside section 3.8. The shared types feed it; its output keeps the legacy shape.
14. **Lenient chromosome input.** A crate may accept provider spellings such as `chr7`, `M` or `23` in one adapter helper that returns a `Chromosome`. The `Chromosome` type itself parses only its canonical spellings.

## 5. Merge and wire style

Ian ruled on 2026-10-24:

- **Merge.** Ian decides when and where the crates merge, after both projects are done. Neither team plans or proposes the merge. The likely home is a new public crate without commit history. Until then each team keeps its own crate and follows this plan.
- **Wire style.** JSON field names stay snake_case as in section 3. A GA4GH VRS and VA-Spec export, which uses camelCase, can be added later as a fixed field map.
