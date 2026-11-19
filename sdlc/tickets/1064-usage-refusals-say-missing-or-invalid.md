# 1064 — usage-refusals-say-missing-or-invalid

Status: COMPLETE.
Landed: 2183d0a4bf7005f8d9bb59118cf2ee54f4df0c65

Milestone: 7

## Outcome

A usage refusal says whether the option was missing, unknown or had a bad value, and shows one correct form of the command. It never describes the valid value as "Bounded nonempty text...".

## Why

Of 222 BioMCP calls in one set of twelve runs, 22 were refused, and nearly every refusal carried the same sentence: "option is missing or invalid; choose an accepted value or form and use the suggested help command". Agents could not tell which, and several stopped using the command.

## Proof

- Proof: cases for the four refusals below, each naming its real cause and a working form; they fail on the base commit.

## Starts from

- Starts from: Seen on main be96d8d40 (1.0.0), Linux, 2026-11-19, in 25 agent runs that used the command line build on real tasks.
- `get drug Aimovig label --source openfda-label --label-section warnings_and_cautions --label-section adverse_reactions` is refused with option `--label-section` and valid values "Bounded nonempty text with surrounding Unicode whitespace removed. Minimum length: 1. Maximum length: 1048576." The cause is that sections are given once, separated by commas.
- `search gene --disease "Fanconi anemia"` is refused with option `--disease` and a list of other options as its valid values. The cause is that the option does not exist.
- `search article TEXT --fields title,authors,journal,year` is refused with a list of field names that has no title in it, and nothing says title is always returned.
- `biomcp skill overview` replies "unknown skill; use biomcp skill list".
- Ticket 1057 covers two specific refusals; this ticket is the general message.

## Keeps

- Keeps: error codes and exit codes.

## Defers

- Defers: nothing.

## Changes

- Changes: the usage error builder, its cases and the error reference in the docs.
