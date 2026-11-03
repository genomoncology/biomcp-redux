//! NCBI EFetch sequence parameters: https://www.ncbi.nlm.nih.gov/books/NBK25499/.
//! seq_start and seq_stop are inclusive, one-based; strand=1 is the forward reference.
use super::FETCH;
use biodata::{Build, Chromosome};
use biomcp_fetch::{Arg, BadReply, Bytes, Input, Leg, Read, RequestError};

/// Read at most one reference base, only for a source-qualified literal SNV.
pub(crate) fn reference_agrees(leg: &Leg<'_>, id: &str, build: Build) -> Result<bool, RequestError> {
    let Some((chromosome, expression)) = id.strip_prefix("chr").and_then(|id| id.split_once(":g.")) else {
        return Ok(false);
    };
    let Ok(chromosome) = Chromosome::parse(if chromosome == "M" { "MT" } else { chromosome }) else {
        return Ok(false);
    };
    let length = expression.bytes().take_while(u8::is_ascii_digit).count();
    let Some((position, alleles)) = expression.get(..length).zip(expression.get(length..)) else {
        return Ok(false);
    };
    let (Ok(position), Some((reference, alternate))) = (position.parse::<u64>(), alleles.split_once('>')) else {
        return Ok(false);
    };
    if position == 0 || reference.len() != 1 || alternate.len() != 1
        || !reference.bytes().chain(alternate.bytes()).all(|base| b"ACGT".contains(&base)) {
        return Ok(false);
    }
    let accession = chromosome.accession(build).to_string();
    let params = [
        ("db", Arg::Text("nuccore")), ("id", Arg::Text(&accession)),
        ("rettype", Arg::Text("fasta")), ("retmode", Arg::Text("text")),
        ("strand", Arg::Num(1)), ("seq_start", Arg::Num(position)),
        ("seq_stop", Arg::Num(position)), ("tool", Arg::Text("biomcp")),
    ];
    let reply = leg.call(&FETCH, Input::Params(&params), Read::<Bytes>::bytes()
        .bounded(1024, biomcp_types::Reason::BadReply)
        .check(&|bytes| fasta(&bytes.0, &accession, position).map(|_| ()).ok_or(BadReply)))?;
    Ok(fasta(&reply.value.0, &accession, position) == Some(reference))
}
fn fasta<'a>(bytes: &'a [u8], accession: &str, position: u64) -> Option<&'a str> {
    let text = std::str::from_utf8(bytes).ok()?;
    let mut lines = text.lines();
    let header = lines.next()?.split_whitespace().next()?;
    if header != format!(">{accession}:{position}-{position}") {
        return None;
    }
    let base = lines.next()?;
    if base.len() != 1 || !base.bytes().all(|b| b"ACGT".contains(&b)) || lines.any(|line| !line.is_empty()) {
        return None;
    }
    Some(base)
}
