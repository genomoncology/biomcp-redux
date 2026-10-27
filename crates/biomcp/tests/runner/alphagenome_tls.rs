//! Feature variant-alphagenome: the CLI preserves native groups through a local TLS gRPC peer.
use super::{alphagenome::invoke, launch::Environment};
use libtest_mimic::Trial;
use serde_json::{Value, json};
#[path = "../../../biomcp-fetch/tests/grpc_worker/support.rs"]
#[allow(
    dead_code,
    reason = "One shared TLS fixture serves transport and CLI contracts"
)]
mod support;
static QUOTA: biomcp_fetch::Quota = biomcp_fetch::Quota::new(
    biomcp_fetch::SourceKey::declare("alphagenome-cli-synthetic"),
    biomcp_fetch::Rate::per_second(1000),
    None,
    1,
);
const SECTION: &str = "/result/candidates/0/sections/alphagenome";
const KEY: &str = "synthetic-alphagenome-cli-placeholder";
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "variant/alphagenome-native-groups-local-tls",
        || run().map_err(Into::into),
    )]
}
fn run() -> Result<(), String> {
    for status in [200, 401, 429, 500, 999] {
        let messages = if status == 999 {
            vec![vec![10, 255]]
        } else {
            messages()
        };
        let fixture = fixture(messages, if status == 999 { 200 } else { status })?;
        let env = environment(&fixture)?;
        let args = ["get", "variant", "CA123643", "alphagenome"].map(str::to_owned);
        let reply = invoke(&env, &args, 0)?;
        if status == 200 {
            groups(&reply)?;
            let args = args
                .into_iter()
                .chain(["--section-offset".into(), "3".into()])
                .collect::<Vec<_>>();
            let later = invoke(&env, &args, 0)?;
            if later.pointer(&format!("{SECTION}/data/scorers/0/rows/0/tensor_index"))
                != Some(&json!([0, 1, 1]))
                || later.pointer(&format!("{SECTION}/data/scorers/1/page/returned"))
                    != Some(&json!(0))
            {
                return Err("scorers did not page independently in native order".into());
            }
        } else {
            let reason = match status {
                401 => "rejected",
                429 => "rate_limited",
                500 => "unavailable",
                _ => "bad_reply",
            };
            if reply.pointer(&format!("{SECTION}/state")) != Some(&json!("unavailable"))
                || reply.pointer(&format!("{SECTION}/reason")) != Some(&json!(reason))
                || reply.pointer(&format!("{SECTION}/data")).is_some()
            {
                return Err(format!(
                    "source failure {status} exposed data or lost its reason"
                ));
            }
        }
        let mut resolved = false;
        let mut requests = 0;
        while let Ok(event) = fixture.events.try_recv() {
            match event {
                support::Seen::Http(request) => {
                    if !request.starts_with("GET /allele/CA123643?") {
                        return Err("unexpected resolver request".into());
                    }
                    resolved = true;
                }
                support::Seen::Request(headers, body) => {
                    if !resolved
                        || headers.get("x-goog-api-key").and_then(|v| v.to_str().ok()) != Some(KEY)
                        || body.get(5..) != Some(expected_request().as_slice())
                    {
                        return Err(
                            "scoring preceded resolution or changed keyed request mapping".into(),
                        );
                    }
                    requests += 1;
                }
                _ => {}
            }
        }
        if requests != if status == 200 { 2 } else { 1 } {
            return Err("scoring request retried or was omitted".into());
        }
        fixture.finish().map_err(|e| e.to_string())?;
    }
    Ok(())
}
fn groups(reply: &Value) -> Result<(), String> {
    let rows = reply
        .pointer(&format!("{SECTION}/data/scorers"))
        .and_then(Value::as_array)
        .ok_or("missing scorer groups")?;
    if rows.len() != 3 {
        return Err("native scorer groups were collapsed".into());
    }
    for (ordinal, (group, shape)) in rows
        .iter()
        .zip([[2, 2, 2], [2, 1, 1], [1, 1, 2]])
        .enumerate()
    {
        if group.get("shape") != Some(&json!(shape))
            || group.get("request_ordinal") != Some(&json!(ordinal))
            || group.get("data_type") != Some(&json!("DATA_TYPE_FLOAT32"))
            || group.get("units").is_some()
            || group.get("model_version").is_some()
        {
            return Err("group provenance or native scalar representation changed".into());
        }
    }
    if rows.first().and_then(|g| g.pointer("/rows/0/value")) != Some(&json!(0.25))
        || rows.first().and_then(|g| g.pointer("/rows/2/tensor_index")) != Some(&json!([0, 1, 0]))
        || rows
            .get(2)
            .and_then(|g| g.pointer("/rows/0/gene"))
            .is_some()
        || rows.first().and_then(|g| g.get("next")).is_none()
        || rows.get(1).and_then(|g| g.get("next")).is_some()
    {
        return Err(
            "values, coordinates, nongene metadata or independent next pages changed".into(),
        );
    }
    Ok(())
}
fn fixture(messages: Vec<Vec<u8>>, status: u16) -> Result<support::Fixture, String> {
    let allele = json!({"@id":"http://reg.genome.network/allele/CA123643","genomicAlleles":[{
        "referenceGenome":"GRCh38","hgvs":["NC_000007.14:g.140753336A>T"],
        "coordinates":[{"start":140753335,"end":140753336,"referenceAllele":"A","allele":"T"}]}]});
    support::Fixture::with_http(
        support::Hold::None,
        0,
        true,
        false,
        messages,
        Some((
            vec![
                "gdmscience.googleapis.com".into(),
                "reg.genome.network".into(),
            ],
            allele.to_string().into_bytes(),
        )),
        status,
    )
    .map_err(|e| e.to_string())
}
fn environment(fixture: &support::Fixture) -> Result<Environment, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ca.pem");
    std::fs::write(&ca, &fixture.certificate).map_err(|e| e.to_string())?;
    env.config(
        &json!({"proxy":format!("http://{}",fixture.address), "ca_bundle":ca,
        "keys":{"alphagenome_api_key":KEY}}),
    )
    .map_err(|e| e.to_string())?;
    Ok(env)
}
fn messages() -> Vec<Vec<u8>> {
    [[2, 2, 2], [2, 1, 1], [1, 1, 2]]
        .into_iter()
        .enumerate()
        .flat_map(|(ordinal, shape)| {
            let tensor = [field(1, &shape), number(2, 2), number(4, 1)].concat();
            let mut metadata = field(1, &variant());
            for track in 0..shape[2] {
                metadata.extend(field(
                    2,
                    &field(1, format!("synthetic track {track}").as_bytes()),
                ));
            }
            if ordinal < 2 {
                for row in 0..shape[1] {
                    metadata.extend(field(
                        3,
                        &field(1, format!("synthetic-gene-{row}").as_bytes()),
                    ));
                }
            }
            let descriptor = field(
                1,
                &field(1, &[field(1, &tensor), field(2, &metadata)].concat()),
            );
            let data = (0..usize::from(shape[0]) * usize::from(shape[1]) * usize::from(shape[2]))
                .flat_map(|index| (index as f32 + 0.25).to_le_bytes())
                .collect::<Vec<_>>();
            [descriptor, field(2, &field(1, &data))]
        })
        .collect()
}
fn expected_request() -> Vec<u8> {
    [
        field(
            1,
            &[
                field(1, b"chr7"),
                number(2, 140491191),
                number(3, 141015479),
                number(4, 3),
            ]
            .concat(),
        ),
        field(2, &variant()),
        number(3, 9606),
        field(4, &field(2, &number(1, 4))),
        field(4, &field(3, &number(2, 7))),
        field(
            4,
            &field(1, &[number(1, 501), number(2, 1), number(3, 3)].concat()),
        ),
    ]
    .concat()
}
fn variant() -> Vec<u8> {
    [
        field(1, b"chr7"),
        number(2, 140753336),
        field(3, b"A"),
        field(4, b"T"),
    ]
    .concat()
}
fn number(tag: u8, mut value: u64) -> Vec<u8> {
    let mut bytes = vec![tag << 3];
    while value > 127 {
        bytes.push((value as u8 & 127) | 128);
        value >>= 7;
    }
    bytes.push(value as u8);
    bytes
}
fn field(tag: u8, data: &[u8]) -> Vec<u8> {
    let mut bytes = number(tag, data.len() as u64);
    if let Some(first) = bytes.first_mut() {
        *first |= 2;
    }
    bytes.extend(data);
    bytes
}
