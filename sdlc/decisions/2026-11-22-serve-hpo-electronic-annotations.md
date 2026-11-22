# Serve HPO's electronic annotations

Ian, November 22, 2026, in chat with the BioMCP PM. Asked whether BioMCP should serve the HPO disease annotations it filtered out, he said "Seems like a bad idea. I don't understand why it was there." Told the filter's source, HPO's annotation documentation on IEA rows derived from OMIM's Clinical Features sections ("Please note that you need to contact OMIM in order to reuse these annotations in other software products"), and asked again, he answered "yes".

BioMCP serves every row of HPO's `phenotype.hpoa`, including those with evidence code `IEA`, in local and hosted modes. The filter added with ticket 1099 (`crates/biomcp-adapters/src/hpo/release.rs`) is removed, and the reply's coverage notice says the annotations come from HPO and that HPO marks its IEA rows as derived from OMIM.
