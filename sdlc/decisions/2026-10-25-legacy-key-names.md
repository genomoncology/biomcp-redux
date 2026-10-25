# Keys keep their 0.9 environment names

Date: 2026-10-25. Decided by Ian.

BioMCP 1.0 reads each key from the same environment variable 0.9 used. Users who upgrade keep their settings and never rename a variable. Ian never asked for a rename. The PM renamed two variables on 2026-10-23 and 2026-10-24 only to match the new config field names, and that reason does not justify breaking existing settings. 1.0 has not shipped, so each key keeps one name and no fallback is needed.

| Key | Environment variable | Config field |
|---|---|---|
| Semantic Scholar | `S2_API_KEY` | `keys.semantic_scholar_api_key` |
| OncoKB | `ONCOKB_TOKEN` | `keys.oncokb_api_key` |
| NCBI | `NCBI_API_KEY` | `keys.ncbi_api_key` |
| NCI | `NCI_API_KEY` | `keys.nci_api_key` |

Keys added later keep their 0.9 names: `OPENFDA_API_KEY`, `UMLS_API_KEY`, `ORCID_ACCESS_TOKEN` and `ALPHAGENOME_API_KEY`. Precedence stays the config file, then the environment variable. `biomcp help key <source>` names the config field and the variable.
