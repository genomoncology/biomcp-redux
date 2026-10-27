echo '{}' > empty-config.json
NO_PROXY='::1,10.0.0.0/8' biomcp --config empty-config.json --offline health | jq -c '{status, error}'
echo "exit code ${PIPESTATUS[0]}"
NO_PROXY='localhost' biomcp --config empty-config.json --offline health | jq -c '{status, sources: (.result.sources | length)}'
rm empty-config.json
