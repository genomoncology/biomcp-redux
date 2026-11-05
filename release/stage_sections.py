"""Consume the five reviewed recovery runs without rebuilding or publishing."""
import argparse
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import shutil
import subprocess
import tempfile
import urllib.request
import urllib.parse
import zipfile

PRODUCT = '1bb97e64f41634b19efc42b937b195e55a20ece4'
PRODUCT_TREE = '812f6942385bc438861f368cb409e560a0a22209'
RECOVERY = 'f025e763db6bb8c109318ce2550e1d36a5d83581'
RECOVERY_TREE = '587f00e0cfd36d48183863ca37ed2dc97aae757c'
REPOSITORY = 'genomoncology/biomcp-release-rehearsal'
RUNS = {'win_amd64': '42039129684', 'manylinux_2_28_aarch64': '42039285107',
        'macosx_11_0_arm64': '42039290253', 'manylinux_2_28_x86_64': '42039294596',
        'macosx_11_0_x86_64': '42039298821'}
# Every payload input comes from PRODUCT; these are the only consumer additions.
CONSUMER_PATHS = {'.github/workflows/release-stage-sections.yml',
                  '.github/workflows/release.yml', 'release/stage_sections.py',
                  'release/test_stage_sections.py', 'scripts/check_stage_sections.sh'}


def git(root, *args):
    return subprocess.check_output(['git', '-C', str(root), *args], text=True).strip()


def source_guard(product, tooling):
    if git(product, 'rev-parse', 'HEAD') != PRODUCT or git(product, 'rev-parse', 'HEAD^{tree}') != PRODUCT_TREE:
        raise ValueError('product checkout differs from frozen source')
    for root in (product, tooling):
        if git(root, 'status', '--porcelain', '--untracked-files=no'):
            raise ValueError('source checkout has modified tracked files')
    if git(tooling, 'rev-parse', 'HEAD') != os.environ['GITHUB_SHA']:
        raise ValueError('consumer checkout differs from actual signer source')
    changes = git(tooling, 'diff', '--name-only', PRODUCT, 'HEAD').splitlines()
    if any(path not in CONSUMER_PATHS and not path.startswith('sdlc/') for path in changes):
        raise ValueError('consumer changed a product or packaging input')
    canonical = tooling / '.github/workflows/release-stage-sections.yml'
    if (tooling / '.github/workflows/release.yml').read_bytes() != canonical.read_bytes():
        raise ValueError('deployed workflow differs from reviewed consumer workflow')
    return {'product_revision': PRODUCT, 'product_tree': PRODUCT_TREE,
            'consumer_revision': git(tooling, 'rev-parse', 'HEAD'),
            'consumer_tree': git(tooling, 'rev-parse', 'HEAD^{tree}'),
            'workflow_sha256': digest(canonical), 'source_ref': os.environ['GITHUB_REF'],
            'signer_workflow_ref': os.environ['GITHUB_WORKFLOW_REF'],
            'run': os.environ['GITHUB_RUN_ID'], 'publishing': False}


def digest(path):
    result = hashlib.sha256()
    with path.open('rb') as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b''):
            result.update(chunk)
    return result.hexdigest()


def api(path):
    request = urllib.request.Request('https://api.github.com/repos/' + REPOSITORY + path,
        headers={'Authorization': 'Bearer ' + os.environ['GH_TOKEN'],
                 'Accept': 'application/vnd.github+json', 'X-GitHub-Api-Version': '2022-11-28'})
    with urllib.request.urlopen(request) as response:
        return json.load(response)


def pages(path, key):
    rows = []
    for page in range(1, 101):
        record = api(path + ('&' if '?' in path else '?') + f'per_page=100&page={page}')
        rows.extend(record[key])
        if len(rows) >= record['total_count']:
            return rows
    raise ValueError('API pagination exceeded bounded recovery inventory')


def validate_run(run, platform):
    if (str(run['id']) != RUNS[platform] or run['head_sha'] != RECOVERY
            or run['event'] != 'workflow_dispatch' or run['path'] != '.github/workflows/release.yml'
            or run['repository']['full_name'] != REPOSITORY
            or run['head_repository']['full_name'] != REPOSITORY
            or run['status'] != 'completed' or run['conclusion'] != 'success'
            or run['run_attempt'] != 1 or not run['head_branch'].startswith('rehearsal-0642-')):
        raise ValueError('recovery run identity or completed success differs')


class ObjectRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, request, response, code, message, headers, url):
        redirected = super().redirect_request(request, response, code, message, headers, url)
        if urllib.parse.urlsplit(url).netloc != urllib.parse.urlsplit(request.full_url).netloc:
            redirected.remove_header('Authorization')
        return redirected


def download_url(url, target):
    if not url.startswith('https://api.github.com/repos/' + REPOSITORY + '/'):
        raise ValueError('download must start at the repository API')
    request = urllib.request.Request(url, headers={'Authorization': 'Bearer ' + os.environ['GH_TOKEN']})
    opener = urllib.request.build_opener(ObjectRedirect())
    with opener.open(request) as source, target.open('xb') as output:
        shutil.copyfileobj(source, output, 1024 * 1024)


def download(artifact, target):
    if artifact['expired'] or not artifact.get('digest', '').startswith('sha256:'):
        raise ValueError('recovery artifact is expired or lacks API digest')
    # GitHub redirects this authenticated API call to a temporary object URL.
    download_url(artifact['archive_download_url'], target)
    if 'sha256:' + digest(target) != artifact['digest']:
        raise ValueError('downloaded archive differs from API digest')


def unpack(archive, root):
    with zipfile.ZipFile(archive) as source:
        names = set()
        for info in source.infolist():
            name = PurePosixPath(info.filename)
            if (name.is_absolute() or '..' in name.parts or '\\' in info.filename
                    or info.filename in names or (info.external_attr >> 16) & 0o170000 == 0o120000):
                raise ValueError('unsafe or duplicate artifact member')
            names.add(info.filename)
        source.extractall(root)


def unique(root, suffix):
    matches = [path for path in root.rglob(PurePosixPath(suffix).name)
               if path.as_posix().endswith('/' + suffix)]
    if len(matches) != 1:
        raise ValueError('expected one artifact member: ' + suffix)
    return matches[0]


def validate_payload(root, platform, run):
    dist = unique(root, 'product/dist/release.json').parent
    sources = json.loads(unique(root, 'section-evidence/sources.json').read_text())
    receipt = json.loads(unique(root, 'section-evidence/artifacts.json').read_text())
    expected_product = {'revision': PRODUCT, 'tree': PRODUCT_TREE}
    expected_tooling = {'revision': RECOVERY, 'tree': RECOVERY_TREE}
    for record in (sources, receipt):
        for key, expected in (('product', expected_product), ('tooling', expected_tooling)):
            if any(record[key].get(field) != value for field, value in expected.items()):
                raise ValueError('artifact source identity differs')
    if (sources['publishing'] is not False or sources['original_run'] != '42030695983'
            or receipt['run'] != str(run) or receipt['section'] != platform
            or sources['product'] != receipt['product'] or sources['tooling'] != receipt['tooling']):
        raise ValueError('artifact recovery receipt differs')
    actual = {'dist/' + path.name: digest(path) for path in dist.iterdir() if path.is_file()}
    if any(not path.is_file() or path.is_symlink() for path in dist.iterdir()) or actual != receipt['sha256']:
        raise ValueError('native file inventory or bytes differ from recovery receipt')
    candidate = json.loads((dist / 'release.json').read_text())
    if (candidate['revision'] != PRODUCT or candidate['platform'] != platform
            or candidate['distribution'] != 'biomcp-cli' or candidate['version'] != '1.0.0'):
        raise ValueError('native candidate differs from frozen product')
    report = json.loads((dist / 'install-report.json').read_text())
    smoke = [row for row in report if 'cases' in row]
    required = {'wheel', 'desktop-extension', 'mcp-registry', 'install-script', 'download-install-script'}
    if platform.startswith('macosx'):
        required.add('homebrew-formula')
    if {row['artifact'] for row in smoke} != required:
        raise ValueError('native installed channel inventory is incomplete')
    if (not report or any(row.get('result') != 'pass' for row in report
                                      if row.get('artifact') != 'homebrew-formula' or platform.startswith('macosx')) or not smoke
            or any(row.get('platform') != platform or not {'candidate-bytes', 'concurrent-first-opens', 'all-cases-and-trials', 'mcp-all-stdout'}.issubset(row.get('checks', []))
                   for row in smoke)):
        raise ValueError('native installed qualification is incomplete')
    if platform.startswith('macosx') and not any(row['artifact'] == 'homebrew-formula' for row in smoke):
        raise ValueError('native Homebrew qualification is missing')
    if platform.startswith('manylinux'):
        arch = 'amd64' if platform.endswith('x86_64') else 'arm64'
        if 'dist/biomcp-container-' + arch + '.tar' not in actual:
            raise ValueError('qualified container archive is missing')
    return dist, {'sources': sources, 'files': receipt, 'install_report': report}


def collect(args):
    if (os.environ.get('GITHUB_REPOSITORY') != REPOSITORY or os.environ.get('REHEARSAL') != 'true'
            or os.environ.get('GITHUB_EVENT_NAME') != 'workflow_dispatch'
            or not os.environ.get('GITHUB_REF', '').startswith('refs/heads/rehearsal-0370-')):
        raise ValueError('consumer requires the publication-disabled rehearsal repository')
    selected = json.loads(os.environ['RECOVERY_RUNS'])
    if selected != RUNS or os.environ.get('PRODUCT_REVISION') != PRODUCT:
        raise ValueError('exact five reviewed recovery runs and frozen source are required')
    binding = source_guard(args.product, args.tooling)
    candidate = api('/actions/runs/42030695983')
    candidate_jobs = pages('/actions/runs/42030695983/jobs', 'jobs')
    if (candidate['head_sha'] != PRODUCT or candidate['event'] != 'workflow_dispatch'
            or candidate['path'] != '.github/workflows/release.yml'
            or candidate['head_branch'] != 'rehearsal-0293-1bb97e64f416'
            or candidate['repository']['full_name'] != REPOSITORY
            or not any(row['id'] == 127256100800 and row['name'] == 'candidate'
                       and row['conclusion'] == 'success' for row in candidate_jobs)):
        raise ValueError('original successful candidate qualification differs')
    args.output.mkdir(exist_ok=False, parents=True)
    args.receipts.mkdir(exist_ok=False, parents=True)
    records = {}
    for platform, run_id in RUNS.items():
        run = api('/actions/runs/' + run_id)
        validate_run(run, platform)
        jobs = pages('/actions/runs/' + run_id + '/jobs', 'jobs')
        if len(jobs) != 1 or jobs[0]['conclusion'] != 'success':
            raise ValueError('expected exactly one successful native recovery job')
        artifacts = pages('/actions/runs/' + run_id + '/artifacts', 'artifacts')
        matching = [item for item in artifacts if item['name'] == 'recovered-' + platform]
        if len(matching) != 1:
            raise ValueError('expected exactly one recovered platform artifact')
        artifact, = matching
        if artifact['workflow_run']['id'] != int(run_id) or artifact['workflow_run']['head_sha'] != RECOVERY:
            raise ValueError('artifact API source differs from recovery run')
        if platform.startswith('manylinux'):
            log = args.receipts / (platform + '.log')
            download_url(jobs[0]['url'] + '/logs', log)
            arch = 'amd64' if platform.endswith('x86_64') else 'arm64'
            container = []
            for line in log.read_text(errors='replace').splitlines():
                start = line.find('{')
                if start < 0:
                    continue
                try:
                    row = json.loads(line[start:])
                except ValueError:
                    continue
                if isinstance(row, dict) and row.get('artifact') == 'docker':
                    container.append(row)
            if (len(container) != 1 or container[0].get('platform') != 'linux/' + arch
                    or container[0].get('result') != 'pass'
                    or not {'candidate-bytes', 'license-notices', 'mcp-all-stdout',
                            'all-cases-and-trials', 'runner-proofs'}.issubset(container[0].get('checks', []))):
                raise ValueError('completed Linux container qualification is missing')
        with tempfile.TemporaryDirectory(dir=args.output.parent) as directory:
            root = Path(directory)
            download(artifact, root / 'artifact.zip')
            unpack(root / 'artifact.zip', root / 'unpacked')
            dist, evidence = validate_payload(root / 'unpacked', platform, run_id)
            expected_cases = sum(1 for path in (args.product / 'tests/cases').rglob('*')
                                 if path.suffix in ('.yaml', '.yml'))
            summaries = [row for row in evidence['install_report'] if 'cases' in row]
            if any(row['cases'] != expected_cases or row['trials'] < expected_cases for row in summaries):
                raise ValueError('native installed cases or trials are incomplete')
            if platform.startswith('manylinux'):
                if container[0].get('cases') != expected_cases or container[0].get('trials', 0) < expected_cases:
                    raise ValueError('container installed cases or trials are incomplete')
            shutil.copytree(dist, args.output / platform)
        records[platform] = {'run': run, 'jobs': jobs, 'artifact': artifact, **evidence}
    (args.receipts / 'recovery-inputs.json').write_text(json.dumps({'binding': binding, 'candidate_run': candidate, 'candidate_jobs': candidate_jobs, 'platforms': records}, indent=2) + '\n')
    with open(os.environ['GITHUB_OUTPUT'], 'a') as output:
        output.write('version=1.0.0\n')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('product', 'tooling', 'output', 'receipts'):
        parser.add_argument('--' + name, type=Path, required=True)
    collect(parser.parse_args())


if __name__ == '__main__':
    main()
