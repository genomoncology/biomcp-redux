# Drug name lookup recovery

These synthetic exchanges replay FDA's no-match response and MyChem's empty
ChEMBL-copy lookup. They work with the synthetic MyChem name pages in
`drug/name-get-pages` and the aliases in `drug/source-aliases`.

The STI-571 and blarcamesine MyChem replies are synthetic. The imatinib label
reuses a synthetic label with invented headings and text from
`drug/several-sections-name`. The approvals case reads the already recorded
imatinib application from `drug/markets`, with only the original approval retained. These examples test name resolution.
