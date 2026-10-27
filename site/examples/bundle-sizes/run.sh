# Synchronize every bundle, then print its rows and source file size,
# and the store folder's size.
printf 'bundle\tstatus\trows\tsource file MB\n'
for b in hgnc clingen wikipathways ema ddinter who-pq who-ivd gtr fda-cdx; do
  biomcp sync "$b" | jq -r --arg b "$b" '
    if .status == "failed" then error("sync \($b) failed") else . end
    | (.result.bytes // ((.result.search.bytes // 0)
        + (.result.get.bytes // 0)) // 0) as $bytes
    | [$b, .status, (.result.rows // .result.search.total_rows // "-"),
       ($bytes / 1e6 * 10 | round / 10)] | @tsv'
done
store="${XDG_CACHE_HOME:-$HOME/.cache}/biomcp"
printf 'store folder: %s\n' "$(du -sh "$store" | cut -f1)"
