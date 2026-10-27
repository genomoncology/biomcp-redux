# Screen registry records in explicit stages

## Question

Which registry trials match the stated KIT L576P and gastrointestinal stromal tumor query, and what changes when the exact variant filter is removed?

## 1. Retrieve and inspect

```sh
biomcp search trial --gene KIT --variant L576P --condition 'gastrointestinal stromal tumor' --limit 100
```

Read each registry identifier, recruitment status, condition text and source problems. Follow page commands to inspect the remaining results.

## 2. Retrieve and inspect

```sh
biomcp search trial --gene KIT --condition 'gastrointestinal stromal tumor' --limit 100
```

Compare this broader query with the preceding exact-variant search. It removes the variant filter and retains the gene and condition. Keep both result sets and their explicit filters in the search record.

## Next

Inspect full registry eligibility text and any continued criteria. Run broader gene and program searches suggested by returned commands before calling a screen complete. A complete response does not mean a complete screen; the clinical team must assess fit and confirm current recruitment. Contact the study team to confirm eligibility and current recruitment.

Read response status, problems and source terms at every step. Recorded replies demonstrate the commands; live replies can change.
