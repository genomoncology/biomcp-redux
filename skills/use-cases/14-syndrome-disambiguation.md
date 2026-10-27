# Resolve an ambiguous gene name before disease review

## Question

How do I avoid silently selecting one gene from an ambiguous alias?

## 1. Retrieve and inspect

```sh
biomcp get gene p16
```

Read the ambiguous resolution and all candidate HGNC identifiers. Preserve the source match kind and candidate names.

## 2. Retrieve and inspect

```sh
biomcp get gene CDKN2A
```

This example explicitly selects one candidate. Check its resolved symbol and identifier before reading disease-related source sections.

## Next

Choose a candidate only from the question’s stated identity or independently reviewed evidence. A gene alias and its candidate list cannot disambiguate a syndrome or diagnose a patient.

Read response status, problems and source terms at every step. Recorded replies demonstrate the commands; live replies can change.
