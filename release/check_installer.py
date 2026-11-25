"""Check that the public installer matches the release candidate before publication."""
import argparse
import hashlib
from pathlib import Path
import sys
import urllib.request


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('candidate', type=Path, help='Path to the candidate install.sh')
    parser.add_argument('--url', default='https://biomcp.org/install.sh',
                        help='Installer URL (default: %(default)s)')
    args = parser.parse_args()
    try:
        candidate = args.candidate.read_bytes()
        with urllib.request.urlopen(args.url, timeout=30) as response:
            served = response.read()
    except (OSError, ValueError) as error:
        sys.exit(f'Installer check failed for {args.url} and {args.candidate}: {error}')
    if served != candidate:
        sys.exit(f'Installer mismatch at {args.url}: served SHA-256 {hashlib.sha256(served).hexdigest()}, '
                 f'candidate SHA-256 {hashlib.sha256(candidate).hexdigest()} ({args.candidate})')
    print(f'Installer at {args.url} matches {args.candidate}')


if __name__ == '__main__':
    main()
