# Registry name searches

The zero and positive bodies are unmodified ClinicalTrials.gov responses captured on 2026-10-29. The bounded requests selected NCT05273554 and United States locations. Both exact `MD Anderson` phrases and both broader native queries (`query.spons` and `query.locn`) returned zero. The exact sponsor phrase `M.D. Anderson Cancer Center` returned one trial. A broader query does not guarantee a match.

The zero body SHA-256 is `b54968c05ff881a1ef7ce604b6a340248f8e2aee8637bef06da0f5693f520df8`. The positive body SHA-256 is `53902d8f35b9cd98b66d77803dc84418398f5416cd0ff6889db93262aced9734`. The version body reuses the same day's retained source-version response.

The recording files are explicit request-mapping stubs. They retain these native values while checking the CLI's source request parameters. The combined-filter continuation stub reuses the positive native row and changes only the total and continuation token to exercise paging. It does not claim the source matched those combined filters. The refusal exchange is synthetic HTTP 403. No trial field, site, identity or clinical fact is invented.
