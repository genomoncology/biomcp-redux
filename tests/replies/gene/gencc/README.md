# GenCC fixture provenance

The successful GenCC exchange contains the three ODC1 rows from an existing public export capture. The capture request was `GET https://thegencc.org/download/action/submissions-export-csv?format=new` on 2026-09-06 at 20:41:34 UTC. The publisher returned HTTP 200 with `Content-Type: text/csv; charset=UTF-8`, ETag `"6ebdbf28b305e99e349ed827a219214b"` and Last-Modified `Sun, 06 Sep 2026 06:00:29 GMT`.

The full capture had 26,826,920 bytes and SHA-256 `d66768e9146ad833369eca7f6099af87ce4710dcc14078b07baa11eb29ccde82`. The retained CSV keeps the 31 header fields and three HGNC:8109 rows in publisher order. It uses minimal CSV quoting and LF record terminators. No retained decoded cell was rewritten. Its 2,315 bytes have SHA-256 `71ce30cda7d25bf2c08eccd0505997fc6b4b9cddf635907b474c18e86e8091f9`.

The initial HTTP 429 exchange and its retry delay are authored. The HGNC exchange is an authored identity fixture. These exchanges prove local behavior and make no claim about current publisher responses.

GenCC documents the export on its [download page](https://thegencc.org/download) and publishes its [terms](https://thegencc.org/terms). Attribute GenCC and the contributing submitters when reusing its data.
