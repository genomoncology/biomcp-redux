# CDC WONDER VAERS refuses a table twice (synthetic)

Synthetic. The one request is the MMR age table of ../vaers-rate-limited-retry. Both exchanges are the HTTP 429 WONDER returned live in pm job j41113 (391 bytes, SHA-256 `dea57953125234ea47645db3a6e397354a3c75c5401663fd3b3e54646f04a279`, no `Retry-After`); the second copy's `fetched` time is 15 seconds later. Nothing else is changed. Live, a second refusal happens when another client's request is accepted during BioMCP's 15-second wait.
