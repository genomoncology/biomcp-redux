echo '{"formatt": "md"}' > typo.json
biomcp key list --config typo.json | jq
echo "exit code ${PIPESTATUS[0]}"
rm typo.json
