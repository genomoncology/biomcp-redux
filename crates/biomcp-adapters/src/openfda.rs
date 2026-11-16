//! Checked openFDA page envelopes with source-specific record admission.
use biodata::PartialDate;
use biomcp_fetch::{BadReply, Reply};
use serde::{Deserialize, de::DeserializeOwned};
/// openFDA's terms for its device, recall and report services, which grant no rights the
/// publisher's third-party exceptions withhold.
pub(crate) const TERMS: biomcp_fetch::Terms = biomcp_fetch::Terms {
    name: "openFDA terms and license; third-party exceptions",
    url: "https://open.fda.gov/terms/",
    usage: biomcp_types::Use::Open,
};
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum Envelope<T, M = Meta> {
    Page { meta: M, results: Vec<T> },
    Absent { error: Absence },
}
/// Page metadata of a search reply, which reports its native window and total.
#[derive(Debug, Deserialize)]
pub(crate) struct Meta {
    pub(crate) last_updated: Option<PartialDate>,
    pub(crate) results: Counts,
}
/// Page metadata of a `count=` reply, which reports no window or total.
#[derive(Debug, Deserialize)]
pub(crate) struct CountMeta {
    pub(crate) last_updated: Option<PartialDate>,
}
/// What every openFDA page's metadata says about the publisher date and the admitted rows.
pub(crate) trait PageMeta: DeserializeOwned {
    fn last_updated(&self) -> Option<PartialDate>;
    fn admits(&self, len: usize) -> bool;
}
impl PageMeta for Meta {
    fn last_updated(&self) -> Option<PartialDate> {
        self.last_updated
    }
    fn admits(&self, len: usize) -> bool {
        self.results.limit > 0
            && len as u64
                == self
                    .results
                    .total
                    .saturating_sub(self.results.skip.into())
                    .min(self.results.limit.into())
    }
}
impl PageMeta for CountMeta {
    fn last_updated(&self) -> Option<PartialDate> {
        self.last_updated
    }
    fn admits(&self, _: usize) -> bool {
        true
    }
}
#[derive(Debug, Deserialize)]
pub(crate) struct Counts {
    pub(crate) skip: u32,
    pub(crate) limit: u32,
    pub(crate) total: u64,
}
#[derive(Debug, Deserialize)]
pub(crate) struct Absence {
    code: String,
    message: String,
    #[serde(flatten)]
    extra: std::collections::BTreeMap<String, serde::de::IgnoredAny>,
}
impl Absence {
    fn valid(&self) -> bool {
        self.extra.is_empty() && self.code == "NOT_FOUND" && self.message == "No matches found!"
    }
}
/// One row of an openFDA page; its own check runs on every row of a valid page.
pub(crate) trait Item<M: PageMeta = Meta>: DeserializeOwned {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
    fn counts(meta: &M, len: usize) -> bool {
        meta.admits(len)
    }
}
/// The one owner of openFDA page, absence and status checks.
impl<T: Item<M>, M: PageMeta> Reply for Envelope<T, M> {
    fn data_date(&self) -> Option<String> {
        match self {
            Self::Page { meta, .. } => meta.last_updated().map(|date| date.to_string()),
            Self::Absent { .. } => None,
        }
    }
    fn check(&self) -> Result<(), BadReply> {
        match self {
            Self::Page { meta, results } if <T as Item<M>>::counts(meta, results.len()) => {
                results.iter().try_for_each(<T as Item<M>>::check)
            }
            Self::Absent { error } if error.valid() => Ok(()),
            _ => Err(BadReply),
        }
    }
    fn check_status(&self, status: u16) -> Result<(), BadReply> {
        if matches!(
            (self, status),
            (Self::Page { .. }, 200) | (Self::Absent { .. }, 404)
        ) {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}

/// Recover the native total only when a nonzero page is reported absent.
/// The probe keeps the selector and sort of the requested page and omits `skip`,
/// as every adapter's first page may.
pub(crate) fn page<T: Item>(
    source: biomcp_types::Source,
    leg: &biomcp_fetch::Leg<'_>,
    endpoint: &biomcp_fetch::Endpoint,
    args: &[(&str, biomcp_fetch::Arg<'_>)],
    offset: u32,
    limit: u32,
    check: &dyn Fn(&Envelope<T>) -> Result<(), BadReply>,
) -> Result<biomcp_fetch::Received<Envelope<T>>, biomcp_fetch::RequestError> {
    use biomcp_fetch::{Arg, Input, Read};
    let mut received = leg.call(endpoint, Input::Params(args), Read::json().check(check))?;
    if offset == 0 || !matches!(received.value, Envelope::Absent { .. }) {
        return Ok(received);
    }
    let mut probe = Vec::new();
    for (name, value) in args {
        if *name == "skip" {
            continue;
        }
        let value = match (*name, value) {
            ("limit", _) => Arg::Num(1),
            (_, Arg::Text(value)) => Arg::Text(value),
            (_, Arg::Num(value)) => Arg::Num(*value),
            (_, Arg::Ids(value)) => Arg::Ids(value),
            (_, Arg::Contact(value)) => Arg::Contact(value),
        };
        probe.push((*name, value));
    }
    let first = leg.call(
        endpoint,
        Input::Params(&probe),
        Read::<Envelope<T>>::json().check(&|reply| match reply {
            Envelope::Page { meta, .. } if meta.results.skip != 0 || meta.results.limit != 1 => {
                Err(BadReply)
            }
            _ => Ok(()),
        }),
    )?;
    if let Envelope::Page { mut meta, .. } = first.value {
        if u64::from(offset) < meta.results.total {
            return Err(biomcp_fetch::bad_reply(source, Some(404)));
        }
        meta.results.skip = offset;
        meta.results.limit = limit;
        received.value = Envelope::Page {
            meta,
            results: Vec::new(),
        };
    }
    Ok(received)
}
