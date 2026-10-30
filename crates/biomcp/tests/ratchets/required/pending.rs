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
76ce98a30d69ae0fc9acd4f33f2115475575d14d47a21cd1828757fee85bf98e
b27597d3a23c786976762482e7a330656bedea3f7cd444dbdd5d53e3d72a9820
";
