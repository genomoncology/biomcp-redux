# Label history replies

Recorded live with `biomcp --record` in pm job j43445 on 2026-11-19, from the Drugs@FDA API and FDA's archived label PDFs at https://www.accessdata.fda.gov. Each label PDF sidecar is replaced by a small synthetic PDF, written by `scripts/label_history_fixtures.py`, that shows in plain Helvetica the text BioMCP's own reader took from the original, so the cases test finding the section, the search and the reply on the real labels' text within the fixture budget. `label-history-unread` is `label-history-livalo` with the SUPPL 20 label cut to its first 300 bytes, so that version cannot be read.

To rebuild a fixture, record the call with `biomcp ... --record DIR`, then write the reader's text for each recorded label PDF sidecar with BioMCP's own reader, which takes the page bound on its first input line and prints its text after one header line:

```sh
for f in DIR/drugsfda/*.bin; do
  { printf '400\n'; cat "$f"; } | biomcp __pdf-read 1073741824 | tail -n +2 > "TEXT/$(basename "$f").txt"
done
python3 scripts/label_history_fixtures.py TEXT DIR
```


Each original label below is named by its FDA URL, SHA-256 and size.

## label-history-olumiant

- https://www.accessdata.fda.gov/drugsatfda_docs/label/2026/207924s010lbl.pdf: sha256 429ee947cc7d6c7f6dbc584b236e9473f9e19bb0ff9f45f72151a3ee5ef6a962, 969398 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2018/207924Orig1s000lbl.pdf: sha256 fb2ab438ff127e3cb61b92675da568aeb8cb1bcebc570085204d713736ea8a9b, 1137374 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2022/207924s007lbl.pdf: sha256 b67a8a93c0e04c5f093f3476f850e0f0df3c35316153da11f291dbe0555f8769, 798191 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2019/207924s001lbl.pdf: sha256 5b6c5bbaf771c9e8858c80fdebf7d6e05312d495d9da5d67cf0f30e6ff09123a, 931469 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2020/207924s002lbl.pdf: sha256 784ea9b311eed9214a6105130eabd05803036061b41ed03d5268be9f0c472152, 985195 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2022/207924Orig1s006lbl.pdf: sha256 78ce9553c664c7d4146e2fa925106abb5b4f01008186f6a0e96b54a7d1678caf, 1576189 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2021/207924s004lbl.pdf: sha256 187f667d7cb6bda0164c0a6d6efae090d56ac81380d2b872187ca337ace78d5f, 1391034 bytes

## label-history-xeljanz

- https://www.accessdata.fda.gov/drugsatfda_docs/label/2012/203214s000lbl.pdf: sha256 59aea7c7eccf6f966b91337f0bcda23d4d009cb2e355b07063e9a7fe18630f71, 422010 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2020/203214s026lbl.pdf: sha256 b1e8f609fddea782ec5bcd20f94c85f25d03179280aab3b3053116275483d8bc, 3979102 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2019/203214s024,208246s010lbl.pdf: sha256 2ffd4e3216d23facd58accb458377a7087c1a2f6555e5c3cfb4a85f5e5c08ade, 4319076 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2026/203214s042lbl.pdf: sha256 21290f871e22e5c897aed537266bd0d82500b50627d8fba53e4156216092de35, 2155944 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2014/203214s005lbl.pdf: sha256 3ed4f57352211acea384161274414fd9ffd6b938e3d1cfe9ab5669429e67ecdd, 1149253 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2015/203214s011lbl.pdf: sha256 f8011ff27b609a51af3e66205b000c68388b8d5b5cb370c48bb6fe44cb6608d0, 1436521 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2021/203214s031lbl.pdf: sha256 7d31333c76b4ddb48d4df52976d73ebdd2d7462136e52ea0753e18afb17bed87, 2756417 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2018/203214s018lbl.pdf: sha256 a11e9a3ab169bd23671960fab5b19b088ee06634a66ebfc5d325a8b1f1ee06db, 1601374 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2024/203214s034,208246s021,213082s006lbl.pdf: sha256 7b30ed9fbe543bacdd899172ae477965d8aa0b3d6d62054a6d0addc4ba8ac4da, 2463447 bytes

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
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2020/761077s007lbl.pdf: sha256 118baaad102e2baf52571785beb979aa0e528c177777fb436836f6a6dce523d8, 3632627 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2021/761077s011lbl.pdf: sha256 1d0570961d17344358936df8eaaf0232fc18cd72f209934dc50e4e9cf7552478, 4069837 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2022/761077Orig1s017Correctedlbl.pdf: sha256 ba48cc8433c307e2a8c25911cc728d3fd4ad3f90d831593fe9ed90710a210953, 20307813 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2018/761077s000lbl.pdf: sha256 67bfd3328fb0f101d51d965e53edb86aa38f7c0834b3e902ef9026b2e3feec40, 2197393 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2024/761077s022lbl.pdf: sha256 0761b624184d1cfee32f4ea1e527db66e52def353755b3ccb5f472f6825c48e1, 2035693 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2025/761077s026lbl.pdf: sha256 3f23eab4c43f449dabff6dac0060c936c1a805f95d5ad51969c64c3e684a141d, 3498011 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2019/761077s003lbl.pdf: sha256 ad4d5bc93229275769997c3acf79212b88a28f7095ec86c733e94523523d9132, 3105200 bytes

## label-history-livalo

- https://www.accessdata.fda.gov/drugsatfda_docs/label/2013/022363s012lbl.pdf: sha256 bd5679e5ea930fa576effc20b5985bb90ab201d06b287eb12cdc5a0d4f915f82, 993867 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2012/022363s010lbl.pdf: sha256 a25586bc35670417df44a7574a14c0cc56c843f1e9d7f050ec5564705c15b590, 941154 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2019/022363s017lbl.pdf: sha256 a80d41f0f3aba8ccf29558b1905ae568adb8cdb8b1583783b3c6c8ed074cdfcf, 1164184 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2016/022363s011lbl.pdf: sha256 5e6c6aa679a5883fab473086c991c092313e77f2731a110c3baf64a16a84ca25, 908792 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2022/022363s020lbl.pdf: sha256 b2f89318d4fca0a231e8f00e8519dc8d3e95e08d329c53ee0def815b312f6e06, 1376410 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2020/022363s018lbl.pdf: sha256 44ce5616665019bc1b4b753ebc08873c22d2ce4527ec0ddae458ea79500edd33, 1194957 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2024/022363s022lbl.pdf: sha256 1da833f39f3aaccb8cb634d89d1a4b40eb3ed071e4fff09fb50dadf9f9872f0a, 1360947 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2009/022363s000lbl.pdf: sha256 6a9dc8a9b1a0d3fb7afbdd0979d6c374aa12cf1c5429f8fa3db348462032fdd1, 534677 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2011/022363s006lbl.pdf: sha256 ada1d02ae5586c9cfb51780056aec761877cf169b74e5597c03355f90681c465, 813981 bytes

## label-history-never

- https://www.accessdata.fda.gov/drugsatfda_docs/label/2012/022363s010lbl.pdf: sha256 a25586bc35670417df44a7574a14c0cc56c843f1e9d7f050ec5564705c15b590, 941154 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2019/022363s017lbl.pdf: sha256 a80d41f0f3aba8ccf29558b1905ae568adb8cdb8b1583783b3c6c8ed074cdfcf, 1164184 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2012/022363s008s009lbl.pdf: sha256 5867e75f2cf52e7fcc43b4fc7fbc0f436a8aa66917fa0ce210719da54a3ecad1, 776888 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2019/022363s015lbl.pdf: sha256 6152a7a0521504d6180ebb2e0675d62c12b0e267f77e9c87cd64aa13c5d0684d, 1241461 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2011/022363s005lbl.pdf: sha256 dc709ceb16711e4dbcd054e751dd3881e411192892c235f00cfabf4f19fbed33, 819034 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2016/022363s011lbl.pdf: sha256 5e6c6aa679a5883fab473086c991c092313e77f2731a110c3baf64a16a84ca25, 908792 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2020/022363s018lbl.pdf: sha256 44ce5616665019bc1b4b753ebc08873c22d2ce4527ec0ddae458ea79500edd33, 1194957 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2024/022363s022lbl.pdf: sha256 1da833f39f3aaccb8cb634d89d1a4b40eb3ed071e4fff09fb50dadf9f9872f0a, 1360947 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2009/022363s000lbl.pdf: sha256 6a9dc8a9b1a0d3fb7afbdd0979d6c374aa12cf1c5429f8fa3db348462032fdd1, 534677 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2011/022363s006lbl.pdf: sha256 ada1d02ae5586c9cfb51780056aec761877cf169b74e5597c03355f90681c465, 813981 bytes

## label-history-example

The help example (ticket 1083), recorded in pm job j43461 on 2026-11-20.

- https://www.accessdata.fda.gov/drugsatfda_docs/label/2020/208065s019lbl.pdf: sha256 ea1676aebab319898ffd0dbe8603edaf4b0dcf976d7ffff2ae4327853bfb719b, 1513278 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2016/208065s003lbl.pdf: sha256 d182038da8452199b0903d32d6d5ccfafa1f2278c0b89e51a9de08386d966e71, 490842 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2018/208065s011lbl.pdf: sha256 a305887fff45f7730b1c3338091e96f1d86cd7825432d47de3ee083c2c49e463, 895568 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2017/208065s007lbl.pdf: sha256 f96307aad730146cd2d14fa6672c148f0beeaedee4ba1ed5a6c98cc0453e9eb6, 679308 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2019/208065s013lbl.pdf: sha256 42288aafb9ac7317c03a568d89e854b7366f567b965951142c21dffb09fb25ec, 1222959 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2018/208065s008lbl.pdf: sha256 5e5aa00435d87510c86b720276f98609ff0335bb76ad0ed7cc9b8bd935a09fcf, 1102797 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2015/208065s000lbl.pdf: sha256 93589e8f9e985bca670f3f7a63173b002ecde773a532df18ef38f7bd4cae58e1, 397071 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2020/208065s016lbl.pdf: sha256 80e628eee7d2564d3093cb506865a1214026436099d4806a66a0dc3dc69a6eee, 1388276 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2020/208065s018lbl.pdf: sha256 a941c02b0c20c2c7ec3b4f9b0cdbb37fd6e3b4e16a62dc790500c0304052127e, 1219865 bytes
- https://www.accessdata.fda.gov/drugsatfda_docs/label/2026/208065s035s036lbl.pdf: sha256 9642c55d3088ce9821fa41b9abc3725e3ea9f42530ee51beb859cd29a6274a83, 1519577 bytes
