# Drugs@FDA whole read that stops early

All three replies are synthetic; they reproduce a fault, not openFDA data. The applications (NDA900001 to NDA900500), sponsor and dates are invented. The request URLs are the ones BioMCP sends for `--applicant EXAMPLE` when it orders or windows by original approval, and the reply shapes follow recorded openFDA replies: the 200 pages in `../sponsor-original-approval/` and the 404 "No matches found!" that openFDA returns for a skip past the end of a match set (checked live on 2026-11-13 with `sponsor_name:SANDOZ*`, skip 515).

The fault is a match set that shrinks between reads, as when openFDA updates the dataset during a search. The first read reports 501 matches and holds 500. The second read, at skip 500, answers 404, and BioMCP's count probe (limit 1, no skip) then reports 500 matches, so the read past them is an honest empty page. BioMCP has read 500 of the 501 it first counted.
