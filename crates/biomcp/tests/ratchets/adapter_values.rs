//! Classify optional-value drops beside the existing source ratchet.
use super::super::{Check, rust_files};
use super::tokens;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
#[derive(Deserialize)]
struct Audit {
    sites: Vec<Site>,
}
#[derive(Deserialize)]
struct Site {
    path: String,
    site: String,
    maximum: usize,
    class: String,
    owner: String,
    reason: String,
}
pub(super) fn check(root: &Path) -> Check {
    let audit = audit()?;
    validate(&audit)?;
    for (relative, site, count) in sites(root)? {
        if !audit
            .sites
            .iter()
            .any(|entry| entry.path == relative && entry.site == site && count <= entry.maximum)
        {
            return Err(format!(
                "{relative}: classify new optional-value drop or file rule: {site}"
            )
            .into());
        }
    }
    Ok(())
}
fn validate(audit: &Audit) -> Check {
    let mut seen = BTreeSet::new();
    for entry in &audit.sites {
        let adapter = entry
            .path
            .strip_prefix("crates/biomcp-adapters/src/")
            .and_then(|path| path.split('/').next())
            .unwrap_or("");
        if !seen.insert((&entry.path, &entry.site))
            || entry.maximum == 0
            || entry.owner != adapter
            || entry.reason.is_empty()
            || !matches!(
                entry.class.as_str(),
                "migrated" | "identity_refusal" | "legitimate_absence" | "pending_audit"
            )
            || !initial_entry(entry)
        {
            return Err(format!(
                "{}: new or invalid pending entry: {}",
                entry.path, entry.site
            )
            .into());
        }
    }
    Ok(())
}
fn initial_entry(entry: &Site) -> bool {
    let identity = format!("{}\n{}", entry.path, entry.site);
    let hash: String = Sha256::digest(identity.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    INITIAL_IDENTITIES
        .lines()
        .filter_map(|line| line.split_once(' '))
        .any(|(allowed, maximum)| {
            allowed == hash
                && maximum
                    .parse::<usize>()
                    .is_ok_and(|maximum| entry.maximum <= maximum)
        })
}
fn audit() -> Result<Audit, Box<dyn std::error::Error>> {
    Ok(serde_json::from_str(include_str!(
        "adapter_value_audit.json"
    ))?)
}
pub(super) fn pending(
    path: &str,
    site: &str,
    count: usize,
) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(audit()?
        .sites
        .iter()
        .any(|entry| entry.path == path && entry.site == site && count <= entry.maximum))
}
type DropSite = (String, String, usize);
pub(crate) fn sites(root: &Path) -> Result<Vec<DropSite>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();
    for path in rust_files(&root.join("crates/biomcp-adapters/src"))? {
        let relative = path
            .strip_prefix(root)?
            .to_string_lossy()
            .replace('\\', "/");
        let text = fs::read_to_string(&path)?;
        let mut found = fingerprints(&tokens(&text), &relative);
        if text.lines().count() > 300 {
            found.insert("file line limit".into(), text.lines().count());
        }
        for (site, count) in found {
            result.push((relative.clone(), site, count));
        }
    }
    Ok(result)
}

fn fingerprints(tokens: &[String], path: &str) -> BTreeMap<String, usize> {
    let mut result = BTreeMap::new();
    let mut function = "module";
    for (index, token) in tokens.iter().enumerate() {
        if token == "fn"
            && let Some(name) = tokens.get(index + 1)
        {
            function = name;
        }
        if let Some(site) = attribute_site(tokens, index) {
            *result.entry(site).or_default() += 1;
        }
        let tail = &tokens[index..];
        let ok = tail.starts_with(&[".".into(), "ok".into(), "(".into(), ")".into()]);
        let pattern = matches!(token.as_str(), "if" | "let")
            && (tail.starts_with(&["if".into(), "let".into(), "Ok".into()])
                || token == "let"
                    && index
                        .checked_sub(1)
                        .and_then(|i| tokens.get(i))
                        .is_none_or(|t| t != "if")
                    && tail.get(1).is_some_and(|t| t == "Ok"));
        // Treat defaults throughout raw/map files conservatively, and also catch
        // parse receivers (including a result assigned earlier in the function).
        let start = tokens[..index].iter().rposition(|t| t == "fn").unwrap_or(0);
        let default = tail.starts_with(&[
            ".".into(),
            "unwrap_or_default".into(),
            "(".into(),
            ")".into(),
        ]) && (path.ends_with("/raw.rs")
            || path.ends_with("/map.rs")
            || tokens[start..index]
                .iter()
                .any(|t| matches!(t.as_str(), "parse" | "from_str" | "from_slice")));
        if ok || default || pattern {
            let end = if pattern {
                (index + 16).min(tokens.len())
            } else {
                index + 4
            };
            let context = tokens[index.saturating_sub(12)..end].join(" ");
            *result.entry(format!("{function}: {context}")).or_default() += 1;
        }
    }
    result
}
pub(super) fn attribute_site(tokens: &[String], index: usize) -> Option<String> {
    if tokens.get(index)? != "#" {
        return None;
    }
    let tail = tokens.get(index + 1..)?;
    let end = tail.iter().position(|t| t == "]")?;
    let attribute = &tail[..end];
    if !attribute.iter().any(|t| t == "serde")
        || !attribute
            .iter()
            .any(|t| t == "default" || t == "deny_unknown_fields")
    {
        return None;
    }
    Some(format!(
        "adapter serde refusal: {}",
        tail[..(end + 13).min(tail.len())].join(" ")
    ))
}

// Initial path/fingerprint/count identities form a fixed ceiling. The active
// inventory may remove entries, lower counts or reclassify without changing it.
const INITIAL_IDENTITIES: &str = "
97e5f9cb91d80acaed67860a709a49f8c5821a601c06d95ed78bcfae53a4f161 1
be0b4d8fc0f58d640968c8aef47e9bacb8c0fda4af9ca3754040be1510755c1e 1
29fc484f0c9a3b6dfcb5bcc73a62b4afd10e657a12b16242665999805400e93e 1
9b205fd9854813c773852daf0044d54389be54ba54cee5082a4dcd4c4199fbba 1
a0102f3a61d98dc39650968df1be0617451ad09ed235ed67e0f0585ad36fa3c9 1
4f6e620e9fcff4f36a84b26a1410d616f0dc9a9e016f4b913add4b74c3e4390f 1
2c52b7539e182340481d696fe96550220f3e2e39b6b8f7a9058df0024399462d 1
c37838fd8843039174ec3698b4664cfbb31f395194d23857a4f13e4eb46358d2 1
87e5da8904ae0a733e2c0fb081123faa4758581983d8995c07c89f3fd8fdd300 1
f025d0e0e25f435170a3b8db6bc2f0def2f00c00a2b2773753e2af1370e0070d 1
2a3dd137a348b990ab5c37e3b92ff15b9578bc6f03e757fd32b50e102bda1337 1
caa99b536b5e029afd4d456fc2bdc398ebaa61cbcb7dd243e8b74d40d787a2a1 1
1ca6f5a78a1c4451f04659b938c5d84744c85bfa10cc55964d17dbac61d4e0ae 1
c80839e3ebb4feae4e4a78e2297bf1e5bc8b701676f1901485e3e24a9e3c8290 1
057f2f37e19bae8ea5053e02dbb8c94b5cab9ee5b81018cdd4fc1797e751ee8c 1
83d14b9253b020d65c66587cd9e28f2d2f5af7c8166d49f41a851af1fa9f35a3 1
cd1ebed4a9a4ba0c64082d919f75b8c158efa9bb292f5f4987ec6d04180dc3bd 1
e2b101394d688d4943f743bcc40cecbdbe83e548315adf247c63745b1bec84ed 1
09b59d56f99b04a908313134a0a43de47b934c4ad7e583451f494dc0d3fa2eff 1
4fcb27bb8e89e2093f510f29b45a70e74b8a250d7798cf59d37783e94b346c68 1
a9fa210ec61a74f5a6fb500db908a99ea661c7f5ebb51f57a6ce20c6eb080b2f 1
dc2e978408db7e2129c40a6fe997dc67a916b52254c9b020e49a2486f4aa5d5d 1
687d90cac660c8d4c4fc1a215fe72f9909b2e251a06c27f494810b455e6def86 1
0a08d0d24bf3c83870d99756cb6af93421e44910cd31a2d970aa4ed26adf8077 1
d71ea4bc06f65a6a7f32e071f4fc6e49dd779c2b8ffcab95be4be9018b905a36 1
d9506ef737996d24a571952581a7e1c1244fd0cc97cc7a8c59ac53b83365b7cf 1
5390fa3b1e4bd0fb66c7649da09570055fc12c7ce027569c17ce8a5de6d4c4ff 1
8f7c38c18fa159f69728ce41cd2a82a47c966b213e64307040575a18233650d2 1
b71f1dc72dca6e98260f69792d2d51e5f5162b3cbdfb510fba7172308ec57c71 1
ebcc0aeb23c331de442caa866605086281726675664cdca2cce376ef70fc66d0 1
425c6bfc3a99bd44efeafd87fdbb53ae903c53da21313cf7a404372520cd270f 1
683dd4411cbcc4b8e54600896ff91355b856560e0d2b2c1d4baadbf3a340e3ae 1
4fdc7ed0b1171598ef9a1d51d59d765c102490301a8317d291835055b94f45d0 1
106659233c315836bdff75ed33bb10e4b1b2f168905f505283386eb609018f53 1
e581879508b35931a574c3d8f70a6cc845cf22c9649dc4096a5151aacab07414 1
405f0316d853b446849c9330ae3b82e3c0162bb48a983b1ed45c61f80afea85f 1
ac1090dec4a58a34011db46d5d323d3921ce488e2d0b140654e4eee89ea6cedd 1
7b6bb2fbc32d8b580114ea77a123cde2ce67a9a1ff77a1edd1328791c4257604 1
e8955adbaaf59c3e01d953c6d3cf0088a58905d9d72d1f38219245e26f7853f3 394
09dff744c3a076fd41b82dd3490a7276f48b9b9f084f4f17e1ff9837b85bd6b8 1
b32af9605163d50a272073a4424f2f7ffacbe80df4af224816090993129c935c 1
18d446e5f5a7b0cfb542d3a71a3a3b0e64ce57d942e3e7c00daf3d90ed1b33aa 1
058652aaf59b02c262ff90227d1d3348e73773a7d2ecc3135ba268b0730fa718 1
37163ad7a883e3f41c9947ed8126798852b7b26b1af9740e3cf8bd683d70c261 1
ea889bdc9888bdb78a856cd4d9da477e6111eedb44318fda6ea4441b9f69b50c 1
f95f38e64563b4088c51482a60a4ffa245961819dceaa30828b0af74e5e0fb03 1
6852d02ca1147378fcabc7e3c9e7b19186653bf627f84b295e05c831ff4d7b0a 1
7f12449359671729e7ade550de96556f0a85d1456ce204746360dc995fc97e5f 1
ca2cc80339d71511bac66e4295bfa0e2ddc4c7eaacd9cd1c304b927ea12efd1e 1
dd949a9ddf22d37d28bbbc43972c8552649afb7caaeeb3ad9f4c798e2a450838 1
43cfc09dba6ed3879aae0352dc411be0501bce9d32790b652e8842ad6b9caebb 1
99c0139d1526ce1f0143a7c19a5f1c3040c0be5a1ca553a1301980d1519d4afa 1
ec0fc66d935a0ab59843805b45e60008c0a5963719d3312aaa39af98bcec41c2 1
18dd3886cdf3a134252263d12890708205b75e7f8653fe601ab29ac382b261de 1
49e99010c547af6c0041c37c0ceab306f344f250f9dbf43d165920bc587b53fd 1
bcf14d394b1a792a201a9917636f11c6a891558db7b65c6aaa90f804c3e54b43 1
ce734dc4d4981a781ec1614a6d6728a7808dcf2ee83bfce2972efc38556de65e 1
0d688fcbce145a672e5d0df68505bb7c1cf0a6e5a560900411d8e5cd9909d10a 1
9e2406a179534a66e1a4fc705873016ebac6ee557c4c77290f39d49f575237ab 1
8ffcbac6a4fab38509daf2f5f2eaa696eb0e77fab84faae8f77f12e064afa60a 1
ab587751bea1505d00d8deda3faab89655a22a425e02ed080a04ed884d3ceecc 2
bbe46d18034015c5695ead2a5cd5847760d7532d780894b77b9fcc203a63897b 1
3307f6afd13de88c66c110c7ff1f9dc7e98e1b851e80800eb05dac89049367c7 1
381d543098608f630d00359b5065bac17faa82a98c00ba0be44dc7791e92b24e 1
8a72e350b27dd3a612a8da585bb9649de269f0f534f097cddf738adda3cd8a3b 1
c1a3da2b42e03c34177e6bdc4eeb6833f801cf42abbf35aa9033e4cf328d6980 1
f780f0f7c3bf46c52d6940a346c9dd564ece80801fac160ebf541967e6a349bf 1
48676d54ca12a0743d5f8cbede388591a4b81a196ee1008b11a81107e575ab76 1
6cbb25c4c562bd33eb1bfa5ea6d896d31a73bc5ed95f3a66728b91aea57385f4 1
7ce8b1c44d987c2e2c4a20ae9284e3af31efbddf8181d3d9f564e083f1500ae9 1
b27e825c5bc2c0a5d00b6a4cf7b7cfc65feb0affb18ab7a9610d77d5e4cab608 1
1060758a28c7179d5d85c6380d7adf6ac30269e9ba252f814b5e39e4ebca7b43 1
60713465761dc6486e79e5689e10c530efd2404cac6c0b27af99bbb213c32e22 1
0f335e1905fd9d29d22ee6f8dbd9e5b857e178558ac7ada640fda3f334d41df1 1
15a80efdb1e7eccef1170e071d7d895d5d97c7b90721757deca8c8ac819850b1 1
6857623f6914816a8ca6566925acbcd525766a6aeb0bd26811adb22a503583c1 1
21b2554f4c68d02d5aa5c8c693726e74741df02dfad1492f5bb5d58cde231508 1
2164c26d5b0f927c0d5acd5e52eb82ad1ec77a3d511798c41c137fff7538a22c 1
5960c2cbf8ca55b50d53e8b44791feef804baaf0f901b5d9b9e3c5c0eeab3a8d 1
1a4b60b96f353272f4d3b48e5cb39934d38681289454449735040139cf4a9788 1
26ae1bb4499d9fccc7f0080da80783861709fa6aa127ae60b8929e69fe0330a9 1
c50ba2fb84ba8edcf6092eb69be37d25e627219bc83d9b43cfa09ab6d4f66b2c 1
487a39103cab890f26f340a08292327e5dcd1d3f339a469961c1fdffcfd425c0 1
003f1ae339a496c90c642edbd313644420ba4e3df38b8edfa3a705ac6aaf766b 1
85cc67c87def65d883c01f7ad825ed1212241c4558ad3f0f72f071ff416aceb9 1
110976556c61f3866f03bb485678188e2c4834923e212c54483964d8c24e4fa1 1
ea1a69fcf04799889e419ba01e21c68d57c9f6588d6f87b8b0a54d590c2245d9 1
5cb9e20b0046618f4c1d969f906470d56e7931fa1c5bdc336b60304cbb4bff9c 1
2669bf8e563e14367a1edf7504db3622db9f6cb5a124af7120ac40731f88d2c8 1
6952bfa8457277092613c6c712adab3bd52681942b9698da8d825d567f389c27 1
e23d01a16345442e2dc6329f398d850b0356fde934a88e0937544d8559404299 1
4913e4e33f04605e4a5b50bbefe20604314842af2b18dd6d027489de83ad9099 1
81c3fd4bd17772d35b537208a480035de85bf56439e4b43a3255eacbb527c1a3 1
90e0260938b4ee7a9c5c53f06c4bf25634bf413438956b4a1e59e4117c0609a8 1
abb3c8cfa5eee43bd35ed6d843c20414f138f942ce6d15c5bcfa76959187fa70 1
acdebca9df7f2bf964381b2478dbda9d0730b40582d08c7983e347ed1ab00c25 1
4f8f43488bf2334c5e2f22d4006295ce38b5f71cd236cf952dfe995ada8c5cc2 1
e888a26de0d95bc340852d890b79ab888063e869652111e86e7606a4dc79fc2f 1
dec3a430ef047627114c25e29541d940e9970eb61ff33fe030d9e27b0b430168 1
dca1425c829f1ef2847b58c078e98957fdcf7d48719302aa81bf8d1cbf10f8e5 1
d90f5654fab0f529ede535574c737c5c337b13b912c20a39c54cff493fb541c2 1
72bdaead12bf24e93619454cfd9d632720a204ce616002b918fdb4feeec1c6bc 1
2bfff1b05b3d7630cd776a86041c8aa4e416bce976986821bef5d0f5c1d569b6 1
979cf93d1f7b2e17368920fac7273c63357f988732e4093760d93156c298d485 1
fdda4088e46a352baca1cd4da99a8bd292ab64f11c3a2a1bcf61fa1fe2240991 1
005d14c4c621e4bb09e42fc5c5c1416e9bb7ba389156e8417dee25dc153c8864 1
b1489b5038beaec71d86dab985de461e887a20bc32bbbf5b57b026c65b6709ae 1
b6d5170180e9f69f1146d64725901c15c9ecd5f61316a819ecbc2381bca0f298 1
e266a1a60dfda25c4d04421ee9f23948a4830cc0bfa1b3cfe338d36f578b6bdb 1
e6f65d451e6088953fbd88f3113977094bbbb9ec627d290523390d5d43a5c8ab 1
0ce99ad3eda7f7a12bca889ed4ffc92153cf070fa14335bec7f5ccbb968e380f 1
3b66df6ccfd9e309cb929ae9852aa7827a6e704f517f57450133742637f88bc2 1
12ec68adb3c8fb57d13c7c9dc382f85aae0ac7a6d3a35163be000d2da001a46e 1
a66d46eea8f885166ef3171aa71b2647c518651b1eef763ee44adf5f9fa2c765 1
2759d4886009f35811e4e6fdafe6318d122001d40dec7d03fc12f2eb5cc3c531 1
d3e838723d0eaade1f4082b986f3e308457c75fb8574c75823bdd32fda7390ac 1
85fa9a6ec8b50b2ad1a324c7016b6b72bf1d9249481ac5d9864048f69b7558a7 1
14b127f58d78396332679d87241037eb57e7814568ec60e362490639bdeb4984 1
abc0e7d11d56113d2c354b0fa2c26417805d2ee531f0d29d1549de80b1f5f01d 1
b7ef9f44fdb6d44d5e296a9510180950223bcedb229e022918da636f252c7945 1
dde726fc86b5c3cc025951116276f212252e91cd6f38a6305952a07c02728f8c 1
2b102f86898a5a9b8e73c133b9dc05f2c24803f36cbb9ad7e07d8b4400bfbee6 1
29176c569f50c53847a70b5329ffbd0869f2c50f797bf3ed4c4719144226e19f 1
998779100b6bb1e900da48e9a3da5fee1abb8fb9d0c7da531282b8c2b0cabd41 1
5bae4ed0bf197047f9be12160683b876b8f93e8a5751b28903b71d5436ed0d1e 2
b4f02d862aaab0b4925aa68f8a8adbba618eb955179ed82dd877765c93abd021 1
f811ad569b2a87b8729b7261bb4373616b2dd4ad5cba310339c9d93f5e9fd1e2 1
fdb2805100962c1b2c921120a2cbcc53ca0dbfdb2bff8c23c09e006e2477cea0 1
f0ba4796e8aefbc7e58bc8750fefcdb93a3f3896f7104a5fced0c7c9d6badf68 1
5c95e9203f8f16358d0182d936b68953f82eb12ab243ac7018aa05ac33916435 1
9d4b26d6ce526967ba45467f44130092c05ad55d0dca9cb960b45b711a96f179 1
673cd2dcc5a2590995d4171b5aabef923cf8e1749c409a891f8e6a050ebf0b21 1
b23ff846b92697cac0ca7cf63887e09e88e7cd6a58144ceec40a87167a261218 1
25caf006df2d55fb9df1c5e99ab7c120c68efae7800e4dd5e2c1dd9697033d6c 1
e6c9186fbfa4dd2a6ac6f6376984d1147122153d953aa71dd514275970368e22 1
1e528b1326c2ec27ce8371455bd37701cd37d2abc57d810356261e51e1db8484 1
901763391dcbfb2f1e4c6c58a86bfebac3fd91cc32f56b497122803d279dc4f6 1
1c8853ff841ebeb5d2dbc15ea8e446e9818b96b70f626c5088beef8902033e7e 1
f511aebfec01f3fe95b3b0bcb89ded093c341626a1fb3eb59c239984d5ceae31 1
2c87dfe9b1e0145fa267e1555e54df2e483873f48ccfcf8915f238d44de68b0f 1
bb8ab703328cb6d1c61315c5c6b3fa2b4b140e026b9e117877e4beeb01280f12 1
30f968daceb43e854f9ad4f21cf34680f87cda2938c9b1c58f62c411caf9f56f 1
4fa9368e5739b16aa2bc0ff0de5510cda10bab970a641aac3af433e5ef2afebe 1
08850b4f03eef90db05e4233447a1df473a31b439d07409c11cc00c59fa4b203 1
3984f307328e9af5bbcb32a0021e2874068a27facd480864eb977f5ae7805ca4 1
5be7de0d10adf236765125b88971c1026a32238a669374caaf82a9f65c29f5fb 1
1d8f13b3946511a9376a8bfd7bd9c55e2a7e0c94dc58fda24a945b9eb2306638 1
b1078883f115ba90e671a02f2b916f0c0ce262fec7c1c4d7a8256ceba1c915f9 1
c6e187f6d2f60844d409a1b7c443fb2b52cab9234a9aacf467d882ef8da3ce6d 1
6bf3df635fd9acd4396f77d7881d838ffab36616382cb89bbca5a2fcec39cfd9 1
932126453fd572e4c6decc321d22dcb1a7c5bb6254334ec4a0e363092bd6b0a5 1
b7b134518c60130a56be81aeb417c78f3343bfd0485826f5a8e4767da546cf3e 1
8eb4a3ae24904d97e59d79dc0fd75ef78299a054af177a4752f3c4827edfdf95 1
f749518b28038486f9d9a67e7a6d45562cbb459176971fefff6c2664b944b673 1
eaa0e8be1c9c0c60732a9f63392b275b5ee4258bc550aa5228a35b7490532b16 1
19fd452d63f90daafaaf76fb7c4edf29c2c2a4190110676fb3296b2f1cfa0be5 1
dd440a0a58fd464afa7d329e37898dafcc442abe69556c8e5f627f82eab6707a 1
91a04272e3ead301493fdba86d7da501abf7f2dbbd8d40318696eb2877bdbe33 1
f08d7d2a12656beeaa5c2d31901e912132aa731f1535b4232d94e4cd6c2a83d8 1
cfa5179473569e88cb0274dce8aea09caf8442d48df176284e6c1dc4f9260346 1
db649f47d80dd8dd791f12c9cbf7d2c2c5c6b85a10d6200ab4acfdfbad3d8bb1 1
4413a608a15b607356d1ec9437176a30cee9f549de40ba3661a4ef6f407c0762 1
f23d2de18bea89589e62672cb4230c4e6e0cfc4b3ee493e3a8d23e95ac86c289 1
245e50e3accad6345c8e6006e0a0a160eaeae547cab68322d15b4b10b26b3ad8 1
8bc99dc48a78854c3096a0ee64982cfd1a3ab3a254cd00e653b931ee6418f57a 1
6b063f110e3fec762067030e6958e54f19fffdefb6c736b103c272f5041b68fe 1
a613e5a7975a71df8eb87eebf4f04679ecf2b0833884e967697019a4bb030283 1
9d9df54ecf30cbcda627e2c1c96adb1ec192f14dcb31dba1b7fd9e8369d2dd86 1
14f01553ade3ab6da7ddddc0d3cb2a970ba9d833e9bf647bf5b361ba227f54cc 1
7957a46e9638019a473d015e836b820d82ca96cd70c50d4c182c2c6865479cce 1
7833a7a0b4333971c0abea4e53361d5ffc7e73fbd798160d5d04e335ce61aee2 1
05667a905304e5cb37b86ff9c3f4e5519df2f45e704adf33bb9d7796d4e50593 1
74bf4673ba3de48aed6ffe9917c1cd4514ab500bd8f75711ea80452dba30c326 1
60476830d90fd5aa556228c3c56c89a81edcd835ea8ade4fd579a86123119bc5 1
aa495275b8cdb58e6be642157bc3cff922405ff3ffbcedf59c03bbef28629b17 1
4ed9097b1ecdef07a26ca33c81ca85a4e085003df37d335c5d1603ca5dc1e5df 1
64585ea386c9845a7d76b636bd0ff767d610163fef2e98b781e6041fcf7fb15d 1
428c733d62151cf44d03629ca6ad5e367fe3dfc1db8a13243d1f556c67ba9d30 1
bb0946302c4cbaf8b282999752ef7f7a167d6db21eeb67f84b4178897f268184 1
3be39751b376649f4966e67bf42fa22d5306bf5a34f11a5c1ae6c36b43968cae 1
4db05a7ac72b0b49f4b081665e11404f714af47a05ad13833c6b2a86b27501d1 1
bdc27bb7a3797144063b18676ae37b0ff8ff8513836866b1f7b1dfbc7c657403 1
a9442a059f39044b55973eeca2aeffefb519a625041d3d14e5197a8e696836e7 1
3917f1008a941ff5f0a832592a12a8bc3896fc6670de958630ee2aacf5604200 1
ba7f570718268f57e22c7e65fff96cf7814e17a668303e741ad7e2c885f697e5 1
f7abd565f1198a192be2c645316bf0945206a0066fab0713ab1b7c5ebe1dbe3c 1
5f6cb61ee07e441962332268131b5721c7b5da7cc474ef6890d77ef269ba8398 1
196c9eef56e5f3de3b5bdcc88014e880222658099241b0a37b8107ae4ad33516 1
909ce8a384982519a7d537bfcf24abf1b26946eb4b74cf1429f6d153d26a523b 1
0702997118e0540972120abbfcb2b284141b2dd1b1a81051a0bd52bd2b935a14 1
a77482564141aaa588f5913c900d387dfd30a630ffa3a7ceff6d5aeab9a40fdb 1
74f35f9b97dd9512bbffcc23b86dc3b9a81300314ca183d9c97a006616774b07 1
99f9a50e1a4b79cf91dc4dedd19ad395f4fd2918f9d176b615d1e0414a2bd51a 1
eaa3397ad2daa14e8abd1fcaba12856d69ede5de4c5157732072e3b0ddfeefa6 1
8981f9f9dc1006ae94d2afd236b8167d26d34196dcfebdf1afb60c5b3479128a 1
39d9c802b4e5ea48d8d841bce18be0f6c2edff109163b3000b967bfbcc8dfa08 1
b394b350917851d64516f4611cbb2122f9548789c5b921bf8a3362bd532506f4 1
a0141697bfb630c02f4eb174d102bc2f583aed74a6035da340a92ecd5bc8f50e 1
e2e55751f0f82ce974cfffd2517d8695f19434d57f88ba5d766ecb8eb4ce84d0 452
d2851f6d0df1fb58a953574a82ab2bd06501ca92fe2e27570e89e5af8887c662 1
063d90c8c563a59e7f14b4f66f84263fe9da46d4f575056b9c9d8e07d65696f6 1
0abbda7664d255ec289709a55f6e41451d8f8e614ae5237cd58fd20c885f2292 1
b833bacf08b88394968b039eb682bdf1139c2a488955fce45993a60424a68037 1
2f74be3124411cb563fc77cf456576c7dbc66507e0fd1f9bd24ab93f7b0dedba 1
14c7ee86f78cefdd927b45c739ed60c11295ea1536194e2fb142bce67318ec20 1
5c71a26f89a7e5179c46686352c8908700efe8b69c6ea6c607523c4a714a7c9f 1
db0dec5e1b864e3d132af6b21d48ac7b82e16854613e824b793982d0d3f30a55 1
327104f52df595f74dece0f483c85cf1aa9ca147c5bdea4dcb7a30b7bdc1805f 1
b83eeca76d791df2f6039352dd8f528bb2abe9f8960549c7596f0dc3c839e72e 1
3fcac04ff11144a28202db934635e835806d947c824316cb07f86425e34c26e8 1
c3f347708634ea173abbadea71a0acd9ec5ea0cf9f66acf8ca57a4b42ced2f06 1
973fd3dd7bcb8f34cef379fe11513bcd5730b75056c48828c7b196d8a6e32c9c 1
3a1b4482643a994d4324dfd09e582b0eb76d1199f5e15a83870b4e50ef7a22eb 1
ca8632fff508bc3acc14eea1acfbf2a62d7696df8e587d89a8104c989cd10ed7 1
c9d6f3448997520d659a34621d4ed8b5b4872409e9dbc23a7a15b4928d8a4585 1
da183914ac4e30f18975fcc74aebb35efef534ae461880a498739e1602e10400 1
d8b1da5d55cba6ebf781bd2ea7efe2f42eb8a06a7d15e2d7a88d884bd9b23f6e 2
794cf89b498f530dba3ec88c6cfe799b3031b7965d41189239bef60244b6a0f8 1
d9df47b42c8330ba9bc3ac7beacb85a73a5ed199fa534bb37e601663e96c52e8 2
682ca2605a89189ee72bdceb1fc5f5d77bb62f13c79d08d771f0041fc152d3d8 1
5ace3be0841df5b8a064f3e1659301469a0353dbc3bc3eb1865f446cedb91280 1
8b55fbc2bceb3a3c79b095d1f86a7e3c6762e36891e856a5260ec7a2efa5a2a2 1
d6fad045d8b59a88bdcf62c14132391fdd5a420fa14bbc17b3d6f6b2ab28baef 1
4cd34fde1c40b1dd4a38b07533f611e0b5555b274a43ff23cf30543feae0d8ff 1
22c9d1f8a1a10cce3392899a125bec67d241c3313f72cd8ebadccf1dee718b0a 1
6116d934e0ce705e184cf0bf270dd784c16b1eb7adf9b5d5aa7d9adf173a9dce 1
5ebf69156aa32e373e982b255c6239367c2a453d28958ea85bddf3bfdda804c2 1
e9055a7763a15a6954091a3a5fe8e585eb9631df707d8be133085e1f4464bcda 1
8edcb30c0f2a48b16deb4249920dd06421d7617c851e4acbf574aa6df4c8060f 1
01fdb9818131dce5613aff4505095745364b8f8711792d07e7fa3a20d9c33926 1
18ea56a9636b90d7053b1bcc49218bf27fbf8b9813ce82954ddb7ed6c86d9717 1
06b98d0c254160438d753adc1b4b43d5f7aa9e97a5ec989a7ce445ee0f4301f2 1
7095f74c7af928ee0cd32fee31f85f98f097960d21cde6ac263e99809a94a25b 1
5679c10a867cab0f9f54e46c3a72384f4767ec30cd1da7295667ea6eccd1b9d3 2
06f4edca4d67018788d0011f73ccc2f058da1c07433ef9d46de07d1044b1a190 1
24e0be044b88799c647fb8ca2f4fb1cc71e3bcfd6d32c7e3ba380cd6c4cd2aa7 1
819b118a5ba1501ba338e0fe0b9b9733af2489dc2af3aaa69b605b2ea8e6ee12 1
";

#[test]
fn adapter_drop_forms_and_file_rules_require_pending_identity() -> Check {
    let root = tempfile::tempdir()?;
    let directory = root.path().join("crates/biomcp-adapters/src/example");
    fs::create_dir_all(&directory)?;
    let path = directory.join("details.rs");
    for source in [
        "fn value() { let native = input.parse().ok(); }",
        "fn value() { if let Ok(native) = input.parse() { use_value(native); } }",
        "fn value() { let Ok(native) = input.parse() else { return; }; }",
        "fn value() { input.filter_map(|v| v.parse().ok()); }",
        "fn value() { input.parse::<u64>().unwrap_or_default(); }",
        "fn value() { let parsed = input.parse::<u64>(); parsed.unwrap_or_default(); }",
        "#[serde(default)] struct Raw;",
        "#[serde(deny_unknown_fields)] struct Raw;",
    ] {
        fs::write(&path, source)?;
        let failure = check(root.path())
            .expect_err("planted adapter violation passed")
            .to_string();
        assert!(failure.contains("example/details.rs"), "{failure}");
    }
    fs::write(&path, "// line\n".repeat(301))?;
    assert!(
        check(root.path())
            .expect_err("301-line adapter passed")
            .to_string()
            .contains("file line limit")
    );
    fs::write(&path, "// line\n".repeat(300))?;
    check(root.path())?;
    fs::write(
        &path,
        "fn value() { /* .ok() */ let note = \"if let Ok(v) .ok()\"; }",
    )?;
    check(root.path())
}

#[test]
fn inventory_allows_shrinking_and_reclassification_but_refuses_new_identities() -> Check {
    let mut current = audit()?;
    current.sites.retain(|entry| entry.maximum > 1);
    for entry in &mut current.sites {
        entry.maximum = 1;
        entry.class = "migrated".into();
    }
    validate(&current)?;
    current.sites[0].site.push_str(" new pending");
    assert!(
        validate(&current).is_err(),
        "replacement at same count passed"
    );
    let mut current = audit()?;
    current.sites[0].maximum += 1;
    assert!(validate(&current).is_err(), "increased ceiling passed");
    let mut current = audit()?;
    current.sites[0].owner = "other_adapter".into();
    assert!(validate(&current).is_err(), "wrong adapter owner passed");
    let mut current = audit()?;
    let duplicate = current.sites.remove(0);
    let copy: Site = serde_json::from_value(serde_json::json!({
        "path": duplicate.path, "site": duplicate.site, "maximum": duplicate.maximum,
        "class": duplicate.class, "owner": duplicate.owner, "reason": duplicate.reason
    }))?;
    current.sites.push(copy);
    current.sites.push(duplicate);
    assert!(
        validate(&current).is_err(),
        "duplicate pending entry passed"
    );
    Ok(())
}
