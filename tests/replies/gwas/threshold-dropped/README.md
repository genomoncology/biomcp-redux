# GWAS threshold walk with a dropped row: authored from a real recording

`gwas/` holds one reply: GWAS Catalog's first page of 3 associations for PubMed 17529967, sorted by p-value (`https://www.ebi.ac.uk/gwas/rest/api/v2/associations?direction=asc&page=0&pubmed_id=17529967&size=3&sort=p_value`), recorded on 2026-11-15 by `biomcp --record` in pm job j41438. The same unchanged reply is in `../threshold-total/`.

This copy is authored: `pvalue_mantissa` is removed from the second row (association 6111, p-value 1e-36). Every other row, field and value is unchanged and in order. The live source gave that row a mantissa; the edit stands in for a row with no exact p-value, so the case can show a finished walk whose count at or below the cut is unknown.
