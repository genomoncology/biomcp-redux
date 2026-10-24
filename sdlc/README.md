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
