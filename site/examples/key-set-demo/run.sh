printf 'example-not-a-key\n' | biomcp key set ncbi --config demo.json | jq -c '.result.entries[]'
jq -c . demo.json
ls -l demo.json | cut -c1-10
biomcp key list --config demo.json | jq -r '.result.entries[] | [.source, .environment, .state] | @tsv'
biomcp key remove ncbi --config demo.json | jq -c '.result.entries[]'
rm demo.json .config.lock
