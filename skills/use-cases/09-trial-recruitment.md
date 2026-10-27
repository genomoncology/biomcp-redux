# Screen registry records in explicit stages

## Question

Which registry trials match a stated condition, and what does a narrower query change?

## 1. Retrieve and inspect

```sh
biomcp search trial --condition melanoma
```

Read each registry identifier, recruitment status, condition text and source problems. Follow page commands to inspect the remaining results.

## 2. Retrieve and inspect

```sh
biomcp search trial --condition melanoma --intervention pembrolizumab
```

Compare the explicit query with the preceding condition search. Keep the narrower result set and its filters in the search record.

## 3. Retrieve and inspect

```sh
biomcp get trial NCT04303780
```

This separate recorded trial illustrates an exact registry read. Read its dates, status, locations and available eligibility sections before following returned commands.

## Next

Inspect full registry eligibility text and any continued criteria. Run broader gene and program searches suggested by returned commands before calling a screen complete. A complete response does not mean a complete screen; the clinical team must assess fit and confirm current recruitment.

Read response status, problems and source terms at every step. Recorded replies demonstrate the commands; live replies can change.
