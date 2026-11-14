# WHO IVD year window: synthetic export

WHO IVD is a noncommercial source, so this replay holds no real WHO rows (0910, replacing the export 0876 recorded). `who-ivd/` holds a synthetic 34-row export of 20 products in the real export's shape: the same columns in the same order, quoting (cells with a space, comma or line break quoted, single words bare), a quoted cell with a line break, a curly apostrophe, line feeds and no final newline. Every product name, product code, WHO product ID and manufacturer is invented and none appears in WHO's export of 2026-11-13.

It keeps the edge cases the case relies on:

- Products are listed newest year first, and a product's rows count once: 9111-911-01 has two rows, 9114-914-00 three and 9119-919-00 two.
- The marker matches as a whole word inside a list: 19112-912-00 is listed for `HBV, HIV, Syphilis`.
- HIV from 2023 on gives nine products, four from 2025, four from 2024 and one from 2023; the 2022, 2019 and 2012 HIV products fall outside the lower bound.
- 2023 alone holds four products (two HCV, one HIV, one HPV), so `--limit 3` returns three of four.
- One product's rows are split, with its last row at the end of the file, as in WHO's export.
