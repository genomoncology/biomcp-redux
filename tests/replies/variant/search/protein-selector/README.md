# Default protein selector replays

MyVariant queries captured on 2026-11-24 use the default dbNSFP selector from main and the SnpEff selector selected on main with noncommercial and licensed sources disabled. The replay bodies keep candidate identities, coordinates, identifiers, counts and source order. They omit source annotation text and transcripts.

The SnpEff V600E query contains a row whose stated reference base disagrees with the GRCh37 reference. The NCBI reference replay preserves that check and the returned allele remains the one from the default dbNSFP query.
