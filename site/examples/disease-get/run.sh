biomcp get disease MONDO:0005105 | jq -r '.result.concept.id + "  " + .result.concept.label, (.next[].args | join(" "))'
