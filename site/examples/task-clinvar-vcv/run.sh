biomcp get variant clinvar:13961 clinvar \
  | jq -r 'def cut: if length > 22 then .[:22] + "…" else . end;
    .result.candidates[0].sections.clinvar.data.assertions[]
    | select(.kind == "vcv")
    | [.domain, (.statement.classification.label | cut),
      (.statement.strength.label | cut),
    .statement.attribution.evaluated] | @tsv'
