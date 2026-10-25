# 0.9 key environment names stay supported

Date: 2026-10-25. Decided by Ian.

BioMCP 1.0 reads both its own environment variable and the 0.9 name for every key. Users who upgrade keep their existing settings and never rename a variable.

| Key | 1.0 name | 0.9 name |
|---|---|---|
| Semantic Scholar | `SEMANTIC_SCHOLAR_API_KEY` | `S2_API_KEY` |
| OncoKB | `ONCOKB_API_KEY` | `ONCOKB_TOKEN` |
| NCBI | `NCBI_API_KEY` | same |
| NCI | `NCI_API_KEY` | same |

Precedence: the config file, then the 1.0 name, then the 0.9 name. When both variables are set, the 1.0 name wins and BioMCP says nothing about it. Keys added later, such as openFDA, UMLS, ORCID and AlphaGenome, take their 0.9 name (`OPENFDA_API_KEY`, `UMLS_API_KEY`, `ORCID_ACCESS_TOKEN`, `ALPHAGENOME_API_KEY`) as their 1.0 name, or read it as the fallback when 1.0 picks a different name.

`biomcp help key <source>` names both variables. The upgrade guide says the old names keep working and does not tell users to rename them.
