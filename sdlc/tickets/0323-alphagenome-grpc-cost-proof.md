# 0323 — Measure the gRPC transport's cost and prove it on every release target

Status: OPEN.

Milestone: 5

Depends on: 0322

## Outcome

A build delegate measures what the landed gRPC transport adds and runs the installed candidate on every release target with the local fixture.

- Why: Ian ruled on 2026-10-27 that AlphaGenome must work (`2026-10-27-alphagenome-streaming-transport.md`). PM ruling 2026-10-27 (m8126 ask 60) splits the transport along its review boundaries: 0320 codec, 0321 worker, 0322 persistence, 0323 cost and target proof.
- Starts from: landed main after 0322, baseline 14e26555, release.md's targets and the 0293 rehearsal recipe.
- Rules: the same toolchain, profile, host and cache conditions for baseline and candidate. Raw timings and sizes are kept through ordinary job evidence. Package checks refuse a runtime Python, SDK, `protoc` or system ZSTD requirement. A failed target returns to the PM before 0198 starts. No AlphaGenome call.
- Target proof, PM ruling 2026-10-27 (m8126 ask 67): on each available target, run the public worker API synthetic TLS fixture through the fetch integration tests, and separately check the installed CLI package and its lack of runtime SDK, Python, `protoc` or system ZSTD needs. Installed CLI proof through the gRPC path belongs to 0198, after its adapter exists.
- Proof: executable size, transitive crate count, cold and warm build time and peak build memory against the baseline; the installed candidate passes the local TLS fixture on Linux x86_64 and arm64, macOS x86_64 and arm64, and Windows x86_64, or names the target it could not run.
- Changes: none expected beyond evidence and fixes it finds, each its own ticket.
- Defers: the variant section (0198).

## Recorded proof

The public worker API TLS fixtures and separate installed CLI package checks passed on Linux x86_64 and arm64, macOS arm64, and Windows x86_64. Native Intel macOS was unavailable. The measured source revision was `804f1309453f05b86956afcdfe2518c51767212c`; the baseline was `14e2655501fbc55cd23f7da41056c19d8c50bc6b`. No AlphaGenome request or key read occurred.

The controlled Yellow pair j17276 used Rust 1.95.0, the release profile, mold, four build jobs, CPUs 0–3, the same source path, and a separate fresh owned compiler cache for each revision. Each revision ran one cold build and one warm-cache build with a fresh target directory. Cold and warm executable hashes matched within each revision. Package nodes include the root and normal/build dependencies; excluding the six workspace packages gives 140 and 186 external packages.

| Measurement | Baseline | Candidate |
|---|---:|---:|
| Executable bytes | 40,249,616 | 40,487,544 |
| Normal/build package nodes | 146 | 192 |
| Cold GNU wall seconds | 415.17 | 462.18 |
| Warm GNU wall seconds | 59.29 | 64.73 |
| Cold sampled summed RSS bytes | 2,213,502,976 | 2,240,741,376 |
| Warm sampled summed RSS bytes | 1,391,984,640 | 1,415,794,688 |

The sampler tracked the union of Cargo descendants and the owned foreground cache server's descendants, including compiler children. All observed affinities stayed within CPUs 0–3. Cold cache hits were zero; warm hits were 181 and 249 with no additional misses. These are single descriptive samples under varying shared host load. Summed process RSS duplicates shared pages and can miss peaks; nominal 100 ms sampling had a largest observed gap of 0.265 seconds. GNU time excludes the separate cache server tree. The samples establish no general performance or memory saving claim. Original Gator timings in j17123 remain separate descriptive samples with incomplete cache-server attribution.

Raw evidence remains outside Git on Yellow under `/home/ian/workspace/data/biomcp-redux/0323-grpc-cost-proof/804f1309453f05b86956afcdfe2518c51767212c/`. The following hashes identify each directory's `SHA256.json`; all raw file hashes were verified after transfer.

| Evidence directory | Passing job | Manifest SHA-256 |
|---|---|---|
| `controlled-cost-yellow/j17276` | j17276 | `efd90fd6bb6f57885c29b1cb2d00e15b5c582870bef7a32a1c4d55f43723980e` |
| `linux-x86_64-floor/j17273` | j17273 | `03a54cf7d1a013255557fe85597744e2d44d697ed62e8ecfe6d6127b8d898b56` |
| `linux-arm64-floor/j17160` | j17160 | `344a764ab1505c6972de732f7b3b77635dc90125bee067d1b56620ef677a4a13` |
| `macos-arm64-canonical/j17179` | j17179 | `ec5f863bb87c204ffcc9bc7b6d7bf54f99e47906080d5a93e8f286668c9b3d79` |
| `windows-x86_64/j17149` | j17149 | `17a6dabf7731a1059a95c29202c30cf0dc6120a1dd4e922a76c0042da68acb62` |

The Linux floor jobs used manylinux_2_28; native Gator proof alone did not establish that floor. macOS j17179 reused the exact retained original build bytes under canonical `/private/tmp`, without recompilation, after the original detailed capture was lost and a noncanonical `/tmp` consumer recipe failed. Original logs and failed recovery witnesses remain in the same Yellow evidence root. Homebrew was unavailable. Installed CLI checks covered the wheel, desktop package, registry package, archive installer and download installer without a runtime SDK, Python, `protoc` or system ZSTD requirement. The worker fixtures exercise gRPC; the current CLI has no reachable gRPC adapter, so installed CLI transport proof remains with 0198.

Owned host builds, caches, temporary tools and ticket containers were removed after consumers and verified capture. The Windows VM remains stopped. Its owned long-path Cargo residue remains at `C:\Windows\Temp\biomcp-0323-148d80d9ebd44fb6870fa05c6ccb147a\cargo` for cleanup when the VM next legitimately runs. No compiled binaries or raw evidence enter Git.
