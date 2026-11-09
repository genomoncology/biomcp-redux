biomcp search gwas --variant rs7903146 --limit 3 \
  | jq -r '.result.items[]
    | [.study, (.traits | join(";")),
    (.alleles[0].rs_id + "-" + .alleles[0].allele),
    "\(.exact_p_value.mantissa)e\(.exact_p_value.exponent)"] | @tsv'
