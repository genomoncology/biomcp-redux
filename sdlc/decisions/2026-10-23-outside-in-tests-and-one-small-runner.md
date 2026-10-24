# Outside-in tests and one small runner

Decided 2026-10-23 by Ian.

- mustmatch and executable Markdown specs are dropped.
- One small custom runner drives the built binary and the MCP tool. Each case is one YAML file under `tests/cases/<feature>/` with a command, a recording and assertions written as JSON pointers.
- Cases run over recorded replies and never touch the network. Recording and replay live in the transport, as `2026-10-23-one-transport.md` says.
- Tests are outside-in only. Unit tests appear only where they truly make sense, such as a parser table, each with a stated reason.
- Every command and option has at least one case. Each case tells a story, names where it was harvested and checks an answer verified against the live source.
- Cases are harvested from published papers and real questions. No case names or embeds a benchmark set. The runner takes `--cases DIR`, so case sets that cannot be published run from outside the repo.
