//! Compare the public pathway-gene query with the publisher's pinned human GMT.
use super::{GET, SEARCH, recording};
use biodata::HgncId;
use biomcp_fetch::{Fetch, FetchConfig, Mode, ReplayCache};
use biomcp_query::{Ctx, DRUG_NAME_LIMIT, FIT_DEADLINE, LABEL_BYTES};
use biomcp_types::{HELD_PAGE_MAX, Paged, PathwayGenesQuery, ReplyData, Request, Status};
use serde_json::{Value, json};
use std::{
    collections::{BTreeSet, HashMap},
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

type Error = Box<dyn std::error::Error>;
const HGNC_URL: &str =
    "https://storage.googleapis.com/public-download-files/hgnc/tsv/tsv/hgnc_complete_set.txt";

pub(super) fn run() -> Result<(), Error> {
    let fixture = super::super::launch::fixtures().join("fixtures/wikipathways-20261110");
    let dir = tempfile::tempdir()?;
    let hgnc = fs::read_to_string(fixture.join("hgnc.tsv"))?;
    let pathways = stage(dir.path(), &fixture, &hgnc)?;
    let gmt = fs::read_to_string(fixture.join("gmt.tsv"))?;
    let mut expected: HashMap<_, BTreeSet<_>> = gmt
        .lines()
        .filter_map(|line| line.split_once('\t'))
        .map(|(id, genes)| (id, genes.split('\t').collect()))
        .collect();
    if expected.len() != 991 {
        return Err("incomplete WikiPathways GMT rows".into());
    }
    let entrez: HashMap<HgncId, &str> = hgnc
        .lines()
        .skip(1)
        .filter_map(|line| {
            let mut fields = line.split('\t');
            Some((fields.next()?.parse().ok()?, fields.nth(5)?))
        })
        .collect();
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: dir.path().to_owned(),
        },
        replay_cache: Some(ReplayCache::temporary_with_owners(&[
            biomcp_query::store::OWNER,
        ])?),
        cache_owners: vec![biomcp_query::store::OWNER],
        ..Default::default()
    })
    .map_err(|error| format!("open replay: {error:?}"))?;
    biomcp_adapters::wikipathways::sync(&fetch)
        .map_err(|error| format!("sync WikiPathways census catalog: {error:?}"))?;
    let context = Ctx {
        fetch: &fetch,
        budget: Duration::from_secs(20),
        fresh: false,
        ncbi_email: None,
        drug_name_limit: DRUG_NAME_LIMIT,
        label_bytes: LABEL_BYTES,
        fit_deadline: FIT_DEADLINE,
    };
    let (mut hits, mut total) = (0, 0);
    for id in &pathways {
        let found = genes(&context, id)?;
        if let Some(expected) = expected.remove(id.as_str()) {
            total += expected.len();
            hits += found
                .iter()
                .filter_map(|gene| entrez.get(gene))
                .filter(|gene| expected.contains(**gene))
                .count();
        }
    }
    if pathways.len() != 1131 || !expected.is_empty() || total == 0 {
        return Err("incomplete WikiPathways census inputs".into());
    }
    if hits * 100 < total * 98 {
        return Err(
            format!("WikiPathways GMT reach fell below 98%: {hits}/{total} pathway genes").into(),
        );
    }
    Ok(())
}

fn genes(context: &Ctx<'_>, id: &str) -> Result<BTreeSet<HgncId>, Error> {
    let mut query = PathwayGenesQuery::new(id.parse()?, HELD_PAGE_MAX)?;
    let mut genes = BTreeSet::new();
    loop {
        let result = biomcp_query::run(context, &Request::PathwayGenes(query.clone()));
        let status = result.status();
        let ReplyData::PathwayGenes {
            body: Some(body), ..
        } = result.body
        else {
            return Err(format!(
                "{id}: pathway genes failed ({status:?}, {:?})",
                result.sources
            )
            .into());
        };
        let count = body
            .gene_count
            .ok_or_else(|| format!("{id}: missing gene count"))?;
        if count.unmapped > 0 && status != Status::Partial {
            return Err(format!(
                "{id}: {} unmapped gene nodes with status {status:?}",
                count.unmapped
            )
            .into());
        }
        for row in body.items {
            if !genes.insert(row.gene.ok_or_else(|| format!("{id}: missing gene ID"))?) {
                return Err(format!("{id}: repeated gene across pages").into());
            }
        }
        let Some(offset) = body.page.next_offset else {
            if genes.len() as u64 != body.total {
                return Err(format!("{id}: lost genes after offset {}", body.page.offset).into());
            }
            return Ok(genes);
        };
        let pool = body
            .page
            .pool
            .ok_or_else(|| format!("{id}: missing pool"))?;
        query.set_page_offset(offset);
        query = query.with_pool(pool.id);
    }
}

fn stage(root: &Path, fixture: &Path, hgnc: &str) -> Result<BTreeSet<String>, Error> {
    let mut catalog = Vec::new();
    let mut ids = BTreeSet::new();
    let bodies = fs::read_to_string(fixture.join("pathways.jsonl"))?;
    for line in bodies.lines() {
        let (id, gpml, pvjson): (String, String, Value) = serde_json::from_str(line)?;
        let path = format!("https://www.wikipathways.org/wikipathways-assets/pathways/{id}/{id}");
        exchange(
            root,
            "wikipathways",
            &format!("{path}.gpml"),
            "application/octet-stream",
            &gpml,
        )?;
        exchange(
            root,
            "wikipathways",
            &format!("{path}.json"),
            "application/json",
            &pvjson.to_string(),
        )?;
        catalog.push(json!({
            "id": id,
            "url": format!("https://www.wikipathways.org/instance/{id}"),
            "species": "Homo sapiens"
        }));
        if !ids.insert(id) {
            return Err("repeated WikiPathways census pathway".into());
        }
    }
    exchange(
        root,
        "wikipathways",
        GET,
        "application/json",
        &json!({"pathwayInfo": catalog}).to_string(),
    )?;
    for row in &mut catalog {
        let fields = row.as_object_mut().ok_or("catalog row must be an object")?;
        fields.insert("datanodes".into(), json!(""));
        fields.insert("annotations".into(), json!(""));
    }
    exchange(
        root,
        "wikipathways",
        SEARCH,
        "application/json",
        &json!({"pathwayInfo": catalog}).to_string(),
    )?;
    exchange(root, "hgnc", HGNC_URL, "text/tab-separated-values", hgnc)?;
    Ok(ids)
}

fn exchange(root: &Path, source: &str, url: &str, media: &str, body: &str) -> Result<(), Error> {
    let file = recording(root, url);
    let file: PathBuf = root
        .join(source)
        .join(file.file_name().ok_or("recording key missing")?);
    fs::create_dir_all(root.join(source))?;
    let wire = json!({"source":source,"method":"GET","url":url,"exchanges":[
        {"status":200,"content_type":media,"delay_ms":0,"retry_after_ms":null,"body":{"kind":"text","value":body}}
    ]});
    fs::write(file, wire.to_string())?;
    Ok(())
}
