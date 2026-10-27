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

BioMCP 1.0.0 prints one JSON object, the reply, for each command. [`biomcp get article`](/reference/commands/get-article/) and [`biomcp get article indexing`](/reference/commands/get-article/) reply with `schema` set to `biomcp.article_cards/3`. Read this page when your code parses that reply. It shows a real reply, then the fields that only this schema has. The fields every reply shares are on [reply envelope and shared types](/reference/schemas/types/).

## Commands that return it

These commands returned this reply:

```sh
biomcp get article 33123537
biomcp get article 33123537 indexing
```

`biomcp schema biomcp.article_cards/3` prints the full JSON Schema.

## Example reply

`biomcp get article 33123537` returned this reply on 2026-10-27. Lists show their first items, and a line that starts with … counts the items left out. Text that ends in … is cut.

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
        "answer": {
          "resolution": "resolved",
          "card": {
            "pmid": "33123537",
            "doi": "10.3389/fcell.2020.573599",
            "pmcid": "PMC7573235",
            "title": "Effect of AKT1 (p. E17K) Hotspot Mutation on Malignant…",
            "journal": "Frontiers in cell and developmental biology",
            "date": "2020",
            "electronic_date": "2020-10-06",
            "types": ["systematic_review"],
            "corrections": [],
            "authors": {…},
            "abstract_parts": […]
          },
          "match": {"kind": "pmid"},
          "fetched": 1793140290
        },
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

`result` holds [ArticleCards](#articlecards). `query` holds [ArticleGet](#articleget). `query` repeats the request as BioMCP read it.

### ArticleCards

Cards in caller order.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `items` | list of [ArticleCardItem](#articlecarditem) | yes | Per-input answers. |

### ArticleGet

Get one to twenty article inputs in caller order.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `input` | list of [ArticleInput](/reference/schemas/types/#articleinput) | yes | PubMed, PMC or DOI identifiers in input order. |
| `section` | list of [ArticleSectionWord](#articlesectionword) | no | Requested [section words](/concepts/progressive-disclosure/); trailing words and --section share this field. |

### AbstractPart

One source-labelled abstract paragraph.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `label` | [Text](/reference/schemas/types/#text) or null | no | Source label, when supplied. |
| `text` | [Markdown](/reference/schemas/types/#markdown) | yes | Whole paragraph text. |

### AnnotatedEntity

One normalized [entity](/concepts/progressive-disclosure/) with its mention count.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `count` | integer (uint64) | yes | Number of annotation mentions. |
| `entrez_id` | [NcbiGeneId](/reference/schemas/types/#ncbigeneid) or null | no | Gene identity, including a variant's associated gene. |
| `hgvs` | [Text](/reference/schemas/types/#text) or null | no | Source HGVS expression for a variant. |
| `kind` | [AnnotationKind](#annotationkind) | yes | Source concept type. |
| `mesh` | [Text](/reference/schemas/types/#text) or null | no | Source MeSH identity for a disease or chemical. |
| `name` | [Text](/reference/schemas/types/#text) | yes | Source normalized name. |
| `rsid` | [Text](/reference/schemas/types/#text) or null | no | Source reference SNP identity for a variant. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `abstract_parts` | list of [AbstractPart](#abstractpart) | yes | Whole abstract, preserving labels and source order. |
| `authors` | [Capped](/reference/schemas/types/#capped) of [Text](/reference/schemas/types/#text) | yes | Authors in source order. |
| `corrections` | list of [Correction](#correction) | yes | Related correction notices or originals. |
| `date` | [PartialDate](/reference/schemas/types/#partialdate) or null | no | Publication date at source precision. |
| `doi` | [Doi](/reference/schemas/types/#doi) | no | DOI assigned to this article. |
| `electronic_date` | [PartialDate](/reference/schemas/types/#partialdate) or null | no | Electronic publication date when different from the publication date. |
| `europepmc_id` | [EuropePmcId](/reference/schemas/types/#europepmcid) | no | Europe PMC identity when this card comes from its core record. |
| `journal` | [Text](/reference/schemas/types/#text) or null | no | Journal name. |
| `pmcid` | [Pmcid](/reference/schemas/types/#pmcid) | no | PMC identity assigned to this article. |
| `pmid` | [Pmid](/reference/schemas/types/#pmid) | no | PubMed identity. |
| `title` | [Text](/reference/schemas/types/#text) | yes | Whole title with inline text preserved. |
| `types` | list of [PublicationType](/reference/schemas/types/#publicationtype) | yes | Every clinically meaningful source publication type. |

### ArticleCardItem

Answer to one input; shared DOIs may expand to several consecutive items.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `answer` | [ArticleResolution](#articleresolution) or null | no | Absent only when a provider failed. |
| `input` | [ArticleInput](/reference/schemas/types/#articleinput) | yes | Original selector. |
| `more` | list of [ArticleSection](#articlesection) | no | Applicable sections omitted by this request. |
| `sections` | [ArticleSections](#articlesections) or null | no | Requested section states for a resolved card. |
| `status` | [Status](/reference/schemas/types/#status) | yes | This card's outcome. |

### ArticleMatch

Identity evidence for an article card.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `kind` | [ArticleMatchKind](#articlematchkind) | yes | Namespace used to resolve the card. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `annotations` | [Section](/reference/schemas/types/#section) of [Capped](/reference/schemas/types/#capped) of [AnnotatedEntity](#annotatedentity) or null | no | Normalized PubTator3 entities. |
| `citations` | [Section](/reference/schemas/types/#section) of [Citations](#citations) or null | no | Citation counts from NIH iCite. |
| `fulltext` | [Section](/reference/schemas/types/#section) of [FullText](#fulltext) or null | no | Licensed PMC body outline. |
| `indexing` | [Section](/reference/schemas/types/#section) of [Indexing](#indexing) or null | no | Index terms from the card's own EFetch citation. |
| `pdf` | [Section](/reference/schemas/types/#section) of [PdfLink](#pdflink) or null | no | Publisher PDF link; no file is downloaded. |
| `supplements` | [Section](/reference/schemas/types/#section) of [Supplements](#supplements) or null | no | JATS supplementary files confirmed by the bucket record. |
| `tldr` | [Section](/reference/schemas/types/#section) of [Tldr](#tldr) or null | no | Machine-written Semantic Scholar summary. |

### Citations

Independently reported iCite counts; absent lists are never confirmed zeros.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `cited_by` | integer or null | no | Number of citing PubMed papers. |
| `cited_by_clinical` | integer or null | no | Number of distinct clinical citing papers, when the list was present. |
| `rcr` | [FiniteNumber](/reference/schemas/types/#finitenumber) or null | no | Relative citation ratio, only when confirmed non-provisional. |
| `references` | integer or null | no | Number of distinct references, when the list was present. |

### Correction

One correction relationship supplied by PubMed.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `kind` | [CorrectionKind](#correctionkind) | yes | Correction category. |
| `pmid` | [Pmid](/reference/schemas/types/#pmid) | no | Related PMID, when supplied. |
| `relation` | [CorrectionRelation](#correctionrelation) | yes | Whether the related record is a notice or original. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `caption` | [Markdown](/reference/schemas/types/#markdown) | yes | Source caption. |
| `label` | [Text](/reference/schemas/types/#text) | yes | Source label. |

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

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `figures` | list of [FigureCaption](#figurecaption) | yes | Source figure captions in document order. |
| `license` | [License](/reference/schemas/types/#license) | yes | Selected bucket license. |
| `manuscript` | boolean | no | Whether this selected version is an author manuscript. |
| `parts` | list of [TextPart](#textpart) | yes | Bounded body parts in reading order. |
| `processing_version` | integer (uint32) | no | Highest numeric processing version. |

### Indexing

Index terms and author identities supplied by this article's EFetch citation.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `affiliations` | [Capped](/reference/schemas/types/#capped) of [Text](/reference/schemas/types/#text) | yes | First ten author affiliations and the complete count. |
| `keywords` | list of [Text](/reference/schemas/types/#text) | yes | Author keywords in source order. |
| `mesh` | list of [MeshHeading](#meshheading) | yes | MeSH headings in source order. |
| `orcids` | list of [Text](/reference/schemas/types/#text) | yes | Author ORCID identifiers, as supplied by the source. |

### MeshHeading

A MeSH descriptor and its qualifiers retain their independent major-topic flags.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `descriptor` | [Text](/reference/schemas/types/#text) | yes | Source descriptor. |
| `major` | boolean | yes | Descriptor major-topic flag. |
| `qualifiers` | list of [MeshQualifier](#meshqualifier) | yes | Source qualifiers. |

### MeshQualifier

A qualifier attached to a MeSH descriptor.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `major` | boolean | yes | Qualifier major-topic flag. |
| `name` | [Text](/reference/schemas/types/#text) | yes | Source qualifier. |

### PdfLink

Source-reported PDF link. Reading this section never downloads the file.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `license` | [License](/reference/schemas/types/#license) | yes | Normalized license; other grants no reuse rights. |
| `status` | [OpenAccessStatus](/reference/schemas/types/#openaccessstatus) | yes | Source access status. |
| `url` | [Text](/reference/schemas/types/#text) | yes | Source publisher URL. |

### SupplementFile

A JATS supplement confirmed by the same processing record.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `caption` | [Markdown](/reference/schemas/types/#markdown) or null | no | JATS caption when present. |
| `kind` | [FileKind](#filekind) | yes | Checked format. |
| `label` | [Text](/reference/schemas/types/#text) or null | no | JATS label when present. |
| `name` | [Text](/reference/schemas/types/#text) | yes | Exact source filename. |

### Supplements

Available supplements with their owning article license.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `files` | list of [SupplementFile](#supplementfile) | yes | Exact JATS/record intersection. |
| `license` | [License](/reference/schemas/types/#license) | yes | Article license from the record. |

### TextPart

One independently readable bounded part.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `kind` | [TextPartKind](#textpartkind) | yes | Source structure kind. |
| `number` | integer (uint32) | yes | One-based reader selector. |
| `title` | [Text](/reference/schemas/types/#text) | yes | Source section title or table label. |
| `tokens` | integer (uint32) | yes | Compact serialized text bytes divided by four, rounded up. |

### TextPartKind

Body part kind.

| Value | Meaning |
| --- | --- |
| `section` | Top-level body section. |
| `table` | Table, including caption and rows. |

### Tldr

Machine-written source summary with independently reported counts.

| Field | Type | Always present | Meaning |
| --- | --- | --- | --- |
| `cited_by` | integer or null | no | Total source citation count, when supplied. |
| `influential_cited_by` | integer or null | no | Source influential citation count, when supplied. |
| `machine_written` | boolean | yes | Explicitly identifies generated prose. |
| `model` | [Text](/reference/schemas/types/#text) | yes | Source model version. |
| `text` | [Text](/reference/schemas/types/#text) | yes | Complete one-sentence source summary. |

## Next

[Reply envelope and shared types](/reference/schemas/types/) defines `status`, `sources`, `next` and every shared type. [Reply schemas](/reference/schemas/) lists every reply. [`biomcp get article`](/reference/commands/get-article/) gives the command's arguments.
