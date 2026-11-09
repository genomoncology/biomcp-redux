printf 'example-not-a-key\n' | biomcp key set ncbi --config demo.json \
  | jq '.result.entries[]'
jq . demo.json
ls -l demo.json | cut -c1-10
biomcp key list --config demo.json \
  | jq -r '.result.entries[] | [.source, .environment, .state] | @tsv'
biomcp key remove ncbi --config demo.json | jq '.result.entries[]'
rm demo.json .config.lock
