# Search registry trials

Inspect registry status and eligibility text. A returned listing is not an enrollment recommendation.

```sh
biomcp search trial --condition melanoma
```

Read response problems and source terms before using the record. Follow its returned next commands for additional pages or sections.

An exact variant search is a starting point. Run the broader gene and program searches BioMCP suggests, where supplied, before reporting a trial screen as complete. For example, an exact variant search can return a next command that removes the variant and keeps the gene and condition:

```sh
biomcp search trial --gene KIT --variant L576P --condition 'gastrointestinal stromal tumor' --limit 100
biomcp search trial --gene KIT --condition 'gastrointestinal stromal tumor' --limit 100
```

Follow returned pages and eligibility-text searches. Each explicit query searches its stated registry fields. Record which queries you ran and any source problems or unfinished follow-ups. A response status of complete describes that response; it does not mean the trial screen is complete. Registry text and broader results do not establish which trials fit.
