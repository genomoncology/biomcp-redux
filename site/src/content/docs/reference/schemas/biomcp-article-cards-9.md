---
title: Reply of biomcp get article (biomcp.article-cards/9)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get article: the
  fields of schema biomcp.article-cards/9."
goal: "Parse the biomcp.article-cards/9 reply: see what each field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article-cards/9
kind: reference
verified: 2026-11-09
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get article`](/reference/commands/get-article/), [`biomcp article text`](/reference/commands/article-text/), [`biomcp get`](/reference/commands/get/) and [`biomcp article`](/reference/commands/article/) reply with `schema` set to `biomcp.article-cards/9`. Read this page when your code parses that reply: it shows the fields this schema defines. [Shared types](/reference/schemas/types/) defines the rest.

`biomcp schema biomcp.article-cards/9` prints the full JSON Schema.

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [Reply_ArticleCards](#reply-articlecards). `query` holds [Reply_ArticleGet](#reply-articleget). `query` repeats the request as BioMCP read it.

### Reply_ArticleCards

Cards in caller order.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_ArticleCardItem](#reply-articlecarditem) | Per-input answers. Optional. |

### Reply_ArticleGet

Get one to twenty article inputs in caller order.

| Field | Type | Meaning |
| --- | --- | --- |
| `input` | list of [Reply_ArticleInput](/reference/schemas/types/#reply-articleinput) | PubMed, PMC or DOI identifiers in input order. |
| `section` | list of [Reply_ArticleSectionWord](#reply-articlesectionword) | Requested [section words](/concepts/progressive-disclosure/); trailing words and --section share this field. Optional. |

### Other types in this reply

35 more types appear only in this reply: [Reply_AbstractPart](#reply-abstractpart), [Reply_AnnotatedEntity](#reply-annotatedentity), [Reply_AnnotationKind](#reply-annotationkind), [Reply_Article](#reply-article), [Reply_ArticleCardItem](#reply-articlecarditem), [Reply_ArticleMatch](#reply-articlematch), [Reply_ArticleMatchKind](#reply-articlematchkind), [Reply_ArticleResolution](#reply-articleresolution), [Reply_ArticleSection](#reply-articlesection), [Reply_ArticleSectionWord](#reply-articlesectionword), [Reply_ArticleSections](#reply-articlesections), [Reply_Capped_AnnotatedEntity](#reply-capped-annotatedentity), [Reply_Citations](#reply-citations), [Reply_Correction](#reply-correction), [Reply_CorrectionKind](#reply-correctionkind), [Reply_CorrectionRelation](#reply-correctionrelation), [Reply_FigureCaption](#reply-figurecaption), [Reply_FileKind](#reply-filekind), [Reply_FullText](#reply-fulltext), [Reply_Indexing](#reply-indexing), [Reply_MeshHeading](#reply-meshheading), [Reply_MeshQualifier](#reply-meshqualifier), [Reply_PdfLink](#reply-pdflink), [Reply_Section_Capped_AnnotatedEntity](#reply-section-capped-annotatedentity), [Reply_Section_Citations](#reply-section-citations), [Reply_Section_FullText](#reply-section-fulltext), [Reply_Section_Indexing](#reply-section-indexing), [Reply_Section_PdfLink](#reply-section-pdflink), [Reply_Section_Supplements](#reply-section-supplements), [Reply_Section_Tldr](#reply-section-tldr), [Reply_SupplementFile](#reply-supplementfile), [Reply_Supplements](#reply-supplements), [Reply_TextPart](#reply-textpart), [Reply_TextPartKind](#reply-textpartkind), [Reply_Tldr](#reply-tldr). Open the list below to read their fields.

<details>
<summary>Fields of the 35 other types</summary>

### Reply_AbstractPart

One source-labelled abstract paragraph.

| Field | Type | Meaning |
| --- | --- | --- |
| `label` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Source label, when supplied. Optional. |
| `text` | [Reply_Markdown](/reference/schemas/types/#reply-markdown) | Whole paragraph text. |

### Reply_AnnotatedEntity

One normalized [entity](/concepts/progressive-disclosure/) with its mention count.

| Field | Type | Meaning |
| --- | --- | --- |
| `count` | integer (uint64) | Number of annotation mentions. |
| `entrez_id` | [Reply_NcbiGeneId](/reference/schemas/types/#reply-ncbigeneid) or null | Gene identity, including a variant's associated gene. Optional. |
| `hgvs` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Source HGVS expression for a variant. Optional. |
| `kind` | [Reply_AnnotationKind](#reply-annotationkind) | Source concept type. |
| `mesh` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Source MeSH identity for a disease or chemical. Optional. |
| `name` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Source normalized name. Optional. |
| `rsid` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Source reference SNP identity for a variant. Optional. |

### Reply_AnnotationKind

Normalized annotation category in deterministic tie order.

Type: string.

### Reply_Article

Article facts with at least one checked identity.

| Field | Type | Meaning |
| --- | --- | --- |
| `abstract_parts` | list of [Reply_AbstractPart](#reply-abstractpart) | Whole abstract, preserving labels and source order. Optional. |
| `authors` | [Reply_Capped_Text](/reference/schemas/types/#reply-capped-text) | Authors in source order. |
| `corrections` | list of [Reply_Correction](#reply-correction) | Related correction notices or originals. Optional. |
| `date` | [Reply_PartialDate](/reference/schemas/types/#reply-partialdate) or null | Publication date at source precision. Optional. |
| `doi` | [Reply_Doi](/reference/schemas/types/#reply-doi) | DOI assigned to this article. Optional. |
| `electronic_date` | [Reply_PartialDate](/reference/schemas/types/#reply-partialdate) or null | Electronic publication date when different from the publication date. Optional. |
| `europepmc_id` | [Reply_EuropePmcId](/reference/schemas/types/#reply-europepmcid) | Europe PMC identity when this card comes from its core record. Optional. |
| `journal` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Journal name. Optional. |
| `pmcid` | [Reply_Pmcid](/reference/schemas/types/#reply-pmcid) | PMC identity assigned to this article. Optional. |
| `pmid` | [Reply_Pmid](/reference/schemas/types/#reply-pmid) | PubMed identity. Optional. |
| `text_withheld` | [Reply_WithheldArticleText](/reference/schemas/types/#reply-withheldarticletext) or null | Text withheld because the record's terms do not permit this use. Optional. |
| `title` | [Reply_Text](/reference/schemas/types/#reply-text) | Whole title with inline text preserved. Optional. |
| `types` | list of [Reply_PublicationType](/reference/schemas/types/#reply-publicationtype) | Every clinically meaningful source publication type. Optional. |

### Reply_ArticleCardItem

Answer to one input; shared DOIs may expand to several consecutive items.

| Field | Type | Meaning |
| --- | --- | --- |
| `answer` | [Reply_ArticleResolution](#reply-articleresolution) or null | Absent only when a provider failed. Optional. |
| `input` | [Reply_ArticleInput](/reference/schemas/types/#reply-articleinput) | Original selector. |
| `more` | list of [Reply_ArticleSection](#reply-articlesection) | Applicable sections omitted by this request. Optional. |
| `sections` | [Reply_ArticleSections](#reply-articlesections) or null | Requested section states for a resolved card. Optional. |
| `status` | [Reply_Status](/reference/schemas/types/#reply-status) | This card's outcome. |

### Reply_ArticleMatch

Identity evidence for an article card.

| Field | Type | Meaning |
| --- | --- | --- |
| `kind` | [Reply_ArticleMatchKind](#reply-articlematchkind) | Namespace used to resolve the card. |

### Reply_ArticleMatchKind

Identifier used to resolve this article.

Type: string.

### Reply_ArticleResolution

Result of resolving an article selector.

One of these forms:

- `resolution` is `"resolved"`: A record whose own identity agrees with the input. Fields: `card` ([Reply_Article](#reply-article)), `fetched` (integer or null), `match` ([Reply_ArticleMatch](#reply-articlematch)).
- `resolution` is `"not-found"`: The source confirmed no matching record.

### Reply_ArticleSection

A landed article section.

Type: string.

### Reply_ArticleSectionWord

Article section request including all landed sections.

Type: string.

### Reply_ArticleSections

Landed article sections with independent source states.

| Field | Type | Meaning |
| --- | --- | --- |
| `annotations` | [Reply_Section_Capped_AnnotatedEntity](#reply-section-capped-annotatedentity) or null | Normalized PubTator3 entities. Optional. |
| `citations` | [Reply_Section_Citations](#reply-section-citations) or null | Citation counts from NIH iCite. Optional. |
| `fulltext` | [Reply_Section_FullText](#reply-section-fulltext) or null | Licensed PMC body outline. Optional. |
| `indexing` | [Reply_Section_Indexing](#reply-section-indexing) or null | Index terms from the card's own EFetch citation. Optional. |
| `pdf` | [Reply_Section_PdfLink](#reply-section-pdflink) or null | Publisher PDF link; no file is downloaded. Optional. |
| `supplements` | [Reply_Section_Supplements](#reply-section-supplements) or null | JATS supplementary files confirmed by the bucket record. Optional. |
| `tldr` | [Reply_Section_Tldr](#reply-section-tldr) or null | Machine-written Semantic Scholar summary. Optional. |

### Reply_Capped_AnnotatedEntity

A list cut to a maximum length. `total` counts the items before the cut.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [Reply_AnnotatedEntity](#reply-annotatedentity) | The items, in the source's order. Optional. |
| `total` | integer (uint64) | How many distinct items there were before the cut. |

### Reply_Citations

Independently reported iCite counts; absent lists are never confirmed zeros.

| Field | Type | Meaning |
| --- | --- | --- |
| `cited_by` | integer or null | Number of citing PubMed papers. Optional. |
| `cited_by_clinical` | integer or null | Number of distinct clinical citing papers, when the list was present. Optional. |
| `rcr` | [Reply_FiniteNumber](/reference/schemas/types/#reply-finitenumber) or null | Relative citation ratio, only when confirmed non-provisional. Optional. |
| `references` | integer or null | Number of distinct references, when the list was present. Optional. |

### Reply_Correction

One correction relationship supplied by PubMed.

| Field | Type | Meaning |
| --- | --- | --- |
| `kind` | [Reply_CorrectionKind](#reply-correctionkind) | Correction category. |
| `pmid` | [Reply_Pmid](/reference/schemas/types/#reply-pmid) | Related PMID, when supplied. Optional. |
| `relation` | [Reply_CorrectionRelation](#reply-correctionrelation) | Whether the related record is a notice or original. |

### Reply_CorrectionKind

Source correction relationship kind.

Type: string.

### Reply_CorrectionRelation

Direction of a source correction relationship.

Type: string.

### Reply_FigureCaption

Figure text in document order.

| Field | Type | Meaning |
| --- | --- | --- |
| `caption` | [Reply_Markdown](/reference/schemas/types/#reply-markdown) | Source caption. |
| `label` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Source label. Optional. |

### Reply_FileKind

Supplement format from an explicit JATS MIME declaration, with suffix fallback.

Type: string.

### Reply_FullText

Bucket license and body outline; processing version is not publication chronology.

| Field | Type | Meaning |
| --- | --- | --- |
| `figures` | list of [Reply_FigureCaption](#reply-figurecaption) | Source figure captions in document order. Optional. |
| `license` | [Reply_License](/reference/schemas/types/#reply-license) | Selected bucket license. |
| `manuscript` | boolean | Whether this selected version is an author manuscript. Optional. |
| `parts` | list of [Reply_TextPart](#reply-textpart) | Bounded body parts in reading order. Optional. |
| `processing_version` | integer (uint32) | Highest numeric processing version. Optional. |

### Reply_Indexing

Index terms and author identities supplied by this article's EFetch citation.

| Field | Type | Meaning |
| --- | --- | --- |
| `affiliations` | [Reply_Capped_Text](/reference/schemas/types/#reply-capped-text) | First ten author affiliations and the complete count. |
| `keywords` | list of [Reply_Text](/reference/schemas/types/#reply-text) | Author keywords in source order. Optional. |
| `mesh` | list of [Reply_MeshHeading](#reply-meshheading) | MeSH headings in source order. Optional. |
| `orcids` | list of [Reply_Text](/reference/schemas/types/#reply-text) | Author ORCID identifiers, as supplied by the source. Optional. |

### Reply_MeshHeading

A MeSH descriptor and its qualifiers retain their independent major-topic flags.

| Field | Type | Meaning |
| --- | --- | --- |
| `descriptor` | [Reply_Text](/reference/schemas/types/#reply-text) | Source descriptor. |
| `major` | boolean | Descriptor major-topic flag. |
| `qualifiers` | list of [Reply_MeshQualifier](#reply-meshqualifier) | Source qualifiers. Optional. |

### Reply_MeshQualifier

A qualifier attached to a MeSH descriptor.

| Field | Type | Meaning |
| --- | --- | --- |
| `major` | boolean | Qualifier major-topic flag. |
| `name` | [Reply_Text](/reference/schemas/types/#reply-text) | Source qualifier. |

### Reply_PdfLink

Source-reported PDF link. Reading this section never downloads the file.

| Field | Type | Meaning |
| --- | --- | --- |
| `license` | [Reply_License](/reference/schemas/types/#reply-license) | Normalized license; other grants no reuse rights. |
| `status` | [Reply_OpenAccessStatus](/reference/schemas/types/#reply-openaccessstatus) | Source access status. |
| `url` | [Reply_Text](/reference/schemas/types/#reply-text) | Source publisher URL. |

### Reply_Section_Capped_AnnotatedEntity

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_Capped_AnnotatedEntity](#reply-capped-annotatedentity)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_Citations

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_Citations](#reply-citations)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_FullText

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_FullText](#reply-fulltext)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_Indexing

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_Indexing](#reply-indexing)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_PdfLink

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_PdfLink](#reply-pdflink)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_Supplements

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_Supplements](#reply-supplements)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_Section_Tldr

One section of a record. `state` says whether the source returned data (`data`), answered with nothing (`empty`), failed (`unavailable`), was turned off (`skipped`), or the section does not apply to this record.

One of these forms:

- `state` is `"data"`: The source returned data for the section. Fields: `data` ([Reply_Tldr](#reply-tldr)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"empty"`: The source answered, with no data for this section. Fields: `record` ([Reply_SourceRecord](/reference/schemas/types/#reply-sourcerecord) or null), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"unavailable"`: The source failed to answer. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"skipped"`: Your settings turned the source off, or a key it needs is missing, so BioMCP did not ask it. Fields: `reason` ([Reply_Reason](/reference/schemas/types/#reply-reason)), `source` ([Reply_Source](/reference/schemas/types/#reply-source)).
- `state` is `"not-applicable"`: The section does not apply to this record.

### Reply_SupplementFile

A JATS supplement confirmed by the same processing record.

| Field | Type | Meaning |
| --- | --- | --- |
| `caption` | [Reply_Markdown](/reference/schemas/types/#reply-markdown) or null | JATS caption when present. Optional. |
| `kind` | [Reply_FileKind](#reply-filekind) | Checked format. |
| `label` | [Reply_Text](/reference/schemas/types/#reply-text) or null | JATS label when present. Optional. |
| `name` | [Reply_Text](/reference/schemas/types/#reply-text) | Exact source filename. |

### Reply_Supplements

Available supplements with their owning article license.

| Field | Type | Meaning |
| --- | --- | --- |
| `files` | list of [Reply_SupplementFile](#reply-supplementfile) | Exact JATS/record intersection. Optional. |
| `license` | [Reply_License](/reference/schemas/types/#reply-license) | Article license from the record. |

### Reply_TextPart

One independently readable bounded part.

| Field | Type | Meaning |
| --- | --- | --- |
| `kind` | [Reply_TextPartKind](#reply-textpartkind) | Source structure kind. |
| `number` | integer (uint32) | One-based reader selector. |
| `title` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Source section title or table label. Optional. |
| `tokens` | integer (uint32) | Compact serialized text bytes divided by four, rounded up. |

### Reply_TextPartKind

Body part kind.

Type: string.

### Reply_Tldr

Machine-written source summary with independently reported counts.

| Field | Type | Meaning |
| --- | --- | --- |
| `cited_by` | integer or null | Total source citation count, when supplied. Optional. |
| `influential_cited_by` | integer or null | Source influential citation count, when supplied. Optional. |
| `machine_written` | boolean | Explicitly identifies generated prose. |
| `model` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Source model version. Optional. |
| `text` | [Reply_Text](/reference/schemas/types/#reply-text) or null | Complete one-sentence source summary. Optional. |

</details>

## Next

[`biomcp get article`](/reference/commands/get-article/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
