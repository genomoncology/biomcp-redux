"""One keyless publisher-page witness per proposed registry record template.

Run only through pm job. No retries or redirects. A failed source skips its
remaining witness. Raw pages remain outside Git and transfer to Yellow before
scratch removal. No publisher failures authorize substitute requests.
"""
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import time

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'scripts'))
from source_capture import file_digest, verify_archive

REQUESTS = [
    ('reactome', 'pathway', 'R-HSA-109581', 'https://reactome.org/content/detail/R-HSA-109581'),
    ('wikipathways', 'pathway', 'WP53', 'https://www.wikipathways.org/pathways/WP53.html'),
    ('cbioportal', 'study', 'brca_tcga', 'https://www.cbioportal.org/study/summary?id=brca_tcga'),
    ('gwas', 'association', '6110', 'https://www.ebi.ac.uk/gwas/associations/6110'),
    ('gwas', 'study', 'GCST0000577', 'https://www.ebi.ac.uk/gwas/studies/GCST0000577'),
    ('faers', 'adverse-event', '26940567', 'https://api.fda.gov/drug/event.json?search=safetyreportid%3A%2226940567%22&limit=5&skip=0&sort=receivedate%3Adesc'),
    ('maude', 'adverse-event', '50952244', 'https://api.fda.gov/device/event.json?search=mdr_report_key%3A%2250952244%22&limit=1'),
    ('nci', 'trial', 'NCI-2021-12653', 'https://www.cancer.gov/research/participate/clinical-trials-search/v?id=NCI-2021-12653'),
    ('fda-510k', 'diagnostic', 'K173492', 'https://www.accessdata.fda.gov/scripts/cdrh/cfdocs/cfpmn/pmn.cfm?ID=K173492'),
]


def main():
    scratch = Path.home() / 'workspace/job-evidence/biomcp-redux-0250' / str(time.time_ns())
    scratch.mkdir(parents=True)
    stopped, rows = set(), []
    for index, (source, kind, native_id, url) in enumerate(REQUESTS):
        if source in stopped:
            rows.append(dict(source=source, kind=kind, skipped=True))
            continue
        plan = dict(deadline_seconds=20, max_bytes=2097152, spacing_seconds=1,
                    requests=[dict(id=kind, source=source, url=url, response_format='bytes')])
        plan_path = scratch / f'{index}.plan.json'
        plan_path.write_text(json.dumps(plan))
        output = scratch / str(index)
        result = subprocess.run([sys.executable, 'scripts/source_capture.py', '--plan', str(plan_path), '--out', str(output)], check=False)
        receipt = json.loads((output / '0000.receipt.json').read_text())
        headers = (output / '0000.headers').read_text(errors='replace')
        location = next((line.split(':', 1)[1].strip() for line in headers.splitlines() if line.lower().startswith('location:')), None)
        row = dict(source=source, kind=kind, native_id=native_id, requested_url=url,
                   final_url=url, redirect_location=location, status=receipt['status'],
                   outcome=receipt['outcome'], body_sha256=receipt['body_sha256'],
                   archive_sha256=file_digest(Path(str(output) + '.tar.gz')))
        rows.append(row)
        if result.returncode:
            stopped.add(source)
        time.sleep(1)
    (scratch / 'summary.json').write_text(json.dumps(rows, indent=2) + '\n')
    members = ['summary.json'] + sorted(p.name for p in scratch.glob('*.tar.gz'))
    manifest = {name: file_digest(scratch / name) for name in members}
    (scratch / 'manifest.json').write_text(json.dumps(manifest))
    archive = scratch.parent / (scratch.name + '.tar.gz')
    with tarfile.open(archive, 'w:gz') as packed:
        for name in members + ['manifest.json']:
            packed.add(scratch / name, arcname=name)
    verify_archive(archive)
    destination = 'workspace/data/biomcp-redux/0250-source-record-urls'
    subprocess.run(['ssh', 'yellow', 'mkdir', '-p', destination], check=True)
    subprocess.run(['scp', str(archive), 'yellow:' + destination + '/publisher-records.tar.gz'], check=True)
    local_hash = file_digest(archive)
    remote = subprocess.check_output(['ssh', 'yellow', 'sha256sum', destination + '/publisher-records.tar.gz'], text=True).split()[0]
    if remote != local_hash:
        raise ValueError('Yellow archive differs')
    print(json.dumps(dict(archive_sha256=local_hash, records=rows)))
    shutil.rmtree(scratch)
    archive.unlink()


if __name__ == '__main__':
    main()
