PDFs for the PDF reader's cases (tickets 1068 and 1069). The real ones were fetched from accessdata.fda.gov on 2026-11-19; pages were copied out unchanged with pypdf to keep them small. Each `.txt` is the reference reading of the PDF beside it by poppler's `pdftotext` 24.02.0 with default options.

- `form-object-carton.pdf`: page 8 of the Rinvoq (NDA 211675) supplement 29 label, https://www.accessdata.fda.gov/drugsatfda_docs/label/2025/211675Orig1s029lbl.pdf (source SHA-256 09c767f4b5ec3a8f9de28af072509de9bc997427789966ffc4e83875efc41bec). Its text sits in form XObjects, and its own content stream only paints them.
- `refused-map-letter.pdf`: the Tagrisso (NDA 208065) supplement 21 approval letter, unchanged, https://www.accessdata.fda.gov/drugsatfda_docs/appletter/2020/208065Orig1s021ltr.pdf (SHA-256 d99477c759dd8111f4a49be1d1d75082a26d3b219e68c771ff52f071621a9a74). Its fonts' ToUnicode maps put `def` inside `/CIDSystemInfo`, which lopdf refuses; without them "NDA 208065" decoded as "1'$ 6833".
- `refused-map-label-page.pdf`: page 1 of the Olumiant (NDA 207924) supplement 2 label of 2020-07-08, https://www.accessdata.fda.gov/drugsatfda_docs/label/2020/207924s002lbl.pdf (source SHA-256 784ea9b311eed9214a6105130eabd05803036061b41ed03d5268be9f0c472152), with the same kind of map.
- `label-xeljanz-2012.pdf`: the Xeljanz (NDA 203214) original label of 2012-11-06, unchanged, https://www.accessdata.fda.gov/drugsatfda_docs/label/2012/203214s000lbl.pdf (SHA-256 59aea7c7eccf6f966b91337f0bcda23d4d009cb2e355b07063e9a7fe18630f71). The label-history section tests in `crates/biomcp-query` read it whole (ticket 1066); its contents list leaves out the boxed warning its body opens with.
- The synthetic PDFs of the other cases are built inside `pdf_reader.rs`.

The two Symbol fixtures contain one page each, copied with pypdf from the review's FDA labels. The expected characters were checked against the rendered pages.

- `symbol-lipitor.pdf` is page 13 of [the Lipitor label](https://www.accessdata.fda.gov/drugsatfda_docs/label/2012/020702s062s063lbl.pdf), source SHA-256 `5aa123a313951edbffb6c794c55a86b7817ce3499b4c92e07be0bc170e416d80`. Table 3 reads “↑ 8.7 fold”.
- `symbol-tagrisso.pdf` is page 8 of [the Tagrisso label](https://www.accessdata.fda.gov/drugsatfda_docs/label/2020/208065s021lbl.pdf), source SHA-256 `e8b4fb5945691feb9837df878d9c8feaf2e34cc6f761400aa7eab7551cd7d43f`. The footnote after “Fatigue” is β.

The number fixtures contain one page each from saved FDA documents. The pages were copied with pypdf, excluding annotations and the structure-tree pointer.

- `numbers-rinvoq.pdf` is page 26 of the [original Rinvoq label](https://www.accessdata.fda.gov/drugsatfda_docs/label/2019/211675s000lbl.pdf), source SHA-256 `9d5db1cc76fd1f38b5ee6d713ed4523119bdf3af463856875f824ddf19c2426b`. The numbers after “Week 26” and “Week 24” are superscript footnote markers.
- `numbers-lipitor.pdf` is page 5 of the [2009 Lipitor label](https://www.accessdata.fda.gov/drugsatfda_docs/label/2009/020702s057lbl.pdf), source SHA-256 `abb4c1e91bc598c3692795ba732f65079832626011847e962164e9c549b29787`. The digits and decimal point in `p=0.0005` are drawn in separate text passes.
- `numbers-fda-letter.pdf` is page 3 of the [Olumiant supplement 2 approval letter](https://www.accessdata.fda.gov/drugsatfda_docs/appletter/2020/207924Orig1s002ltr.pdf), source SHA-256 `19f2a7a35cfc1c2b9e969357aee0b39bb7a2f3636840d2b30a0b846aabe08af9`. Its phone number wraps after the hyphen.
