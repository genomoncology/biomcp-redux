# Drugs@FDA whole read that stops early

Both replies are synthetic; they reproduce a fault, not openFDA data. The request URLs are the ones BioMCP sends for `--applicant EXAMPLE --sort original-approval`, and the reply shape follows the recorded openFDA replies in `../sponsor-original-approval/`. The first read reports 3 matching applications but holds 2; the second read, at skip 2, comes back empty with the same total. The applications, sponsor and product are invented.
