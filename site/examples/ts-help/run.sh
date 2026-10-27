biomcp --help
echo "exit code $?"
biomcp -h | jq -c .error.code
biomcp | jq -c .error.code
biomcp help get gene | jq -c '{status, command: .result.commands[0].command}'
