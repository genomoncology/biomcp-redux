echo '{"formatt": "md"}' > typo.json
biomcp key list --config typo.json
echo "exit code $?"
rm typo.json
