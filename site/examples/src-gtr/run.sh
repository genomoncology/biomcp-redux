# Download the gtr bundle first when this machine lacks it; the page shows show.sh.
biomcp --offline search diagnostic --source gtr --gene BRAF --limit 2 | jq -e '.status != "failed"' > /dev/null || biomcp sync gtr > /dev/null
bash show.sh
