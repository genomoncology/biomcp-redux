# List a trial's nearest sites with only the columns needed

## Question

Which sites of trial NCT00627419 are nearest to Columbus, Ohio, and what is each site's recruitment status?

## 1. Retrieve and inspect

```sh
biomcp get trial NCT00627419 locations --lat 39.96 --lon -83.00 --limit 20 --fields=facility,city,state,status,distance_mi
```

Typed tool: `trial` `{"action":"get","get":{"id":["NCT00627419"],"section":["locations"],"lat":"39.96","lon":"-83.00","limit":20},"fields":["facility","city","state","status","distance_mi"]}`.

With `--lat` and `--lon`, sites come nearest first with their straight-line miles. `--fields` keeps only the named columns of each site row, so a long site list costs about half as much to read; the reply's `fields` lists them, and the page command keeps them. A dotted path such as `coordinates.latitude` names one value inside a column. Without `--fields`, each site also carries its postal code, country, coordinates and contacts.

## Next

Distance is straight-line miles from the stated point, not travel distance. Read the site's status and contacts before suggesting it, and confirm eligibility and recruitment with the study team.

Read response status, problems and source terms at every step. Live replies can change.
