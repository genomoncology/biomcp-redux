port=$((20000 + RANDOM % 20000))
timeout 5 biomcp serve --http 0.0.0.0:$port --allow-host localhost > /dev/null 2>&1 &
sleep 1
echo "Host localhost: $(curl -s -o /dev/null -w '%{http_code}' --noproxy '*' http://localhost:$port/health)"
echo "Host 127.0.0.1: $(curl -s -o /dev/null -w '%{http_code}' --noproxy '*' http://127.0.0.1:$port/health)"
wait
