# Authored drug recall page whose records all lack a recall number

Authored from the real `--status Ongoing` page recorded in pm job j41420 (`tests/replies/drug/recall-status-pages/`): every record's `recall_number` is blanked to `""`, the value openFDA itself sent for event 100178 on that page. Everything else is the recorded body. It tests a page whose matched records are all declined; openFDA has not been seen to send such a page.
