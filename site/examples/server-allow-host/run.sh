port=$((20000 + RANDOM % 20000))
timeout 5 biomcp serve --http 0.0.0.0:$port \
  --allow-host localhost > /dev/null 2>&1 &
sleep 1
status() { curl -s -o /dev/null -w '%{http_code}' --noproxy '*' "$1"; }
echo "Host localhost: $(status http://localhost:$port/health)"
echo "Host 127.0.0.1: $(status http://127.0.0.1:$port/health)"
wait
