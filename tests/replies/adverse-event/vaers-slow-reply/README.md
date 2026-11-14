# CDC WONDER VAERS COVID-19 symptoms, slow reply

The one reply was recorded unchanged from CDC WONDER's public VAERS endpoint (`POST https://wonder.cdc.gov/controller/datarequest/D8`, vaccine `COVID19`, grouped by `D8.V13-level2` symptoms, form body SHA-256 `1177166ef3960aee234e0cad87a754055009bd76aab856a4de4d9c707324e0ab`) on 2026-11-13 by `biomcp --record` in pm job j39797. Status 200, `text/html;charset=ISO-8859-1`, 882,762 bytes, SHA-256 of the Latin-1 body `49fb0e21f619432aacbb2b9ad7d138fb2a282b0bc7d5fbba55b791b81ca68727`. WONDER took 36.8 seconds to answer, past the 20-second default command budget, so the command completes only within the 60-second budget VAERS declares.

The dataset vintage is 10/30/2026. The table has 12,971 symptom rows and a report total of 995,917 events.
