#!/usr/bin/env python3
"""Write the synthetic PDFs of the PDF reader's font cases (1069) into a fixture folder.

Each draws English sentences in an Identity-H composite font whose two-byte codes are glyph
IDs 29 below the Latin character codes, as the FDA labels that misread did. No font program is
embedded; the reader only needs the codes and the character map.

- garbled.pdf: no /ToUnicode map, so nothing says what the glyphs are and the text is garbled.
- refused-map-hostile.pdf: a map in the header style lopdf refuses, with a range over every
  four-byte code and thousands of extra sections, within the reader's rebuild budget.
- refused-map-over-budget.pdf: the same map with more sections than the budget allows.
"""
import sys
import zlib
from pathlib import Path

SENTENCES = [
    "There is a risk of serious infections in patients treated with the drug.",
    "Monitor patients for signs and symptoms of infection during and after treatment.",
    "The most common adverse reactions were nausea, headache and upper respiratory infections.",
    "Do not start the drug in patients with an active serious infection.",
]
HEADER = (
    "/CIDInit /ProcSet findresource begin\r12 dict begin\rbegincmap\r/CIDSystemInfo <<\r"
    "/Registry (Synthetic+0) def\r/Ordering (T1UV) def\r/Supplement 0 def\r>> def\r"
    "/CMapName /Synthetic+0 def\r"
)
FOOTER = "endcmap\rCMapName currentdict /CMap defineresource pop\rend end\r"


def glyphs(line: str) -> str:
    return "".join(f"{ord(c) - 29:04X}" for c in line)


def content() -> bytes:
    lines = [f"<{glyphs(s)}> Tj T*" for _ in range(12) for s in SENTENCES]
    return f"BT /F1 9 Tf 11 TL 36 760 Td {' '.join(lines)} ET".encode()


def cmap(sections: int) -> bytes:
    body = "1 begincodespacerange\r<0000> <FFFF>\rendcodespacerange\r"
    body += "2 beginbfrange\r<00000000> <FFFFFFFF> <0041>\r<0003> <005D> <0020>\rendbfrange\r"
    body += "1 beginbfchar\r<FF00> <0041>\rendbfchar\r" * sections
    body += "1 beginbfrange\r<0003> <005D> <0020>\rendbfrange\r"
    return (HEADER + body + FOOTER).encode()


def pdf(to_unicode: bytes | None) -> bytes:
    page = content()
    objects = [
        b"<< /Type /Catalog /Pages 2 0 R >>",
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 5 0 R >> >> /Contents 4 0 R >>",
        stream(page),
        b"<< /Type /Font /Subtype /Type0 /BaseFont /Synthetic /Encoding /Identity-H /DescendantFonts [6 0 R]"
        + (b" /ToUnicode 7 0 R" if to_unicode else b"")
        + b" >>",
        b"<< /Type /Font /Subtype /CIDFontType2 /BaseFont /Synthetic /CIDSystemInfo << /Registry (Adobe) /Ordering (Identity) /Supplement 0 >> /DW 500 >>",
    ]
    if to_unicode:
        objects.append(stream(to_unicode))
    out = bytearray(b"%PDF-1.7\n")
    offsets = []
    for number, body in enumerate(objects, 1):
        offsets.append(len(out))
        out += f"{number} 0 obj\n".encode() + body + b"\nendobj\n"
    xref = len(out)
    out += f"xref\n0 {len(objects) + 1}\n0000000000 65535 f \n".encode()
    out += b"".join(f"{offset:010d} 00000 n \n".encode() for offset in offsets)
    out += f"trailer\n<< /Size {len(objects) + 1} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n".encode()
    return bytes(out)


def stream(data: bytes) -> bytes:
    packed = zlib.compress(data, 9)
    return f"<< /Length {len(packed)} /Filter /FlateDecode >>\nstream\n".encode() + packed + b"\nendstream"


def main() -> None:
    folder = Path(sys.argv[1])
    (folder / "garbled.pdf").write_bytes(pdf(None))
    (folder / "refused-map-hostile.pdf").write_bytes(pdf(cmap(20_000)))
    (folder / "refused-map-over-budget.pdf").write_bytes(pdf(cmap(40_000)))


if __name__ == "__main__":
    main()
