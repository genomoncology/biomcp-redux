# 0348 — Prove that fetched source text reaches agents as inert data

Status: COMPLETE.
Landed: ef00dcdfab3a6191a969cd371896dd7471e63d36

Milestone: 5

## Outcome

Text that BioMCP fetches from a source reaches the agent as data. Markdown output escapes source text so that HTML, Markdown links, images, headings and code fences from a source field render as literal text, and only BioMCP's own next steps and source links are live. JSON keeps source text as string values. The MCP server instructions and the skill text say that returned fields are source records, never instructions to follow.

- Why: Ian's security screening request through Jarvis (m10245, 2026-10-27). Agent tools face attacks that hide instructions in upstream content.
- Starts from: the Markdown renderer, the MCP server instructions, `skills/SKILL.md` and the replay case tooling.
- Rules: BioMCP never deletes or rewrites source text to hide content; it escapes it. No text heuristics decide what looks like an instruction, as `2026-10-24-no-text-heuristics.md` says.
- Proof: a replay case whose recorded source reply puts `<script>`, `<img src=x>`, a Markdown link, a heading, a code fence and the words "ignore previous instructions" in a title and an abstract shows all of them as escaped literal text in Markdown and as plain strings in JSON; the MCP instructions contain the data statement.
- Changes: the Markdown renderer if a gap is found, MCP instructions, SKILL.md and cases.
- Defers: nothing. This ticket joins the 0336 refresh.
