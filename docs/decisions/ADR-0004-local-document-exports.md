# ADR-0004: Local HTML and DOCX export architecture

**Status:** Accepted
**Date:** 2026-11-02

## Context

The MVP needs a portable static resume bundle and an editable Word-compatible document. Both must work offline and preserve content from the shared VisualDocument. HTML should remain close to the existing preview; DOCX should remain straightforward to edit, even when its pagination differs from WebKit.

## Decision

- DOCX has two explicitly separate modes. Standard is the default, using single-column native Word paragraphs without layout tables. Design retains template structures as an experimental export. Neither mode changes the saved template or source facts.
- A stable design mode based on per-page layout and actual local rendering is deferred until a Modern Business prototype passes rendering/editing acceptance. Existing experimental design exports do not claim that pagination guarantee. Introducing a required LibreOffice dependency needs a separate implementation/acceptance step; standard export remains independent of LibreOffice.

- Add a `resume-export` Rust workspace crate. It receives VisualDocument values and owns archive writing and DOCX OpenXML generation; it does not read the database or perform network access.
- Build the HTML article from VisualDocument text with escaping, reuse the local template stylesheet, and package exactly `index.html` and `resume.css` in a ZIP. No scripts, remote assets, or arbitrary profile-supplied HTML are included.
- Generate DOCX as editable OOXML structures from the visual document. Preserve detected HTTP(S) URLs as editable external hyperlink relationships and apply page dimensions, font family, body scale, density, palette, and accent color. Map two-column layouts, identity treatments, and dense skill sections to native Word tables where appropriate.
- Write both formats through a same-directory temporary ZIP and atomically replace the user-selected output path only after successful completion.
- Use the Preview page's current paper-size setting for both formats. DOCX remains editable and is not expected to match browser pagination or every template's geometry pixel-for-pixel.

## Alternatives considered

- Scrape the live preview HTML in the UI and let Rust package it: rejected because it makes preview DOM the export source of truth and crosses the command boundary with raw HTML.
- Build a Word document by converting arbitrary preview HTML: rejected because browser layout constructs are brittle and usually difficult to edit in Word-compatible applications.
- Add a third-party DOCX rendering service or remote assets: rejected because export must remain fully local and offline.

## Consequences

- `resume-export` depends on the domain contract and ZIP writing only. The UI supplies the currently compiled VisualDocument and local stylesheet text; Rust validates visual schema/style tokens before rendering.
- HTML preserves the chosen visual template closely. DOCX starts with editable paragraphs and headings, then uses tables for template structures where they meaningfully improve alignment without sacrificing editability.
- Acceptance still requires opening the ZIP's `index.html` without a network, printing it, and opening/editing DOCX in LibreOffice and Microsoft Word where available. Long Unicode text, links, A4/Letter settings, and multi-page content need user-facing regression checks.
