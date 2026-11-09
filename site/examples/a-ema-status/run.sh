biomcp get drug EMEA/H/C/002409 regulatory --source ema \
  | jq '.result.regulatory.data.records[] | {status,
    authorised: (.dates[] | select(.role == "marketing_authorisation_date")
      | .date)}'
