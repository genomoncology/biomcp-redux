biomcp get pgx diplotype --gene CYP2C19 --source cpic --diplotype '*2/*2' \
  | jq '.result.diplotypes[] | {diplotype, function1, function2, generesult}'
