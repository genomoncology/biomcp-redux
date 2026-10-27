---
title: Reply of biomcp get article (biomcp.article_cards/3)
description: "The JSON reply BioMCP 1.0.0 prints for biomcp get article: a real
  example and the fields of schema biomcp.article_cards/3."
goal: "Parse the biomcp.article_cards/3 reply: see a real example and what each
  field means."
reader: A developer or agent parsing BioMCP JSON
sidebar:
  label: biomcp.article_cards/3
kind: reference
verified: 2026-10-27
---

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get article`](/reference/commands/get-article/) and [`biomcp get article indexing`](/reference/commands/get-article/) reply with `schema` set to `biomcp.article_cards/3`. Read this page when your code parses that reply: it shows a real one, then the fields only this schema has. [Shared types](/reference/schemas/types/) defines the rest.

## Commands that return it

These commands returned this reply:

```sh
biomcp get article 33123537
biomcp get article 33123537 indexing
```

`biomcp schema biomcp.article_cards/3` prints the full JSON Schema.

## Example reply

`biomcp get article 33123537` returned this on 2026-10-27. Lists show their first items; a line with … counts the rest, and text ending in … is cut.

```json
{
  "schema": "biomcp.article_cards/3",
  "status": "complete",
  "query": {"input": ["33123537"]},
  "result": {
    "items": [
      {
        "input": "33123537",
        "status": "complete",
        "answer": {…},
        "more": [
          "indexing", "annotations", "fulltext", "supplements", "citations",
          "tldr", "pdf"
        ]
      }
    ]
  },
  "sources": [
    {
      "source": "pubmed",
      "use": "restricted",
      "terms": "NCBI policies; publisher rights in abstracts",
      "terms_url": "https://www.ncbi.nlm.nih.gov/home/about/policies/",
      "state": "ok"
    }
  ],
  "next": [
    {
      "kind": "section",
      "args": [
        "get", "article", "33123537", "indexing", "annotations", "fulltext",
        "supplements", "citations",
        … 2 more
      ]
    },
    … 3 more
  ]
}
```

## Fields

A field marked Optional can be missing from a reply. Every other field is always there.

`result` holds [ArticleCards](#articlecards). `query` holds [ArticleGet](#articleget). `query` repeats the request as BioMCP read it.

### ArticleCards

Cards in caller order.

| Field | Type | Meaning |
| --- | --- | --- |
| `items` | list of [ArticleCardItem](#articlecarditem) | Per-input answers. |

### ArticleGet

Get one to twenty article inputs in caller order.

| Field | Type | Meaning |
| --- | --- | --- |
| `input` | list of [ArticleInput](/reference/schemas/types/#articleinput) | PubMed, PMC or DOI identifiers in input order. |
| `section` | list of [ArticleSectionWord](#articlesectionword) | Requested [section words](/concepts/progressive-disclosure/); trailing words and --section share this field. Optional. |

### AbstractPart

One source-labelled abstract paragraph.

| Field | Type | Meaning |
| --- | --- | --- |
| `label` | [Text](/reference/schemas/types/#text) or null | Source label, when supplied. Optional. |
| `text` | [Markdown](/reference/schemas/types/#markdown) | Whole paragraph text. |

### AnnotatedEntity

One normalized [entity](/concepts/progressive-disclosure/) with its mention count.

| Field | Type | Meaning |
| --- | --- | --- |
| `count` | integer (uint64) | Number of annotation mentions. |
| `entrez_id` | [NcbiGeneId](/reference/schemas/types/#ncbigeneid) or null | Gene identity, including a variant's associated gene. Optional. |
| `hgvs` | [Text](/reference/schemas/types/#text) or null | Source HGVS expression for a variant. Optional. |
| `kind` | [AnnotationKind](#annotationkind) | Source concept type. |
| `mesh` | [Text](/reference/schemas/types/#text) or null | Source MeSH identity for a disease or chemical. Optional. |
| `name` | [Text](/reference/schemas/types/#text) | Source normalized name. |
| `rsid` | [Text](/reference/schemas/types/#text) or null | Source reference SNP identity for a variant. Optional. |

### AnnotationKind

Normalized annotation category in deterministic tie order.

| Value | Meaning |
| --- | --- |
| `gene` | Gene mention. |
| `variant` | Genetic variant mention. |
| `disease` | Disease mention. |
| `chemical` | Chemical mention. |

### Article

Article facts with at least one checked identity.

| Field | Type | Meaning |
| --- | --- | --- |
| `abstract_parts` | list of [AbstractPart](#abstractpart) | Whole abstract, preserving labels and source order. |
| `authors` | [Capped](/reference/schemas/types/#capped) of [Text](/reference/schemas/types/#text) | Authors in source order. |
| `corrections` | list of [Correction](#correction) | Related correction notices or originals. |
| `date` | [PartialDate](/reference/schemas/types/#partialdate) or null | Publication date at source precision. Optional. |
| `doi` | [Doi](/reference/schemas/types/#doi) | DOI assigned to this article. Optional. |
| `electronic_date` | [PartialDate](/reference/schemas/types/#partialdate) or null | Electronic publication date when different from the publication date. Optional. |
| `europepmc_id` | [EuropePmcId](/reference/schemas/types/#europepmcid) | Europe PMC identity when this card comes from its core record. Optional. |
| `journal` | [Text](/reference/schemas/types/#text) or null | Journal name. Optional. |
| `pmcid` | [Pmcid](/reference/schemas/types/#pmcid) | PMC identity assigned to this article. Optional. |
| `pmid` | [Pmid](/reference/schemas/types/#pmid) | PubMed identity. Optional. |
| `title` | [Text](/reference/schemas/types/#text) | Whole title with inline text preserved. |
| `types` | list of [PublicationType](/reference/schemas/types/#publicationtype) | Every clinically meaningful source publication type. |

### ArticleCardItem

Answer to one input; shared DOIs may expand to several consecutive items.

| Field | Type | Meaning |
| --- | --- | --- |
| `answer` | [ArticleResolution](#articleresolution) or null | Absent only when a provider failed. Optional. |
| `input` | [ArticleInput](/reference/schemas/types/#articleinput) | Original selector. |
| `more` | list of [ArticleSection](#articlesection) | Applicable sections omitted by this request. Optional. |
| `sections` | [ArticleSections](#articlesections) or null | Requested section states for a resolved card. Optional. |
| `status` | [Status](/reference/schemas/types/#status) | This card's outcome. |

### ArticleMatch

Identity evidence for an article card.

| Field | Type | Meaning |
| --- | --- | --- |
| `kind` | [ArticleMatchKind](#articlematchkind) | Namespace used to resolve the card. |

### ArticleMatchKind

Identifier used to resolve this article.

| Value | Meaning |
| --- | --- |
| `pmid` | PubMed identifier. |
| `pmcid` | PMC identifier. |
| `doi` | Digital object identifier. |
| `europepmc_id` | Europe PMC preprint identifier. |

### ArticleResolution

Result of resolving an article selector.

One of these forms:

- `resolution` is `"resolved"`: A record whose own identity agrees with the input. Fields: `card` ([Article](#article)), `fetched` (integer (uint64)), `match` ([ArticleMatch](#articlematch)).
- `resolution` is `"not_found"`: The source confirmed no matching record.

### ArticleSection

A landed article section.

| Value | Meaning |
| --- | --- |
| `fulltext` | Licensed PMC body outline. |
| `supplements` | Supplementary files confirmed by PMC metadata. |
| `annotations` | Normalized text-mined entities from PubTator3. |
| `tldr` | Machine-written Semantic Scholar summary and counts. |
| `citations` | Citation counts from NIH iCite. |
| `indexing` | MeSH, keywords and author indexing from the citation. |
| `pdf` | Publisher PDF link without downloading. |

### ArticleSectionWord

Article section request including all landed sections.

| Value | Meaning |
| --- | --- |
| `annotations` | Normalized text-mined entities from PubTator3. |
| `fulltext` | Licensed PMC body outline. |
| `supplements` | Supplementary files confirmed by PMC metadata. |
| `tldr` | Machine-written Semantic Scholar summary and counts. |
| `citations` | Citation counts from NIH iCite. |
| `indexing` | MeSH, keywords and author indexing from the citation. |
| `pdf` | Publisher PDF link without downloading. |
| `all` | Every landed article section. |

### ArticleSections

Landed article sections with independent source states.

| Field | Type | Meaning |
| --- | --- | --- |
| `annotations` | [Section](/reference/schemas/types/#section) of [Capped](/reference/schemas/types/#capped) of [AnnotatedEntity](#annotatedentity) or null | Normalized PubTator3 entities. Optional. |
| `citations` | [Section](/reference/schemas/types/#section) of [Citations](#citations) or null | Citation counts from NIH iCite. Optional. |
| `fulltext` | [Section](/reference/schemas/types/#section) of [FullText](#fulltext) or null | Licensed PMC body outline. Optional. |
| `indexing` | [Section](/reference/schemas/types/#section) of [Indexing](#indexing) or null | Index terms from the card's own EFetch citation. Optional. |
| `pdf` | [Section](/reference/schemas/types/#section) of [PdfLink](#pdflink) or null | Publisher PDF link; no file is downloaded. Optional. |
| `supplements` | [Section](/reference/schemas/types/#section) of [Supplements](#supplements) or null | JATS supplementary files confirmed by the bucket record. Optional. |
| `tldr` | [Section](/reference/schemas/types/#section) of [Tldr](#tldr) or null | Machine-written Semantic Scholar summary. Optional. |

### Citations

Independently reported iCite counts; absent lists are never confirmed zeros.

| Field | Type | Meaning |
| --- | --- | --- |
| `cited_by` | integer or null | Number of citing PubMed papers. Optional. |
| `cited_by_clinical` | integer or null | Number of distinct clinical citing papers, when the list was present. Optional. |
| `rcr` | [FiniteNumber](/reference/schemas/types/#finitenumber) or null | Relative citation ratio, only when confirmed non-provisional. Optional. |
| `references` | integer or null | Number of distinct references, when the list was present. Optional. |

### Correction

One correction relationship supplied by PubMed.

| Field | Type | Meaning |
| --- | --- | --- |
| `kind` | [CorrectionKind](#correctionkind) | Correction category. |
| `pmid` | [Pmid](/reference/schemas/types/#pmid) | Related PMID, when supplied. Optional. |
| `relation` | [CorrectionRelation](#correctionrelation) | Whether the related record is a notice or original. |

### CorrectionKind

Source correction relationship kind.

| Value | Meaning |
| --- | --- |
| `retraction` | Retraction relationship. |
| `erratum` | Erratum relationship. |
| `expression_of_concern` | Expression of concern relationship. |

### CorrectionRelation

Direction of a source correction relationship.

| Value | Meaning |
| --- | --- |
| `notice` | The related record is the notice. |
| `original` | The related record is the original. |

### FigureCaption

Figure text in document order.

| Field | Type | Meaning |
| --- | --- | --- |
| `caption` | [Markdown](/reference/schemas/types/#markdown) | Source caption. |
| `label` | [Text](/reference/schemas/types/#text) | Source label. |

### FileKind

Supplement format from an explicit JATS MIME declaration, with suffix fallback.

| Value | Meaning |
| --- | --- |
| `xlsx` | Excel workbook. |
| `csv` | Comma-separated text. |
| `tsv` | Tab-separated text. |
| `txt` | Plain text lines. |
| `pdf` | PDF download offered without parsing. |
| `other` | Other source format. |

### FullText

Bucket license and body outline; processing version is not publication chronology.

| Field | Type | Meaning |
| --- | --- | --- |
| `figures` | list of [FigureCaption](#figurecaption) | Source figure captions in document order. |
| `license` | [License](/reference/schemas/types/#license) | Selected bucket license. |
| `manuscript` | boolean | Whether this selected version is an author manuscript. Optional. |
| `parts` | list of [TextPart](#textpart) | Bounded body parts in reading order. |
| `processing_version` | integer (uint32) | Highest numeric processing version. Optional. |

### Indexing

Index terms and author identities supplied by this article's EFetch citation.

| Field | Type | Meaning |
| --- | --- | --- |
| `affiliations` | [Capped](/reference/schemas/types/#capped) of [Text](/reference/schemas/types/#text) | First ten author affiliations and the complete count. |
| `keywords` | list of [Text](/reference/schemas/types/#text) | Author keywords in source order. |
| `mesh` | list of [MeshHeading](#meshheading) | MeSH headings in source order. |
| `orcids` | list of [Text](/reference/schemas/types/#text) | Author ORCID identifiers, as supplied by the source. |

### MeshHeading

A MeSH descriptor and its qualifiers retain their independent major-topic flags.

| Field | Type | Meaning |
| --- | --- | --- |
| `descriptor` | [Text](/reference/schemas/types/#text) | Source descriptor. |
| `major` | boolean | Descriptor major-topic flag. |
| `qualifiers` | list of [MeshQualifier](#meshqualifier) | Source qualifiers. |

### MeshQualifier

A qualifier attached to a MeSH descriptor.

| Field | Type | Meaning |
| --- | --- | --- |
| `major` | boolean | Qualifier major-topic flag. |
| `name` | [Text](/reference/schemas/types/#text) | Source qualifier. |

### PdfLink

Source-reported PDF link. Reading this section never downloads the file.

| Field | Type | Meaning |
| --- | --- | --- |
| `license` | [License](/reference/schemas/types/#license) | Normalized license; other grants no reuse rights. |
| `status` | [OpenAccessStatus](/reference/schemas/types/#openaccessstatus) | Source access status. |
| `url` | [Text](/reference/schemas/types/#text) | Source publisher URL. |

### SupplementFile

A JATS supplement confirmed by the same processing record.

| Field | Type | Meaning |
| --- | --- | --- |
| `caption` | [Markdown](/reference/schemas/types/#markdown) or null | JATS caption when present. Optional. |
| `kind` | [FileKind](#filekind) | Checked format. |
| `label` | [Text](/reference/schemas/types/#text) or null | JATS label when present. Optional. |
| `name` | [Text](/reference/schemas/types/#text) | Exact source filename. |

### Supplements

Available supplements with their owning article license.

| Field | Type | Meaning |
| --- | --- | --- |
| `files` | list of [SupplementFile](#supplementfile) | Exact JATS/record intersection. |
| `license` | [License](/reference/schemas/types/#license) | Article license from the record. |

### TextPart

One independently readable bounded part.

| Field | Type | Meaning |
| --- | --- | --- |
| `kind` | [TextPartKind](#textpartkind) | Source structure kind. |
| `number` | integer (uint32) | One-based reader selector. |
| `title` | [Text](/reference/schemas/types/#text) | Source section title or table label. |
| `tokens` | integer (uint32) | Compact serialized text bytes divided by four, rounded up. |

### TextPartKind

Body part kind.

| Value | Meaning |
| --- | --- |
| `section` | Top-level body section. |
| `table` | Table, including caption and rows. |

### Tldr

Machine-written source summary with independently reported counts.

| Field | Type | Meaning |
| --- | --- | --- |
| `cited_by` | integer or null | Total source citation count, when supplied. Optional. |
| `influential_cited_by` | integer or null | Source influential citation count, when supplied. Optional. |
| `machine_written` | boolean | Explicitly identifies generated prose. |
| `model` | [Text](/reference/schemas/types/#text) | Source model version. |
| `text` | [Text](/reference/schemas/types/#text) | Complete one-sentence source summary. |

## Next

[`biomcp get article`](/reference/commands/get-article/) gives the command's arguments; [reply schemas](/reference/schemas/) lists every reply.
