# DOCX acceptance workflow

Shared typography and spacing tokens live in `design/resume-layout.json`.
Preview resolves them into CSS variables; DOCX resolves them into Word styles.
The Word builder in `crates/resume-export/src/word.rs` owns text runs,
paragraphs, table cells, rows, and table boundaries. Modern Business skill
cards use these primitives, including right-aligned proficiency text.

Generate nine templates × two paper sizes × long/sparse content:

```sh
cargo run -p resume-export --example export_gallery -- /tmp/resume-export-gallery
python3 scripts/check-docx.py /tmp/resume-export-gallery
python3 scripts/check-docx.py /tmp/resume-export-gallery/standard --standard --compare-content /tmp/resume-export-gallery
```

The package check parses every XML part, checks that text belongs to Word runs,
verifies the styles relationship, and checks table separator paragraphs.
It is a structural regression check, not a visual acceptance result.

Run outside restricted execution environments when LibreOffice cannot start:

```sh
python3 scripts/check-docx.py /tmp/resume-export-gallery/standard --standard --render
```

Standard is the default UI export and has no layout tables. The gallery root
contains experimental design documents; `standard/` contains matching standard
documents. Use `--compare-content` to detect missing or duplicated text across modes.
The planned stable design renderer needs actual local rendering and per-page
placement; current experimental exports do not provide that guarantee.

PDFs appear in `/tmp/resume-export-gallery/rendered`. Inspect every page,
especially final text, Unicode glyphs, sparse sidebars, long rows and page
boundaries. Compare Modern Business against Preview first: name/contact
alignment, section hierarchy, paragraph spacing, skill name/proficiency
alignment, and card density. Open an exported DOCX, edit a paragraph and a
skill, save, reopen, and render again. Repeat in Microsoft Word where available.

Acceptance remains pending until actual PDFs and editing behavior have been
reviewed. XML validity alone does not establish Word schema or visual parity.
