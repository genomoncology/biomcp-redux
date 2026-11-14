# CDC WONDER VAERS COVID-19 symptoms

The one reply was recorded unchanged from CDC WONDER's public VAERS endpoint (`POST https://wonder.cdc.gov/controller/datarequest/D8`, vaccine `COVID19`, grouped by `D8.V13-level2` symptoms, form body SHA-256 `1177166ef3960aee234e0cad87a754055009bd76aab856a4de4d9c707324e0ab`) on 2026-11-13 by `biomcp --record` in pm job j39525. Status 200, `text/html;charset=ISO-8859-1`, 882,761 bytes, SHA-256 of the Latin-1 body `abb1cb80d73ed72aa9e81b960db91207f4ad4611648778aaf2998060f395cc12`. WONDER took 18 seconds to answer.

The dataset vintage is 10/30/2026. The table has 12,971 symptom rows in alphabetical order, a report total of 995,917 events and a footer of 4,194,079 symptom mentions (421.13%), so percents add to more than 100. WONDER's own measure metadata gives the largest row count as 151,715, which is HEADACHE, the first row once sorted.

CDC WONDER's VAERS request form and API documentation, read live the same day, offer no sort or rank option, so BioMCP orders the table it receives.
