biomcp get trial NCT06887088 locations | jq -r '.result.items[0].sections.locations.data.items[0:3][] | [.facility, .city, .country, .status] | @tsv'
