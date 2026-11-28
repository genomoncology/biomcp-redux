# 1242 — extend-offline-reach-check

Status: OPEN.

Milestone: 10

Depends on: 1240

## Outcome

The offline reach check from 1240 also covers each record class whose fix landed after it: ClinVar records without a classification and included-only records (1205), Open Targets drugs without a mechanism (1211), ChEMBL rows on salt forms (1210), gnomAD held alleles by every input form (1209), the Evidence Repository rows without a document ID (1204), and the CIViC profiles without a ClinGen allele ID and submitted evidence on the drug route, once their tickets land.

## Why

PM ruling 2026-11-27: 1240 lands with the classes that are reachable on main today, rather than waiting for every reach fix, so the gate starts guarding them now. This ticket adds each later class so a regression in it fails the gate by name.

## Evidence

- 1240's report (gate j48491): the check covers GenCC, WikiPathways, Reactome, CPIC, Purple Book and recorded WHO eEML replies in 13 seconds; a dropped ClinVar class failed it by name in mutation job j48423.

## Proof

- Proof: one pinned slice per added class; a mutation that drops each class fails the check and names it; the reach stage stays at 60 seconds or less with no network.

## Starts from

- Starts from: the 1240 reach check and the recorded cases each listed ticket added.

## Keeps

- Keeps: no network in the gate; no licensed content in the repo.

## Defers

- Defers: classes from tickets that have not landed when this one starts; the PM adds them as they land.

## Changes

- Changes: the reach check's fixtures and class list.
