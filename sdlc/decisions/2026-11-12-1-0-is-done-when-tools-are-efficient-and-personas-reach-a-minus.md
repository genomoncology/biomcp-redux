# 1.0 is done when the tools are efficient and every persona reaches A-

Decided 2026-11-12 by Ian, passed on by Jarvis (m35087) from Ian's 1.0 and persona briefs.

## Decision

- 1.0 is the rebuild on shared BioData, the multi-tool MCP and hosting. The Library and the website come after 1.0.
- Done means efficient tools and all eleven personas at A- or better. A top model (Ian named Sol, Opus and Fable as the level) reviews every persona's actual run and report and judges both correctness and whether the tools were efficient enough. No number is the bar.
- BioMCP does not need to beat web search plus curl. This confirms Ian's 2026-11-11 ruling to stop the token race with curl.
- Fix the major bugs the review finds before release; polish the command line, skills and help for agents.

## What it changes

- The release gate for milestone 7 is the persona review (biomed-agent-bench), not the curl comparison.
- Persona findings become tickets here as before; the review also names missing sources, which go to `planning/sources-to-add.md` or a ticket.

## Where we stand

- The blind persona rerun of 2026-11-12 (bench 0037, BioMCP 58d83b6da) graded none of the eleven at A-: three B+, two B, three B-, three C+. Since then 0850, 0837, 0846, 0843, 0853, 0854, 0840, 0844 and 0830 landed; 0856 (variant spellings) and 0855 are in review.
- The November 10 self-grades (six A-, five B+) are not the current measure.
