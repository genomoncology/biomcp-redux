count() { biomcp search trial --condition melanoma --status RECRUITING "$@" \
  --count-only \
  | jq -r '[.sources[] | select(.source == "ctgov") | .count][0]'; }
printf '%s\t%s\n' "$(count --variant 'BRAF V600E')" "--variant 'BRAF V600E'"
printf '%s\t%s\n' "$(count --variant 'V600')" "--variant 'V600'"
printf '%s\t%s\n' "$(count --gene BRAF)" "--gene BRAF"
printf '%s\t%s\n' "$(count --criteria 'BRAF')" "--criteria 'BRAF'"
printf '%s\t%s\n' "$(count)" "(condition and status only)"
