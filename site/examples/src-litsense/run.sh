biomcp search article 'BRAF V600E resistance' --source litsense --limit 2 \
  | jq 'def cut: if length > 56 then .[:56] + "…" else . end;
    {status, sources: [.sources[] | {source, state, reason}]},
    (.result.items[] | {pmid, sentence: (.sentence.text[0:120] | cut)})'
