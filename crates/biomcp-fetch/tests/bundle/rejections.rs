use super::*;

#[test]
fn rejected_rows_preserve_siblings_and_survive_revalidation_and_reuse() {
    #[derive(Debug, Deserialize)]
    struct Keyed {
        symbol: Option<String>,
        #[serde(rename = "name")]
        _name: String,
    }
    impl Row for Keyed {
        fn keys(&self) -> Keys {
            Keys {
                exact: self
                    .symbol
                    .iter()
                    .map(|s| (KeyKind::Symbol, s.clone().into()))
                    .collect(),
                words: Vec::new(),
            }
        }
    }
    for format in [RowFormat::Tsv, RowFormat::Csv { banner_lines: 0 }] {
        let server = support::Server::new();
        let bundle = Bundle::<Keyed> {
            spec: server.spec(&QUOTA, KeyUse::None),
            name: "keyed",
            path: "/rows",
            format,
            fresh: Freshness::Days(1),
            row: PhantomData,
        };
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("cache.sqlite");
        let fetch = cached(&path);
        let body =
            "symbol\tname\nGOOD\tvalid\n\tprivate missing identity\nBAD\t\nLAST\tvalid sibling\n";
        let body = if matches!(format, RowFormat::Tsv) {
            body.into()
        } else {
            body.replace('\t', ",")
        };
        let sync = || {
            std::thread::scope(|scope| {
                let call = scope.spawn(|| fetch.sync(&bundle));
                server.next().respond(200, "ETag: \"rows\"\r\n", &body);
                call.join().unwrap().unwrap()
            })
        };
        let info = sync();
        assert_eq!(info.rows, 2);
        assert_eq!(
            info.rejections,
            vec![
                biomcp_fetch::BundleRejection {
                    row: 3,
                    reason: "no usable query key".into()
                },
                biomcp_fetch::BundleRejection {
                    row: 4,
                    reason: "cannot decode row".into()
                },
            ]
        );
        let leg = fetch.leg(bundle.spec, fetch.deadline(Duration::from_secs(2)));
        assert_eq!(
            leg.rows(&bundle, Find::Exact(KeyKind::Symbol, "LAST"))
                .unwrap()
                .len(),
            1
        );
        let (_, reused) = leg
            .stored_rows(&bundle, Find::Exact(KeyKind::Symbol, "GOOD"))
            .unwrap();
        assert_eq!(reused.rejections, info.rejections);
        if matches!(format, RowFormat::Tsv) {
            drop(fetch);
            let reopened = cached(&path);
            let revalidated = std::thread::scope(|scope| {
                let call = scope.spawn(|| reopened.sync(&bundle));
                server.next().respond(304, "", "");
                call.join().unwrap().unwrap()
            });
            assert_eq!(revalidated.rejections, info.rejections);
        } else {
            assert_eq!(sync().rejections, info.rejections);
        }
    }
}
