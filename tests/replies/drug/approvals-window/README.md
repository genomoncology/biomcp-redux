# Drugs@FDA approvals by date window and whole history

The reply for `https://api.fda.gov/drug/drugsfda.json?search=application_number%3A%22BLA125514%22&limit=1` (HTTP 200) was recorded unchanged from the public openFDA Drugs@FDA endpoint on 2026-11-13 by `biomcp --record` in pm job j39043, without a key. The publisher's dataset date was 2026-11-12. The application held 128 submissions: 1 original and 127 supplements, 26 with a status date on or after 2024-01-01.

No FDA letter is recorded: every page in the case asks for more than 50 rows, so it reads no supplement letters.
