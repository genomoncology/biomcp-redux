# Download the fda-cdx bundle first when this machine lacks it; the page shows show.sh.
biomcp --offline search diagnostic --source fda-cdx --biomarker BRAF --limit 2 | jq -e '.status != "failed"' > /dev/null || biomcp sync fda-cdx > /dev/null
bash show.sh
