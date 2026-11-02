#!/usr/bin/env python3
import pathlib
import sys


text = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8")
pages = [page for page in text.split("\f") if page.strip()]
sparse = False

for page_number, page in enumerate(pages, start=1):
    char_count = len("".join(page.split()))
    print(f"  page {page_number}: {char_count} text chars")
    sparse |= char_count < 500

if len(pages) < 3 or sparse:
    sys.exit(1)
