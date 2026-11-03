#!/usr/bin/env python3
"""Validate exported Word packages; optionally render through local LibreOffice."""
import argparse
import pathlib
import subprocess
import xml.etree.ElementTree as ET
import zipfile
from collections import Counter

parser = argparse.ArgumentParser()
parser.add_argument("directory", type=pathlib.Path)
parser.add_argument("--render", action="store_true")
parser.add_argument("--pattern", default="*.docx")
parser.add_argument("--standard", action="store_true", help="Require a paragraph-only standard document")
parser.add_argument("--compare-content", type=pathlib.Path, help="Compare text with matching DOCX filenames in another directory")
args = parser.parse_args()
ns = {"w": "http://schemas.openxmlformats.org/wordprocessingml/2006/main"}
files = sorted(args.directory.glob(args.pattern))
if not files:
    raise SystemExit("No DOCX files found")
for path in files:
    with zipfile.ZipFile(path) as archive:
        for name in archive.namelist():
            if name.endswith((".xml", ".rels")):
                ET.fromstring(archive.read(name))
        root = ET.fromstring(archive.read("word/document.xml"))
        if args.standard and root.findall(".//w:tbl", ns):
            raise ValueError(f"{path.name}: standard export contains layout tables")
        if args.compare_content:
            with zipfile.ZipFile(args.compare_content / path.name) as reference:
                reference_root = ET.fromstring(reference.read("word/document.xml"))
            # Runs may split at hyperlinks or combine name/proficiency text.
            # Compare word multiplicities so these layout choices do not look
            # like content loss while missing/duplicated words still fail.
            texts = lambda tree: Counter(word for t in tree.findall(".//w:t", ns) for word in (t.text or "").split())
            if texts(root) != texts(reference_root):
                raise ValueError(f"{path.name}: content differs between export modes")
        parents = {child: parent for parent in root.iter() for child in parent}
        for text in root.findall(".//w:t", ns):
            if parents[text].tag != "{" + ns["w"] + "}r":
                raise ValueError(f"{path.name}: text outside a Word run")
        relationships = ET.fromstring(archive.read("word/_rels/document.xml.rels"))
        if not any(r.attrib.get("Type", "").endswith("/styles") and r.attrib.get("Target") == "styles.xml" for r in relationships):
            raise ValueError(f"{path.name}: missing styles relationship")
        for table in root.findall(".//w:tbl", ns):
            parent = parents[table]
            siblings = list(parent)
            index = siblings.index(table)
            if index + 1 >= len(siblings) or siblings[index + 1].tag != "{" + ns["w"] + "}p":
                raise ValueError(f"{path.name}: missing table separator")
    if args.render:
        output = args.directory / "rendered"
        output.mkdir(exist_ok=True)
        subprocess.run(["libreoffice", "-env:UserInstallation=file:///tmp/resume-gallery-lo-profile", "--headless", "--convert-to", "pdf", "--outdir", str(output), str(path)], check=True, timeout=90)
        if not (output / (path.stem + ".pdf")).exists():
            raise RuntimeError(f"{path.name}: no rendered PDF")
print(f"Validated {len(files)} DOCX packages" + (" and rendered PDFs" if args.render else ""))
