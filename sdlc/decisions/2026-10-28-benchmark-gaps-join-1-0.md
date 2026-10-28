# The public benchmark gaps join 1.0

Date: 2026-10-28

Decided by Ian ("let's go ahead and get those tickets prepped and in there").

The 0405 replay of MCP-Bench's BioMCP tasks found four capabilities that agents trained on biomcp-python 0.4.6 use and 1.0 lacks. All four ship in 1.0, as an explicit expansion of the rule that new capabilities wait:

- NCI organization search, and NCI trials at a chosen organization (0408, 0409).
- NCI biomarker search, and NCI trials that include or exclude a biomarker by exact NCIt code (0410, 0411).
- FDA device recalls, and MAUDE device reports by manufacturer and product code (0412, 0413). This replaces the 0162 ruling that dropped FDA recalls.
- Several trial conditions or interventions joined with OR in one search (0414). This replaces the after-1.0 line recorded on 2026-10-28.

The PM checked each source on 2026-10-28 with bounded calls: NCI `/organizations` and `/biomarkers` answer with the existing NCI key, and openFDA `device/recall.json` and `device/enforcement.json` answer without one. None needs a dataset download. Ian asked for an adversarial review of these designs before building.
