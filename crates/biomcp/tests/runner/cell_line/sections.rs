//! Feature cell-line-sections: callers retrieve whole native rows independently.
use super::*;
pub(super) fn run() -> Result<(), String> {
    let stub = recorded_stub()?;
    let env = setup(&stub)?;
    for section in ["names", "identity", "xrefs", "variants"] {
        let value = invoke(&env, &["get", "cell-line", "CVCL_2119", section], 0)?;
        path(&value, "/status", json!("complete"))?;
    }
    let requests = stub.finish()?;
    if requests
        .iter()
        .filter(|(_, r)| r.contains("/cell-line/CVCL_2119?"))
        .count()
        != 1
    {
        return Err("independent section pages did not reuse the checked complete card".into());
    }
    Ok(())
}
