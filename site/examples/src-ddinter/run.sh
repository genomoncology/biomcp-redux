# Download the ddinter bundle first when this machine lacks it; the page shows show.sh.
biomcp --offline get drug DDInter1925 interactions --source ddinter --section-limit 2 | jq -e '.status != "failed"' > /dev/null || biomcp sync ddinter > /dev/null
bash show.sh
