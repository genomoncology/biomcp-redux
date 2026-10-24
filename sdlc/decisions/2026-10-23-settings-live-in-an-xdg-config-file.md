# Settings live in an XDG config file

Decided 2026-10-23 by the PM, applying the 0.9 review's finding of 88 environment reads in 47 files.

- `biomcp` reads `$XDG_CONFIG_HOME/biomcp/config.json`, or `~/.config/biomcp/config.json` when `XDG_CONFIG_HOME` is unset. `--config PATH` names another file. A missing default file is not an error. A missing named file is.
- Precedence, highest first: the command-line flag, the config file, the built-in default. Every setting in the file has a flag of the same name.
- Source API keys may also come from their documented environment variables, read only by the config module.
- An unknown key, a wrong type or a bad value is refused with a named error that gives the key.
- Only the binary reads the file or the environment. Libraries take a typed context.
- Tests set `XDG_CONFIG_HOME` to an empty temporary directory, so a developer's own file never changes a result.
