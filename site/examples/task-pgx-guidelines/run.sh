biomcp get pgx guidelines --gene CYP2C19 --source cpic | jq -r '.result.items[] | [.guideline_name, .guideline_url] | @tsv'
