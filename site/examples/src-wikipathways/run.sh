# Download the wikipathways bundle first when this machine lacks it; the page shows show.sh.
biomcp --offline search pathway MAPK --source wikipathways --limit 2 | jq -e '.status != "failed"' > /dev/null || biomcp sync wikipathways > /dev/null
bash show.sh
