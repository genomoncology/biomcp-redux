"""Choose an explicit source revision or require a clean local release tree."""
import argparse
import os
import re
import subprocess
import sys


def revision(explicit=None):
    selected = explicit or os.environ.get('BIOMCP_BUILD_REVISION')
    if not selected:
        dirty = subprocess.check_output(['git', 'status', '--porcelain', '--untracked-files=normal'], text=True)
        if dirty:
            raise ValueError('dirty source tree: provide an explicit revision for a local rehearsal')
        selected = subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()
    if not re.fullmatch(r'[0-9a-f]{40}', selected):
        raise ValueError('source revision must be a full Git commit')
    return selected


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--revision', help='Explicit source identity for a rehearsal or Git archive')
    try:
        print(revision(parser.parse_args().revision))
    except (ValueError, subprocess.CalledProcessError) as error:
        sys.exit(str(error))
