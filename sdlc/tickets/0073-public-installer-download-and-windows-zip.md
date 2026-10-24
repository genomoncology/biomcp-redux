# 0073 — Download release archives in the public installer and unpack Windows ZIPs

Status: COMPLETE.
Landed: d0af3b1a18c3341bdd197b9aae346186d7ff1960

Milestone: 5

Depends on: 0058
Depends on: 0065

## Outcome

`install.sh` from biomcp.org installs 1.0 the way it installed 0.9: it picks the archive for the platform, downloads it from the release, checks its checksum and installs the executable. On Windows x86_64 under MSYS, MINGW or Cygwin it unpacks the ZIP archive. The local tar path from 0058 stays as it is.

- Why: 0065 found that 0.9's installer downloads its archive and supports Windows x86_64 through a ZIP, while 0058's installer only accepts a supplied tar archive. 0.9 users who install with the script would lose Windows and the one-line install.
- Starts from: the release plan from 0065; 0.9's `install.sh`; 0058's installer and install gate.
- Keeps: checksum verification before install, no checkout fallback, isolated install folder, and 0058's offline gate. Publishing still needs Ian.
- Changes: `install.sh`, the install gate cases, and the release plan's installer row.
- Proof: the install gate serves candidate archives and checksums from a local file server and runs the script against it for Linux x86_64, Linux ARM64, macOS and Windows x86_64 ZIP. A wrong checksum fails with no executable installed. An unknown platform fails with a message naming it. 0066 runs the Windows case on the existing Windows VM.
- Defers: publishing, new platforms 0.9 did not support, and package-manager channels.
