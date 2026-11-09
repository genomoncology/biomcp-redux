# Download the who-ivd bundle first when this machine lacks it; the page shows
# show.sh.
biomcp --offline search diagnostic --source who-ivd --limit 2 \
  | jq -e '.status != "failed"' > /dev/null || biomcp sync who-ivd > /dev/null
bash show.sh
