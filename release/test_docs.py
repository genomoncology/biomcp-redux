"""Public package descriptions and staged release notes preserve migration guidance."""
from email.parser import Parser
from pathlib import Path
import tempfile
import unittest
import zipfile

from release.artifacts import wheel
from release.readme import description
from release.test_licenses import write_payloads
from release.workflow import release_notes


class PublicDocs(unittest.TestCase):
    def test_built_wheel_carries_readme_and_absolute_links(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_payloads(root)
            binary = root / 'biomcp'
            binary.write_bytes(b'candidate')
            artifact = wheel(root, binary, {'version': '1.0.0', 'platform': 'linux_x86_64',
                                            'distribution': 'biomcp-cli'})
            with zipfile.ZipFile(artifact) as archive:
                metadata = Parser().parsestr(archive.read('biomcp_cli-1.0.0.dist-info/METADATA').decode())
            self.assertEqual(metadata['Description-Content-Type'], 'text/markdown')
            self.assertIn(description(), metadata.get_payload())
            self.assertIn('# BioMCP\n', metadata.get_payload())
            self.assertNotIn('](docs/', metadata.get_payload())
            self.assertNotIn('](sdlc/', metadata.get_payload())
            self.assertIn('https://github.com/genomoncology/biomcp/blob/main/docs/recipes.md', metadata.get_payload())

    def test_staged_notes_include_migration_and_candidate_schema_ids(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            release_notes(root, '1.0.0', 'a' * 40)
            notes = (root / 'RELEASE-NOTES.md').read_text()
            self.assertIn(Path('release/notes.md').read_text(), notes)
            self.assertIn('FHIR patient reading is planned for 1.1', notes)
            self.assertIn('## Changed schema identities', notes)
            self.assertIn('biomcp.adverse-event-faers-card/', notes)
            self.assertIn('gh attestation verify ARTIFACT', notes)
