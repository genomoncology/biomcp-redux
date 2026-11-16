# sdlc/

The plan, decisions and tickets for BioMCP 1.0. pm reads and changes ticket status.

| Path | What it is |
| --- | --- |
| [planning/architecture.md](planning/architecture.md) | The layers, the crates, the adapter shape, the output contract and the dependency direction |
| [planning/rust-standards.md](planning/rust-standards.md) | How Rust is written here, the ratchets and which tool enforces each rule |
| [planning/slices.md](planning/slices.md) | How each vertical slice runs, from investigation to tickets |
| [planning/slices/](planning/slices/) | One design file per slice, written when its investigation lands |
| [plan.md](plan.md) | The current milestone's stages, why and in what order |
| [planning/milestones.md](planning/milestones.md) | Every milestone, current and future |
| [decisions/](decisions/) | Dated rulings |
| [tickets/](tickets/) | One file per ticket, numbered from 0001 |

Each slice adds its own design file, `planning/slices/<entity>.md`, when its investigation lands.

Records name internal machines by role, such as the build machine or the PM's machine. In evidence paths, `{workspace}` is the workspace root on the named machine, `{home}` its home folder, and `{repo}` this repository's working name, so `data/{repo}-evidence/0640/` is the ticket evidence folder. `scripts/check_docs.py` fails on internal names here as in every public file, except in `data/`, the machine-readable registry `pm disk` reads.
