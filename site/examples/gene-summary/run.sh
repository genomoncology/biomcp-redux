biomcp get gene BRAF | jq -c '.result.items[0].answer.gene.summary | {source, dated, text: (.text[0:160] + "...")}'
