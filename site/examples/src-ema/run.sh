# Download the ema bundle first when this machine lacks it; the page shows
# show.sh.
biomcp --offline search drug Zelboraf --source ema --limit 1 \
  | jq -e '.status != "failed"' > /dev/null || biomcp sync ema > /dev/null
bash show.sh
