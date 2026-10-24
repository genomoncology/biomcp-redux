# 0075 — Install the candidate formula through an owned local tap

Status: COMPLETE.
Landed: 95de99e01aa6804811d37ff46bb6ad95b59b050d

Milestone: 1

## Outcome

The isolated Homebrew artifact gate installs its candidate formula through an owned local tap, as current Homebrew requires, without changing the host or using a published package.

- Why: platform job j10296 installed and checked the candidate through the other five channel paths, but Homebrew rejected the direct `artifacts/biomcp.rb` formula path because the formula was outside a tap. No Homebrew installation proof occurred.
- Starts from: the exact refusal and retained candidate artifacts from 0066 job j10296. The existing isolated Homebrew checkout runs successfully; retain its isolation and candidate identity.
- Changes: `release/gate.py` Homebrew setup and narrowly related proof. Put the generated candidate formula into a local owned tap and invoke it by its qualified name. Preserve exact local candidate URL, checksum, executable and fixture selectors. Remove the owned tap/prefix after success or failure; do not install or configure host Homebrew.
- Keeps: wheel, desktop, registry, local installer, download installer and installed HTTP checks already proved. No source calls, public tap, analytics, updates, package publication or new proof framework.
- Proof: exercise the real isolated Homebrew command against a produced local candidate and run the existing installed proof. Reuse retained candidate bytes and runner when available. Confirm candidate identity and cleanup. The platform delegate runs only the missing Homebrew row after landing; do not repeat successful channel proofs. Run routine code checks through pm jobs.
- Defers: native platform completion, which remains with 0066, and publishing.
