# Label history replies

Recorded live with `biomcp --record` in pm job j43418 on 2026-11-19, from the Drugs@FDA API and FDA's archived label PDFs at https://www.accessdata.fda.gov. Each label PDF sidecar is replaced by a small synthetic PDF, written by `scripts/label_history_fixtures.py`, that shows in plain Helvetica the text BioMCP's own reader took from the original, so the cases test finding the section, the search and the reply on the real labels' text within the fixture budget. `label-history-unread` is `label-history-livalo` with the SUPPL 20 label cut to its first 300 bytes, so that version cannot be read.

Each original label below is named by its FDA URL, SHA-256 and size.

## label-history-olumiant

- https://www.accessdata.fda.gov/drugsatfda_docs/label/2026/207924s010lbl.pdf: sha256 429ee947cc7d6c7f6dbc584b236e9473f9e19bb0ff9f45f72151a3ee5ef6a962, 969398 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2018/207924Orig1s000lbl.pdf: sha256 fb2ab438ff127e3cb61b92675da568aeb8cb1bcebc570085204d713736ea8a9b, 1137374 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2020/207924s002lbl.pdf: sha256 784ea9b311eed9214a6105130eabd05803036061b41ed03d5268be9f0c472152, 985195 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2022/207924Orig1s006lbl.pdf: sha256 78ce9553c664c7d4146e2fa925106abb5b4f01008186f6a0e96b54a7d1678caf, 1576189 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2021/207924s004lbl.pdf: sha256 187f667d7cb6bda0164c0a6d6efae090d56ac81380d2b872187ca337ace78d5f, 1391034 bytes

## label-history-xeljanz

- https://www.accessdata.fda.gov/drugsatfda_docs/label/2021/203214s028,208246s013,213082s003lbl.pdf: sha256 6c2a35794405c20d3cfd0a5197600690a02bf8ef6c561d90b69e840aa0912142, 3660257 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2012/203214s000lbl.pdf: sha256 59aea7c7eccf6f966b91337f0bcda23d4d009cb2e355b07063e9a7fe18630f71, 422010 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2020/203214s026lbl.pdf: sha256 b1e8f609fddea782ec5bcd20f94c85f25d03179280aab3b3053116275483d8bc, 3979102 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2026/203214s042lbl.pdf: sha256 21290f871e22e5c897aed537266bd0d82500b50627d8fba53e4156216092de35, 2155944 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2019/203214s025lbl.pdf: sha256 1b8262c4d3aba60ef2ee5d5221c484ab1e2901e705f9fc20c6ebab93acb9e41b, 2457002 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2021/203214s031lbl.pdf: sha256 7d31333c76b4ddb48d4df52976d73ebdd2d7462136e52ea0753e18afb17bed87, 2756417 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2018/203214s020,208246s006lbl.pdf: sha256 af831846d512800dbbf76a11fd7b49a59028c2bc4b1d1b11717e279994efbc9a, 2277879 bytes

## label-history-rinvoq

- https://www.accessdata.fda.gov/drugsatfda_docs/label/2023/211675s012lbl.pdf: sha256 6fe26901a1d5133db0716809ee07d6dc52286059e6224aed421d6a39006a0f75, 5321986 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2019/211675s000lbl.pdf: sha256 9d5db1cc76fd1f38b5ee6d713ed4523119bdf3af463856875f824ddf19c2426b, 485707 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2026/218347Orig1s008,211675Orig1s034lbl.pdf: sha256 a2548a983da3db15e6c2288e8cdceb6ed8f8c6c5059eb6e9add74eae6e66feb8, 2128698 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2022/211675s004lbl.pdf: sha256 3cd07face2fc651a8098dcff72677a95181a35d5c7853f70efaa95d292052404, 2318887 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2020/211675s001lbl.pdf: sha256 ed470bf54dd855267b2033b8fc668ac19f5d82386a38406a2f47dff5a7366a03, 402049 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2021/211675s008lbl.pdf: sha256 a3fb885330770ab012922f3fef1017cf32fe6ad22b2606e862433009fbb87a02, 1429043 bytes

## label-history-aimovig

- https://www.accessdata.fda.gov/drugsatfda_docs/label/2025/761077Orig1s025lbl.pdf: sha256 fd1a7d34f60539fe078cd722c29c950a632a5aad13c9fecbf58674b71a1f80fa, 2922236 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2021/761077s009lbl.pdf: sha256 c2dcde80f4ba7ed71ae6d9489bc20bc1b2497c62509c577eb58f499c44cbe9c1, 4738365 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2022/761077s015lbl.pdf: sha256 09e5b9501adb51cb9a5c0697fc660400986aa99eb8b2360c3be4ec27274b5706, 6183110 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2022/761077Orig1s017Correctedlbl.pdf: sha256 ba48cc8433c307e2a8c25911cc728d3fd4ad3f90d831593fe9ed90710a210953, 20307813 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2018/761077s000lbl.pdf: sha256 67bfd3328fb0f101d51d965e53edb86aa38f7c0834b3e902ef9026b2e3feec40, 2197393 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2024/761077s022lbl.pdf: sha256 0761b624184d1cfee32f4ea1e527db66e52def353755b3ccb5f472f6825c48e1, 2035693 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2025/761077s026lbl.pdf: sha256 3f23eab4c43f449dabff6dac0060c936c1a805f95d5ad51969c64c3e684a141d, 3498011 bytes

## label-history-livalo

- https://www.accessdata.fda.gov/drugsatfda_docs/label/2013/022363s012lbl.pdf: sha256 bd5679e5ea930fa576effc20b5985bb90ab201d06b287eb12cdc5a0d4f915f82, 993867 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2019/022363s017lbl.pdf: sha256 a80d41f0f3aba8ccf29558b1905ae568adb8cdb8b1583783b3c6c8ed074cdfcf, 1164184 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2022/022363s020lbl.pdf: sha256 b2f89318d4fca0a231e8f00e8519dc8d3e95e08d329c53ee0def815b312f6e06, 1376410 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2020/022363s018lbl.pdf: sha256 44ce5616665019bc1b4b753ebc08873c22d2ce4527ec0ddae458ea79500edd33, 1194957 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2024/022363s022lbl.pdf: sha256 1da833f39f3aaccb8cb634d89d1a4b40eb3ed071e4fff09fb50dadf9f9872f0a, 1360947 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2009/022363s000lbl.pdf: sha256 6a9dc8a9b1a0d3fb7afbdd0979d6c374aa12cf1c5429f8fa3db348462032fdd1, 534677 bytes

## label-history-never

- https://www.accessdata.fda.gov/drugsatfda_docs/label/2024/022363s022lbl.pdf: sha256 1da833f39f3aaccb8cb634d89d1a4b40eb3ed071e4fff09fb50dadf9f9872f0a, 1360947 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2009/022363s000lbl.pdf: sha256 6a9dc8a9b1a0d3fb7afbdd0979d6c374aa12cf1c5429f8fa3db348462032fdd1, 534677 bytes
