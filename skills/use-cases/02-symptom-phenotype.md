# Inspect a phenotype term

## Question

How do I inspect an ontology root identifier without treating it as a symptom?

## 1. Retrieve and inspect

```sh
biomcp get phenotype HP:0000001
```

HP:0000001 is the HPO root ‘All’, not a symptom. Read the term identifier, label, definition and source. This executable fixture uses an authored synthetic definition; its text is a command demonstration. These root-only reads do not demonstrate symptom search or patient phenotype selection.

## 2. Retrieve and inspect

```sh
biomcp get phenotype HP:0000001 --source monarch
```

Read the explicit source and compare the identity with the preceding reply. Do not replace a patient description with a synthetic fixture label.

## Next

For a real symptom question, search the person’s stated term, inspect all candidate definitions and retain the selected source identifier. A term record supplies no diagnosis.

Read response status, problems and source terms at every step. Recorded replies demonstrate the commands; live replies can change.
