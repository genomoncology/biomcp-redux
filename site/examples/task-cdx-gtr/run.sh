biomcp --offline search diagnostic --source gtr --gene KRAS --limit 1 | jq -e '.status != "failed"' > /dev/null || biomcp sync gtr > /dev/null
bash show.sh
