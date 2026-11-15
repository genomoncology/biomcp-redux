# CDC WONDER VAERS MMR age table after a rate-limit refusal

Both exchanges were recorded unchanged from CDC WONDER's public VAERS endpoint (`POST https://wonder.cdc.gov/controller/datarequest/D8`, vaccine `MMR`, grouped by age, form body SHA-256 `c768ab536ccd6014df03043d5d9b49e4271bb74c3cabf9cb0ceb5bb3393d47d5`) on 2026-11-15 by `biomcp --record` in pm job j41113, started a second after an MMR seriousness table from the same address.

1. HTTP 429, `text/html;charset=ISO-8859-1`, 391 bytes, SHA-256 `dea57953125234ea47645db3a6e397354a3c75c5401663fd3b3e54646f04a279`, no `Retry-After` header. Its message asks for at least 15 seconds between consecutive requests.
2. 18 seconds later, after BioMCP's 15-second wait: HTTP 200, 60,283 bytes, SHA-256 of the Latin-1 body `f3ee0305ed20ad4cf8ca9991f46dc52d4bc085ab8d5e18296ef738d9335cfa1f`. Dataset vintage 10/30/2026, 13 age rows, report total 84,145.
