//! Frozen hash admissions shrink together with the one editable pending list.
use super::Check;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

pub(super) fn read(
    text: &str,
    admitted: &str,
) -> Result<BTreeSet<String>, Box<dyn std::error::Error>> {
    let pending: BTreeSet<_> = text.lines().map(str::to_owned).collect();
    if pending.len() != text.lines().count() || pending.contains("") {
        return Err("required coverage: duplicate or empty pending identity".into());
    }
    let hashes: BTreeSet<_> = pending.iter().map(|item| hash(item)).collect();
    let admitted: BTreeSet<_> = admitted.lines().map(str::to_owned).collect();
    admissions(&hashes, &admitted)?;
    Ok(pending)
}
fn admissions(current: &BTreeSet<String>, admitted: &BTreeSet<String>) -> Check {
    if current != admitted {
        return Err("required coverage: pending identities and frozen admissions must shrink together; new pending identities are forbidden".into());
    }
    Ok(())
}
fn hash(item: &str) -> String {
    Sha256::digest(item.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
pub(super) fn admissions_retire_and_refuse_restoring_a_closed_gap() -> Check {
    let admitted = hash("option search trial --old-gap");
    read("option search trial --old-gap\n", &admitted)?;
    assert!(
        read("", &admitted).is_err(),
        "retired admission must leave with its pending identity"
    );
    read("", "")?;
    assert!(
        read("option search trial --old-gap\n", "").is_err(),
        "a closed gap cannot return"
    );
    assert!(
        read("option search trial --new-gap\n", &admitted).is_err(),
        "new gap cannot use an old admission"
    );
    Ok(())
}

// Generated once from the initial missing identities. Retire hashes with pending entries.
pub(super) const INITIAL_ADMISSIONS: &str = "\
9b571400c0326510feb04dc9d8307f8ed385f599eb5a59c75577d87a4190e7f6
1b141733398dbaad24dc30525d5516eb6453d7cb69630091e9cb16060d636cef
049e52b67d73787967ab7f57f7b6a0245486032b07cf9b4546d9384ed8913b74
f75ec3f3aff58cd3eb34ea145796657c99ccf98083111aeb24fcb922a53481f5
e60560d4397470d95654c472316b74c350910bc6db54bd97a0183ec0cc2bb3a5
97224436329f8e1d9c9498b593cac0da543c1a64d789d8c46826cf3a95e4bb44
d044dcc28bb64896edbd9b52b9ae5b82bf83ae9be63d5f73b1c4296816b4ffe6
65861d1309eecf86ff230f0412973660a1ed1a590cea9045090745abf29283c1
c78f2834708d0ad48fc8fdafa61ad024ce2a06dfa07e10b3bb95e52f6f068aa8
c292e1b8499faeecf762a771e4f9a34bca8873e4c796083ebfa7507f3b1a240b
f18a7736f671a4a9dc3d728450df02a815efb4675b8242417249400a4f56e9e7
4a38051e478702190f0689af35e284a410db38d4a7c5952938a4f32b52b784c5
841b694d5df7aa3512ff22764d5dbfd188a10a92efed2c26c50bced5e80f4748
65b6b9b1a740e166330644c2dbefbaca5290c611c6bae76b90509da59fcdada3
3ea440565181d46fe9d3ccba3bbc5a7df350378773a2d58ffb1ad37ad97a81ee
b3ba52f8e06d971bc8a125e70f0bb47f09ad5f203f2d5be47906b63d82652aad
b5b464c328c63045b1c0195ef9ed1922b8e5b930e0ccab7f6842768d59256434
821723167757dc8b8d466a1e1ee11ae63b57decfe6d939f944542d09972906d0
504712c312d51dd77f53cbf8048f391596ce6038d6233013d305120c3787861a
9bc233f340ff52eb6da603f9c0cbcc85c7fdf74b9175e43d51667bb4f9d4e928
c80d32df94e533e2f3caff6698ce4a267d70904fd427af6a71ac67ad6ca50961
7c177a821ca895c1b250ccc5e2ecf30b2500f39bc80b0325336f90605edc1306
d3e1543955382fe6084248802fb5264ee0a0fdcf94a0c095f1a9c4c6f2f06a89
d9946b4775860a616d8d1bb5a9661fe34961241582f03591f345380edee93210
1d3ee7200f218c7e6fa0bbb87d6a310e3fb7e29d5306e83c7ca5a28456e5eeec
b1df5d8acd81b47891ecd46dc710292bfd9a31224e3b6c7a581045e4c6879720
04f7017851e4c433e73ec044e19846f6b65e9d07b476c789d3188754fa4aba9d
76ce98a30d69ae0fc9acd4f33f2115475575d14d47a21cd1828757fee85bf98e
b27597d3a23c786976762482e7a330656bedea3f7cd444dbdd5d53e3d72a9820
3d5be15d104408c22fde886fdb5ca08dabbe0ff25e358baf2062df09a7f7220d
7648cbd760ae084c9185ae671e829700329081995c78f9bbd7e022e66a858db6
2eccdae00ccf4b6d3d721a4aeec2ab08f47786f65cfe4e53c5c8494b3c2330fb
";
