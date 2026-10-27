biomcp --offline search diagnostic --source fda-cdx --biomarker KRAS --limit 1 | jq -e '.status != "failed"' > /dev/null || biomcp sync fda-cdx > /dev/null
bash show.sh
