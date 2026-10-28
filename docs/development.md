# Development

Run `make schema-generate` after changing schema types. The command uses the schema contract test to check compatibility before writing the checked-in schemas. It fails unless the named generation test runs and passes.

Run `make schema-release-check` to compare the generated schemas with the recorded release baseline and check for drift. This command leaves the checked-in schemas and release baseline unchanged.
