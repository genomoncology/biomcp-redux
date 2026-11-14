# Drugs@FDA sponsor search by original approval

The two replies were recorded unchanged from the public openFDA Drugs@FDA endpoint on 2026-11-13 by `biomcp --record` in pm job j39749, without a key. The publisher's dataset date was 2026-11-13. The recording wrappers hold each body as parsed JSON. The same requests, fetched again the same day with curl, returned bodies whose parsed JSON equals the recorded values exactly; the byte counts and SHA-256 below are of those original bodies.

| Request | Status | Bytes | SHA-256 of the original body |
| --- | --- | --- | --- |
| `https://api.fda.gov/drug/drugsfda.json?search=sponsor_name%3ANOVARTIS%2A&limit=500&sort=application_number%3Aasc` | 200 | 2,120,486 | `d385af390cfbe04157216bdbe107221d43e78c1c8c2d670710556ef2cb54bab3` |
| `https://api.fda.gov/drug/drugsfda.json?search=sponsor_name%3AA%2A&limit=500&sort=application_number%3Aasc` | 200 | 1,440,215 | `802c92c3dcc36ea792ee8e6c1cab618f733c234cd65f1fc8671bd14b1c6fd084` |

The first body holds all 204 applications whose sponsor name starts with NOVARTIS; 180 list an approved original (ORIG, status AP) submission. Computed from the body alone, newest first: NDA218436 2025-09-30, NDA219208 2025-04-02, NDA218466 2024-04-24, NDA218591 2024-04-12, then NDA218276 2023-12-05; those first four are the applications first approved on or after 2024-01-01. NDA217513 and NDA217514 share 2023-03-16 and NDA215833 and NDA215841 share 2022-03-23; application number breaks both ties. The second body is the first read for the prefix A, which matches 4,953 applications, more than the 3,000 BioMCP reads. The prefix `NOVARTIS` matches several sponsor names; the case makes no claim about the company family.
