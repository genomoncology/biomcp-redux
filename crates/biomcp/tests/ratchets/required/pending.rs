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
9abf5e633c9f0208fb8b7c8eaef16bd04c89ed6110a29491cb3b069084d9d36f
e6253c079c23f2afe0c5c3e7e5b854cef489c85edf4e5ca14771ee73a6c5e3ef
b658aeecfe96533d36cd252738b7600745ab641b92843a1b4695f39ea3d1a7af
8248e658facb53787caf6cea948692ee59b46bc3a84a5e72d4b4fc3974a6f49d
a453d9a52c08b65e415618742b7a33c43470068c1117a3d48175cddc611a7033
c7f750422b86f25c412445aebe40f57f903e1e1e3085bc623a3db6456a5d0a0a
68df2c7f3ddb488f839afbc040b44433f054c942d349550b5a4d158ba4897da2
7196aecdf1bf943e7655fb566138b0ed19cc1ca0d7a8ee1a4887a8e51621d0a3
c6d86b706995174c633fff9004521c4bf90c0f39121d09efcd1c90b941cb47e1
bbe9035d0becbaced4e673480cf2b23df56e05d8d7747c9ece004f87617e486b
7fd1575a16d19a52dc8be6926a4f928fa289ae59d883bec468c75d5a9c31d926
9b571400c0326510feb04dc9d8307f8ed385f599eb5a59c75577d87a4190e7f6
c3ea6096f63dd46a56879411eaae3f68596ce127f571f32fd330a56988798ec4
d7b6ea0d19e05d5a529266fa484e44667dbbeab06ea302e21f29317942d7241b
97d2124dd399a83a86e1cf234c1f3a492511ef3dddc6e14399d307015fb34fe5
49eed1a01e8460eb40239c696a07c833d7c009ee04227eba273df05b80299b2f
b2b2f1d07f01d90fa24274d17757cd8ff1d21bac3620fdcdfe5236e2b03090f0
2ab9cd7df465fee69e249c7581221fe88b6ee62e94510bd69a64e78693f19597
544ea17171e54610ef802969ae21f331c10cfa6c54dc97dc25cdb4dc34459889
1b141733398dbaad24dc30525d5516eb6453d7cb69630091e9cb16060d636cef
5d6f0ce35ee561e73425ebecd97f70a7ad7f795c0342af8380c795171d1655b5
33f5581fe4125f807559783300e9b02bcb5fa19f9152ccde7e2a278d93c04453
049e52b67d73787967ab7f57f7b6a0245486032b07cf9b4546d9384ed8913b74
f75ec3f3aff58cd3eb34ea145796657c99ccf98083111aeb24fcb922a53481f5
e60560d4397470d95654c472316b74c350910bc6db54bd97a0183ec0cc2bb3a5
97224436329f8e1d9c9498b593cac0da543c1a64d789d8c46826cf3a95e4bb44
d044dcc28bb64896edbd9b52b9ae5b82bf83ae9be63d5f73b1c4296816b4ffe6
65861d1309eecf86ff230f0412973660a1ed1a590cea9045090745abf29283c1
c78f2834708d0ad48fc8fdafa61ad024ce2a06dfa07e10b3bb95e52f6f068aa8
b5a1ba4f2928fc0294254420c2123dcd016a7ea8dcfd906b19b60ac2595e028a
c292e1b8499faeecf762a771e4f9a34bca8873e4c796083ebfa7507f3b1a240b
f18a7736f671a4a9dc3d728450df02a815efb4675b8242417249400a4f56e9e7
4a38051e478702190f0689af35e284a410db38d4a7c5952938a4f32b52b784c5
841b694d5df7aa3512ff22764d5dbfd188a10a92efed2c26c50bced5e80f4748
65b6b9b1a740e166330644c2dbefbaca5290c611c6bae76b90509da59fcdada3
3ea440565181d46fe9d3ccba3bbc5a7df350378773a2d58ffb1ad37ad97a81ee
e861575b64a491c9f1474cbbe5aefd18c83b6da500300f38984665d2708ef6e3
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
18e5becfba90323bada8d7d28a08514cfbc2843bef682838e3a73a49aca8d621
b1df5d8acd81b47891ecd46dc710292bfd9a31224e3b6c7a581045e4c6879720
2fb1c7fd9d7a8292587c6d32989dba157174101a02a1428b1ff3851fd06d135a
82ea60584add579d3f3dbfb9c2d0c6e36b013e8706df8d3723e23412259b84d5
04f7017851e4c433e73ec044e19846f6b65e9d07b476c789d3188754fa4aba9d
76ce98a30d69ae0fc9acd4f33f2115475575d14d47a21cd1828757fee85bf98e
b27597d3a23c786976762482e7a330656bedea3f7cd444dbdd5d53e3d72a9820
3d5be15d104408c22fde886fdb5ca08dabbe0ff25e358baf2062df09a7f7220d
7648cbd760ae084c9185ae671e829700329081995c78f9bbd7e022e66a858db6
2eccdae00ccf4b6d3d721a4aeec2ab08f47786f65cfe4e53c5c8494b3c2330fb
";
