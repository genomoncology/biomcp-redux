//! Public resolution contracts not covered by the shipped command cases.
#![expect(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "Fixture setup must fail on invalid checked inputs"
)]
use biodata::{GeneInput, GeneResolution, HgncStatus, LocusGroup, MatchKind};
use biomcp_adapters::hgnc;
use biomcp_fetch::{Fetch, FetchConfig, Mode};
use biomcp_query::gene;
use std::{path::PathBuf, time::Duration};
fn captured() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/replies/hgnc")
}
fn replay(dir: PathBuf) -> Fetch {
    Fetch::open(FetchConfig {
        mode: Mode::Replay { dir },
        ..Default::default()
    })
    .unwrap()
}
fn resolve(fetch: &Fetch, text: &str) -> GeneResolution {
    let leg = fetch.leg(&hgnc::SPEC, fetch.deadline(Duration::from_secs(120)));
    let mapped = gene::resolve(&leg, &GeneInput::new(text).unwrap()).unwrap();
    assert!(mapped.problems.is_empty(), "{text}: {:?}", mapped.problems);
    mapped.value
}
#[test]
fn captured_ambiguity_orders_every_distinct_identity_without_selection() {
    let fetch = replay(captured());
    let GeneResolution::Ambiguous { candidates } = resolve(&fetch, "100874204") else {
        panic!("expected ambiguous Entrez ID");
    };
    let got: Vec<_> = candidates
        .iter()
        .map(|v| (v.hgnc_id.get(), v.r#match.as_ref().unwrap().kind))
        .collect();
    assert_eq!(
        got,
        [(40244, MatchKind::EntrezId), (55559, MatchKind::EntrezId)]
    );
}
#[test]
fn captured_withdrawn_identity_preserves_all_stated_successors() {
    let fetch = replay(captured());
    for (text, id, status, targets) in [
        ("A2MR", 9, HgncStatus::MergedSplit, vec![6692]),
        ("HGNC:9", 9, HgncStatus::MergedSplit, vec![6692]),
        ("CBBM", 1531, HgncStatus::MergedSplit, vec![4206, 9936]),
        ("ADCR", 231, HgncStatus::MergedSplit, vec![12797]),
    ] {
        let GeneResolution::Withdrawn {
            hgnc_id,
            hgnc_status,
            merged_into,
            ..
        } = resolve(&fetch, text)
        else {
            panic!("expected withdrawn {text}")
        };
        assert_eq!(hgnc_id.get(), id);
        assert_eq!(hgnc_status, status);
        assert_eq!(
            merged_into.iter().map(|id| id.get()).collect::<Vec<_>>(),
            targets
        );
    }
    assert!(
        matches!(resolve(&fetch,"AK6"),GeneResolution::Resolved{gene,..} if gene.symbol.as_str()=="AK6")
    );
    for text in ["999999999", "ENSG99999999999"] {
        assert!(matches!(resolve(&fetch, text), GeneResolution::NotFound {}));
    }
}
#[test]
fn captured_gene_fields_keep_source_precision_and_optional_absence() {
    let fetch = replay(captured());
    let GeneResolution::Resolved { gene, .. } = resolve(&fetch, "MIR21") else {
        panic!("MIR21")
    };
    assert_eq!(gene.locus_group, LocusGroup::NonCodingRna);
    assert!(gene.uniprot.is_empty());
    let GeneResolution::Resolved { gene, .. } = resolve(&fetch, "DMD") else {
        panic!("DMD")
    };
    assert!(gene.location.is_none());
}
const COMPLETE: &str = "0887a33cc809657a2a564d65e8cbfcddf45adc3810a8bfb0653e03b418cf2953.json";
#[expect(
    clippy::disallowed_methods,
    reason = "Mutate captured public text only in an isolated synthetic fixture"
)]
fn synthetic(mut change: impl FnMut(&mut Vec<String>, &[&str])) -> tempfile::TempDir {
    let mut recording: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(captured().join("hgnc").join(COMPLETE)).unwrap(),
    )
    .unwrap();
    let body = recording["exchanges"][0]["body"]["value"].as_str().unwrap();
    let mut lines = body.lines();
    let header = lines.next().unwrap();
    let fields: Vec<_> = header.split('\t').collect();
    let mut edited = vec![header.to_owned()];
    for line in lines {
        let mut cells: Vec<String> = line.split('\t').map(str::to_owned).collect();
        change(&mut cells, &fields);
        edited.push(cells.join("\t"));
    }
    recording["exchanges"][0]["body"]["value"] = format!("{}\n", edited.join("\n")).into();
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir(dir.path().join("hgnc")).unwrap();
    std::fs::write(
        dir.path().join("hgnc").join(COMPLETE),
        serde_json::to_vec(&recording).unwrap(),
    )
    .unwrap();
    dir
}
#[test]
fn synthetic_multiple_index_matches_keep_one_best_identity() {
    let dir = synthetic(|cells, fields| {
        let symbol = fields.iter().position(|f| *f == "symbol").unwrap();
        if cells[symbol] == "KMT2D" {
            let alias = fields.iter().position(|f| *f == "alias_symbol").unwrap();
            cells[alias] = "MLL2".into();
        }
        // Make a third record share the alias to exercise locus-group ordering.
        if cells[symbol] == "H3P10" {
            let alias = fields.iter().position(|f| *f == "alias_symbol").unwrap();
            cells[alias] = "MLL2".into();
        }
    });
    let fetch = replay(dir.path().into());
    let GeneResolution::Ambiguous { candidates } = resolve(&fetch, "MLL2") else {
        panic!("MLL2")
    };
    assert_eq!(
        candidates
            .iter()
            .map(|v| v.hgnc_id.get())
            .collect::<Vec<_>>(),
        [7133, 15840, 43802]
    );
    assert_eq!(
        candidates[0].r#match.as_ref().unwrap().kind,
        MatchKind::PreviousSymbol
    );
}

#[test]
// Expire only the case-owned documented cache metadata.
fn stale_identity_and_annotation_sources_make_only_their_answers_partial() {
    use biodata::{GeneGet, GeneSearch, QueryBody, Request, Source, SourceState, Status};
    use biomcp_fetch::ReplayCache;
    use biomcp_query::{Ctx, run};
    let directory = tempfile::tempdir().unwrap();
    let case_path = directory.path().join("case");
    let cache = ReplayCache::create_case(&case_path).unwrap();
    let query = Request::GetGene(GeneGet::new(vec!["BRAF".parse().unwrap()].into()).unwrap());
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/replies/gene/main"),
        },
        replay_cache: Some(cache.clone()),
        ..Default::default()
    })
    .unwrap();
    assert_eq!(
        run(
            &Ctx {
        ncbi_email:None,
                fetch: &fetch,
                budget: Duration::from_secs(20),
                fresh: false
            },
            &query
        )
        .status,
        Status::Complete
    );
    drop(fetch);
    let database = rusqlite::Connection::open(case_path.join("replay.sqlite")).unwrap();
    database
        .execute("UPDATE bundle SET fresh_until=0", [])
        .unwrap();
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Offline,
        replay_cache: Some(cache),
        ..Default::default()
    })
    .unwrap();
    let cx = Ctx {
        ncbi_email:None,
        fetch: &fetch,
        budget: Duration::from_secs(20),
        fresh: false,
    };
    let outcome = run(&cx, &query);
    assert_eq!(outcome.status, Status::Partial);
    assert_eq!(outcome.sources[0].source, Source::Hgnc);
    assert_eq!(outcome.sources[0].state, SourceState::Stale);
    assert_eq!(outcome.sources[1].state, SourceState::Ok);
    let QueryBody::GeneCards(cards) = outcome.body else {
        panic!("gene cards");
    };
    assert_eq!(cards.items[0].status, Status::Partial);
    let search =
        Request::SearchGene(GeneSearch::new(None, Some("9p21.3".parse().unwrap()), 10, 0).unwrap());
    assert_eq!(run(&cx, &search).status, Status::Partial);
    database
        .execute("UPDATE bundle SET fresh_until=9223372036854775807", [])
        .unwrap();
    database
        .execute("UPDATE reply SET fresh_until=0", [])
        .unwrap();
    let outcome = run(&cx, &query);
    assert_eq!(outcome.status, Status::Partial);
    assert_eq!(outcome.sources[0].state, SourceState::Ok);
    assert_eq!(outcome.sources[1].source, Source::MyGene);
    assert_eq!(outcome.sources[1].state, SourceState::Stale);
}

#[test]
#[expect(
    clippy::disallowed_methods,
    reason = "Mutate isolated public recordings"
)]
fn clingen_local_failure_survives_later_source_deadline() {
    use biodata::{GeneGet, GeneSectionWord, Request};
    use biomcp_fetch::RequestError;
    use biomcp_query::{Ctx, run};
    let directory = tempfile::tempdir().unwrap();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/replies/gene/clingen");
    for source in ["hgnc", "clingen"] {
        std::fs::create_dir(directory.path().join(source)).unwrap();
        for entry in std::fs::read_dir(root.join(source)).unwrap() {
            let path = entry.unwrap().path();
            let mut record: serde_json::Value =
                serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
            if record["url"].as_str().unwrap().contains("dosage") {
                continue;
            }
            record["exchanges"][0]["delay_ms"] = 0.into();
            // Remove annotation IDs so this isolates the ClinGen local/source boundary.
            if source == "hgnc" {
                let body = record["exchanges"][0]["body"]["value"].as_str().unwrap();
                let mut lines = body.lines();
                let header = lines.next().unwrap();
                if let Some(index) = header.split('\t').position(|v| v == "entrez_id") {
                    let mut output = format!("{header}\n");
                    for line in lines {
                        let mut cells: Vec<_> = line.split('\t').collect();
                        cells[index] = "";
                        output.push_str(&cells.join("\t"));
                        output.push('\n');
                    }
                    record["exchanges"][0]["body"]["value"] = output.into();
                }
            }
            std::fs::write(
                directory
                    .path()
                    .join(source)
                    .join(path.file_name().unwrap()),
                serde_json::to_vec(&record).unwrap(),
            )
            .unwrap();
        }
    }
    let fetch = replay(directory.path().to_owned());
    let query = Request::GetGene(
        GeneGet::with_sections(
            vec!["TP53".parse().unwrap(), "MLH1".parse().unwrap()].into(),
            vec![GeneSectionWord::ClinGen].into(),
        )
        .unwrap(),
    );
    let outcome = run(
        &Ctx {
        ncbi_email:None,
            fetch: &fetch,
            budget: Duration::from_millis(1500),
            fresh: false,
        },
        &query,
    );
    assert_eq!(
        outcome
            .sources
            .iter()
            .find(|source| source.source == biodata::Source::ClinGen)
            .unwrap()
            .reason,
        Some(biodata::Reason::Syncing)
    );
    assert!(
        matches!(outcome.local, Some(RequestError::Recording(_))),
        "a later source failure must retain the earlier local error: {:?}",
        outcome.local
    );
}
