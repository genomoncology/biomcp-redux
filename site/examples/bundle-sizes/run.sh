# Synchronize every bundle, then print its rows and source file size, and the store folder's size.
printf 'bundle\tstatus\trows\tsource file MB\n'
for b in hgnc clingen wikipathways ema ddinter who-pq who-ivd gtr fda-cdx; do
  biomcp sync "$b" | jq -r --arg b "$b" '[$b, .status,
    (.result.rows // .result.search.total_rows // "-"),
    ((.result.bytes // ((.result.search.bytes // 0) + (.result.get.bytes // 0)) // 0) / 1e6 * 10 | round / 10)] | @tsv'
done
printf 'store folder: %s\n' "$(du -sh "${XDG_CACHE_HOME:-$HOME/.cache}/biomcp" | cut -f1)"
