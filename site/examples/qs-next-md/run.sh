biomcp get gene BRAF --format md | sed -n '/^## Next/,/^## Query/p' | sed '$d'
