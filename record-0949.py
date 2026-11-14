"""Temporary 0949 job script: fetch GTR live, count EGFR x MedGen:C0007131, print a cut replay as base64."""
import base64, gzip, io, re, sys, urllib.request, zlib

BASE = "https://ftp.ncbi.nlm.nih.gov/pub/GTR/data/"
GENE, CUI = "EGFR", "C0007131"


def get(name):
    with urllib.request.urlopen(BASE + name, timeout=600) as r:
        print(name, r.headers.get("Last-Modified"), flush=True)
        return r.read()


rel = get("test_condition_gene.txt").decode("utf-8")
lines = rel.split("\n")
head, kept = lines[0], []
gene_ids, cond_ids, all_cond_rows = set(), set(), 0
for line in lines[1:]:
    c = line.split("\t")
    if len(c) < 8:
        continue
    if c[2] == "gene" and c[7] == GENE:
        gene_ids.add(c[0]); kept.append(line)
    elif c[2] == "condition" and c[3] == CUI:
        cond_ids.add(c[0]); kept.append(line)
both = gene_ids & cond_ids
print(f"gene {GENE}: {len(gene_ids)} tests; condition {CUI}: {len(cond_ids)} tests; both: {len(both)}")
print("both:", sorted(both))
keep = gene_ids | cond_ids
rel_out = "\n".join([head] + kept) + "\n"

ver = gzip.decompress(get("test_version.gz")).decode("utf-8")
vl = ver.split("\n")
ver_out = "\n".join([vl[0]] + [l for l in vl[1:] if l.split("\t", 1)[0] in keep]) + "\n"
print("version rows:", ver_out.count("\n") - 1)

# Stream the XML: keep each GTRLabData block's GTRLab and the kept tests' attributes and Specimen.
raw = get("gtr_ftp.xml.gz")
d = zlib.decompressobj(16 + zlib.MAX_WBITS)
buf, out, blocks = "", [], 0
test_re = re.compile(r'<GTRLabTest ([^>]*?)(/>|>(.*?)</GTRLabTest>)', re.S)
spec_re = re.compile(r'<Specimen>.*?</Specimen>', re.S)
lab_re = re.compile(r'<GTRLab .*?</GTRLab>|<GTRLab [^>]*/>', re.S)


def cut(block):
    tests = []
    for m in test_re.finditer(block):
        attrs = m.group(1)
        acc = re.search(r'GTRAccession="([^"]+)"', attrs)
        v = re.search(r'Version="([^"]+)"', attrs)
        if not (acc and v) or f"{acc.group(1)}.{v.group(1)}" not in keep:
            continue
        s = spec_re.search(m.group(3) or "")
        tests.append(f"<GTRLabTest {attrs}>{s.group(0)}\n    </GTRLabTest>" if s else f"<GTRLabTest {attrs.rstrip()} />")
    if not tests:
        return None
    lab = lab_re.search(block)
    return "<GTRLabData>" + (lab.group(0) if lab else "") + "".join(tests) + "</GTRLabData>"


prefix = None
dec = __import__("codecs").getincrementaldecoder("utf-8")()
pos = 0
while pos < len(raw) or buf:
    if pos < len(raw):
        buf += dec.decode(d.decompress(raw[pos:pos + (8 << 20)]))
        pos += 8 << 20
    if prefix is None:
        i = buf.find("<GTRLabData>")
        if i < 0:
            continue
        prefix, buf = buf[:i], buf[i:]
    while True:
        j = buf.find("</GTRLabData>")
        if j < 0:
            break
        block, buf = buf[:j + len("</GTRLabData>")], buf[j + len("</GTRLabData>"):]
        blocks += 1
        c = cut(block)
        if c:
            out.append(c)
    if pos >= len(raw):
        break
xml_out = prefix + "".join(out) + "</GTRPublicData>"
print("xml blocks:", blocks, "kept labs:", len(out), "kept tests:", xml_out.count("<GTRLabTest "))

for name, text in [("relations", rel_out), ("versions", ver_out), ("xml", xml_out)]:
    data = gzip.compress(text.encode("utf-8"), 9)
    print(f"BEGIN {name} {len(text)}")
    print(base64.b64encode(data).decode())
    print(f"END {name}")
