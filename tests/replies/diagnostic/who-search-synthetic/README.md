# WHO IVD search: synthetic export

WHO IVD is a noncommercial source, so this replay holds no real WHO rows (0695, 0920). The ten rows are invented: names, product codes, manufacturers and years. The product IDs are invented too; 0920 replaced 0999-999-99's earlier value, which was a real WHO product ID, and none appears in the WHO export 0876 recorded. Only the marker words (HIV, HIV-1 / HIV-2, HPV) follow WHO's spelling, because the marker filter matches them. The rows keep the search cases: seven HIV products in export order with one product on two rows, `SHIV` that must not match `HIV`, and a manufacturer `xNorth-North-North` that must not match `North` as a substring.

Synthetic: who-ivd
