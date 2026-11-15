The synthetic recordings contain invented source text. Live folders contain only CC BY paper text; the bucket JSON record supplies each license.

On 2026-10-24, a native `--record search article "28944238[uid]" --limit 1` supplied the unchanged ESummary exchange reused in live-introduction for PMID reader resolution. A native `--record article text 21217686 --part 1` supplied the unchanged ESummary in live-no-pmcid. Neither recording fetched a paper body. New synthetic adapter variants copy the complete native live-introduction recording and edit response exchanges only; their JATS is invented, including license boundary cases.

Semantic Scholar's API terms are noncommercial. In `live-all/semantic-scholar/`, the citation counts are synthetic (0695 changed them) and the open-access reply is `null`, so no real Semantic Scholar data remains (0920).
