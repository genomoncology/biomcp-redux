# PMA trade name against applicant

The replies were recorded from the public openFDA device PMA endpoint on 2026-11-17 by `biomcp --record` in pm job j42287, without a key; each body is unchanged.

| Request | Status | Recorded bytes | SHA-256 of the recorded file |
| --- | --- | --- | --- |
| `https://api.fda.gov/device/pma.json?limit=5&skip=0&sort=decision_date%3Adesc&search=trade_name%3A%22Tempus%22` | 404 | 506 | `75ba915ac13f638305b010fb8be3d0f4bb38d2a60fecfacbb0899ad71c8e3360` |
| `https://api.fda.gov/device/pma.json?limit=5&skip=0&sort=decision_date%3Adesc&search=applicant%3A%22Tempus%22` | 200 | 10,300 | `c940b9d9f93f7b7518ce4a5481ecb3057ebb28dccb2790fc7909f1321d44af9f` |

The question is the reg-affairs pair run's "what is Tempus xT's FDA pathway" (bench ticket 0061, 2026-11-16). openFDA's trade name for P210011 is "xT CDx" and its applicant is "Tempus AI, Inc.", so `trade_name:"Tempus"` finds nothing while `applicant:"Tempus"` counts 6 records, all P210011 and its supplements. Read live on 2026-11-17: `trade_name:"Tempus xT"` also finds nothing.
