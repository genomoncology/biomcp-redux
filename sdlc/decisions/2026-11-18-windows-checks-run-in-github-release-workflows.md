# Windows checks run in GitHub release workflows

Ian, November 18, 2026, via Jarvis (m38146), answered “Yeah, go ahead ... make that happen” when asked “Can we just run only the Windows stuff up on GitHub, then, for public projects?” The shared Windows Server VM is retired and its resources return to its host's owner.

Run BioMCP's Windows checks in the existing GitHub Windows release jobs after local checks pass. Handle Windows-only fixes and reruns there at release time. This replaces the retired VM recovery path in [release qualification](../../release/qualification.md). Jarvis owns the shared machine cleanup.
