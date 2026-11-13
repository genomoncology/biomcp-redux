# Orphadata fixture provenance

The two Orphadata exchanges are cut from the live files fetched on 2026-11-13: `GET https://www.orphadata.com/data/xml/en_product9_prev.xml` (HTTP 200, `Content-Type: application/xml`, Last-Modified `Mon, 29 Jun 2026 12:08:03 GMT`, 16,178,169 bytes, SHA-256 `6b492b3cc61e5a0327de12f59386a07a071760a938d0bbd8f525bc4a5f71b7f0`, 6,728 disorders) and `GET https://www.orphadata.com/data/xml/en_product9_ages.xml` (HTTP 200, `Content-Type: application/xml`, Last-Modified `Mon, 29 Jun 2026 12:08:02 GMT`, 6,948,952 bytes, SHA-256 `38fef7e9e398addb147e57df7bb523e5ef072e03c0208ae81b37a2fd200af1ab`, 7,790 disorders). Both files carry `JDBOR date="2026-06-23"` and version `1.3.42 / 4.1.8 [2025-03-03]`.

Each retained body keeps the file's header, its licence element and the complete Kabuki syndrome (ORPHAcode 2322) disorder element, unchanged, then closes the list. The `DisorderList count` attribute still states the full file's count. No retained text was rewritten. The cut prevalence body is 4,052 bytes (SHA-256 `5c35b51762f6824d22fb1872fb559be7ebfb5be4b8db7bebc7a32b805b6bb258`) and the ages body 1,673 bytes (SHA-256 `d2289ac5e66d7abebcf61500bb60daed7020f142d5905d613eafd1cfdbc7846e`).

The two MyDisease exchanges for MONDO:0016512 and MONDO:0007254 were recorded live on 2026-11-13 by `biomcp --record`.

Orphadata Science data is licensed under CC BY 4.0 (https://sciences.orphadata.com/faq/). Orphadata Science: Free access data from Orphanet. © INSERM 1999. Available on https://sciences.orphadata.com/ . Data version 2026-06-23.
