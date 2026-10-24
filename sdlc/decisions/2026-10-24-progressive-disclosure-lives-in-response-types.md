# Progressive disclosure lives in response types

Date: 2026-10-24

Decided by the PM, from Ian's question on how biodata structure fits progressive disclosure.

Biodata entities are complete records with no disclosure logic, so the Genome team and other consumers get the whole entity. BioMCP discloses through four response layers: header, card, section, and pivot or page. Every slice places each fact by one rule set: identity-source facts the common question needs go on the card, facts from another source or request go in a section, growable lists are capped with their total and a next step, lists of other entities become pivots, and entities reference each other only by ID and label. `Option` means the source has no value, never "not requested". No request-shaped field selection. `planning/architecture.md` section 3.10 holds the rules and their checks.
