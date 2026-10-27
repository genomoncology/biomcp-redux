biomcp --help | jq
echo "exit code ${PIPESTATUS[0]}"
biomcp -h | jq .error.code
biomcp | jq .error.code
biomcp help get gene | jq '{status, command: .result.commands[0].command}'
