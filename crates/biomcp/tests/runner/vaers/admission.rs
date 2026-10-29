use super::{Check, both, environment, words};
use crate::runner::network_stub::Stub;
use serde_json::json;
pub(super) fn run() -> Check {
    for fault in [
        "oversized",
        "row-width",
        "vaccine",
        "group",
        "scope",
        "html",
    ] {
        let stub = Stub::for_sources(
            &["wonder.cdc.gov"],
            |_| "application/xml",
            move |_| {
                let body = super::fixture::table("MMR", "D8.V13-level2");
                Ok(match fault {
                    "oversized" => {
                        body.replace("Synthetic zero-row notice.", &"x".repeat(2 * 1024 * 1024))
                    }
                    "scope" => body.replacen("<value code=\"00\"/>", "<value code=\"FR\"/>", 1),
                    "row-width" => body.replacen("<c v=\"80.00%\"/>", "", 1),
                    "vaccine" => body.replacen("code=\"MMR\"", "code=\"COVID19\"", 1),
                    "group" => body.replacen("code=\"D8.V13-level2\"", "code=\"D8.V10\"", 1),
                    _ => "<html><body>Synthetic access denied</body></html>".into(),
                })
            },
        )?;
        let env = environment(&stub)?;
        for _ in 0..2 {
            let value = both(&env, words("MMR"), 4)?;
            if value.pointer("/sources/0/reason") != Some(&json!("bad-reply"))
                || value.get("result").is_some()
            {
                return Err(format!("invalid native {fault} became an answer"));
            }
        }
        if stub.finish()?.len() != 4 {
            return Err(format!("invalid {fault} reply entered cache"));
        }
    }
    Ok(())
}
