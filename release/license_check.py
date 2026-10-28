"""Read required license payloads from each distribution without installing it."""
import json
from pathlib import Path
import tarfile
import zipfile

if __package__:
    from .notices import NAMES, validate
else:
    from notices import NAMES, validate


def zip_payload(path, prefix=''):
    with zipfile.ZipFile(path) as archive:
        files = {name: archive.read(prefix + name) for name in NAMES if prefix + name in archive.namelist()}
    validate(files)
    return files


def tar_payload(path):
    with tarfile.open(path) as archive:
        files = {name: archive.extractfile(name).read() for name in NAMES if name in archive.getnames()}
    validate(files)
    return files


def container_payload(path):
    # docker save records layers in application order. Inspect the effective files.
    prefix = 'usr/share/doc/biomcp/'
    files = {}
    with tarfile.open(path) as image:
        manifest = json.load(image.extractfile('manifest.json'))
        if len(manifest) != 1:
            raise ValueError('container archive must contain one image')
        for layer in manifest[0]['Layers']:
            with tarfile.open(fileobj=image.extractfile(layer), mode='r|*') as archive:
                for member in archive:
                    name = member.name.removeprefix('./')
                    parent, _, leaf = name.rpartition('/')
                    if leaf.startswith('.wh.'):
                        removed = (parent + '/' if parent else '') + leaf[4:]
                        if leaf == '.wh..wh..opq':
                            removed = parent + '/'
                        files = {key: value for key, value in files.items()
                                 if prefix + key != removed and not (prefix + key).startswith(removed.rstrip('/') + '/')}
                    elif name.startswith(prefix):
                        relative = name[len(prefix):]
                        if relative in NAMES:
                            files[relative] = archive.extractfile(member).read() if member.isfile() else b''
    validate(files)
    return files


def native(directory, tag):
    # Import here to avoid the artifact producer's shared validation import cycle.
    if __package__:
        from .artifacts import inspect
    else:
        from artifacts import inspect
    wheel, = directory.glob('*.whl')
    record = inspect(wheel)
    wheel_notices = zip_payload(wheel, wheel.name.split("-py3-none-")[0] + ".dist-info/")
    if record != json.loads((directory / 'release.json').read_text()):
        raise ValueError('wheel producer differs from candidate record')
    archive, = [path for path in directory.iterdir() if path.name.endswith(('.tar.gz', '.zip'))]
    reference = tar_payload(archive) if archive.name.endswith('.tar.gz') else zip_payload(archive)
    if reference != wheel_notices:
        raise ValueError('wheel notices differ from archive')
    if reference != {name: (directory / name).read_bytes() for name in NAMES}:
        raise ValueError('archive notices differ from producer')
    if tag.startswith(('macosx_', 'win_')):
        bundle, = directory.glob('*.mcpb')
        if zip_payload(bundle) != reference:
            raise ValueError('desktop notices differ from archive')
    if tag.startswith('manylinux_'):
        image, = directory.glob('biomcp-container-*.tar')
        if container_payload(image) != reference:
            raise ValueError('container notices differ from archive')
