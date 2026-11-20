"""Write the label-history cases' synthetic label PDFs (ticket 1066).

Each archived FDA label PDF a recorded label-history call read is replaced in the replay by a
small PDF that shows the text BioMCP's own reader took from it, one reader line per text line
(long lines wrapped at spaces), in plain Helvetica. The cases then test finding the section,
the search and the reply on the real labels' text, within the repository's fixture budget.

Usage: python3 scripts/label_history_fixtures.py TEXT_DIR REPLAY_DIR
TEXT_DIR holds <sidecar name>.txt, the reader's text for each recorded PDF sidecar.
Each sidecar named there is rewritten in place, and README lines giving each original's
SHA-256 and size are printed.
"""
import hashlib
import pathlib
import sys
import zlib

LINES_PER_PAGE = 60
WIDTH = 180


def lines(text):
    for line in text.split("\n"):
        while len(line) > WIDTH:
            cut = line.rfind(" ", 0, WIDTH)
            if cut <= 0:
                cut = WIDTH
            yield line[:cut]
            line = line[cut:].lstrip(" ")
        yield line


def escape(line):
    raw = line.encode("cp1252", errors="replace")
    return raw.replace(b"\\", b"\\\\").replace(b"(", b"\\(").replace(b")", b"\\)")


def pdf(text):
    all_lines = list(lines(text))
    pages = [all_lines[i:i + LINES_PER_PAGE] for i in range(0, len(all_lines), LINES_PER_PAGE)] or [[]]
    objects = []
    font = 3 + 2 * len(pages)
    kids = []
    for n, page in enumerate(pages):
        content = b"BT /F1 6 Tf 8 TL 20 780 Td\n" + b"".join(b"(" + escape(l) + b") Tj T*\n" for l in page) + b"ET\n"
        packed = zlib.compress(content, 9)
        page_id, content_id = 3 + 2 * n, 4 + 2 * n
        kids.append(page_id)
        objects.append((page_id, b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 %d 0 R >> >> /Contents %d 0 R >>" % (font, content_id)))
        objects.append((content_id, b"<< /Length %d /Filter /FlateDecode >>\nstream\n" % len(packed) + packed + b"\nendstream"))
    objects.insert(0, (2, b"<< /Type /Pages /Kids [" + b" ".join(b"%d 0 R" % k for k in kids) + b"] /Count %d >>" % len(kids)))
    objects.insert(0, (1, b"<< /Type /Catalog /Pages 2 0 R >>"))
    objects.append((font, b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>"))
    objects.sort()
    out = bytearray(b"%PDF-1.4\n%\xe2\xe3\xcf\xd3\n")
    offsets = {}
    for number, body in objects:
        offsets[number] = len(out)
        out += b"%d 0 obj\n" % number + body + b"\nendobj\n"
    xref = len(out)
    size = max(offsets) + 1
    out += b"xref\n0 %d\n0000000000 65535 f \n" % size
    for number in range(1, size):
        out += b"%010d 00000 n \n" % offsets[number]
    out += b"trailer\n<< /Size %d /Root 1 0 R >>\nstartxref\n%d\n%%%%EOF\n" % (size, xref)
    return bytes(out)


def main(text_dir, replay_dir):
    for text_file in sorted(pathlib.Path(text_dir).glob("*.txt")):
        sidecars = list(pathlib.Path(replay_dir).rglob(text_file.stem))
        if len(sidecars) != 1:
            sys.exit(f"{text_file.stem}: {len(sidecars)} sidecars")
        original = sidecars[0].read_bytes()
        sidecars[0].write_bytes(pdf(text_file.read_text(encoding="utf-8")))
        print(f"{sidecars[0].relative_to(replay_dir)} sha256 {hashlib.sha256(original).hexdigest()} {len(original)} bytes")


if __name__ == "__main__":
    main(*sys.argv[1:3])
