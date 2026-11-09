port=$((20000 + RANDOM % 20000))
timeout 5 biomcp serve --http 127.0.0.1:$port &
sleep 1
curl -s --noproxy '*' http://127.0.0.1:$port/health
echo
wait
