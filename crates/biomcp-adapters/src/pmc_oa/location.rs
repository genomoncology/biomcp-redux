//! Checked bucket location. No source-supplied authority becomes a request target.
use biomcp_fetch::BadReply;
#[derive(Debug)]
pub(super) struct Location {
    pub path: String,
    pub md5: Option<String>,
}
impl Location {
    pub(super) fn checked(url: &str, prefix: &str) -> Result<Self, BadReply> {
        let path = url
            .strip_prefix("s3://pmc-oa-opendata/")
            .or_else(|| url.strip_prefix("https://pmc-oa-opendata.s3.amazonaws.com/"))
            .ok_or(BadReply)?;
        let (path, query) = path
            .split_once('?')
            .map_or((path, None), |(p, q)| (p, Some(q)));
        let file = path.strip_prefix(prefix).ok_or(BadReply)?;
        if file.is_empty()
            || file.contains('/')

            || !path
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"/.-_".contains(&b))
        {
            return Err(BadReply);
        }
        let md5 = query
            .map(|q| {
                let value = q.strip_prefix("md5=").ok_or(BadReply)?;
                if value.len() != 32 || !value.bytes().all(|b| b.is_ascii_hexdigit()) {
                    return Err(BadReply);
                }
                Ok(value.to_owned())
            })
            .transpose()?;
        Ok(Self {
            path: format!("/{path}"),
            md5,
        })
    }
}
pub(super) fn version(key: &str, id: biodata::Pmcid) -> Option<u32> {
    let prefix = format!("{id}.");
    let (v, file) = key.strip_prefix(&prefix)?.split_once('/')?;
    let version: u32 = v.parse().ok()?;
    (version > 0 && v == version.to_string() && file == format!("{id}.{v}.json")).then_some(version)
}
