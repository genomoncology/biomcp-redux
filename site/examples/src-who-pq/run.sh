# Download the who-pq bundle first when this machine lacks it; the page shows
# show.sh.
biomcp --offline search drug artesunate --source who-pq --limit 2 \
  | jq -e '.status != "failed"' > /dev/null || biomcp sync who-pq > /dev/null
bash show.sh
