# Keyed sources run only with the user's key

Date: 2026-10-24

Decided by Ian.

A source whose terms require the user's own license or account, such as OncoKB, is supported. It runs only when the user supplies its API key in configuration. Without the key, BioMCP never calls the source, including its public endpoints, and the output names the source as skipped because no key is configured, with a hint on how to add one. OncoKB is the most important of these sources. Each slice names its keyed sources.
