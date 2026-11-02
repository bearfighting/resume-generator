# Product Roadmap

> Status: 2026-11-02  
> Active focus: professional resume templates and export formats  
> Release work is intentionally postponed until these product areas are accepted.

## Product direction

Local Resume Studio keeps career facts and resume drafts on the user's device, then produces clear, professional documents from one structured source. A resume should be quick to scan, easy to edit, reliable when printed, and usable without a network connection.

Use career-center guidance and public examples to learn general design principles. Do not copy proprietary templates or depend on online template assets. Keep an ATS-oriented single-column template available.

## Completed foundation

- **Phases 0–4:** Linux Tauri shell, local SQLite persistence, versioned profile/draft model, semantic and visual compilation, three built-in templates, editor, preview, and selectable/searchable PDF output.
- **Phase 5 product checks:** user verified profile categories and style persistence, backup export/import including invalid backup feedback, and PDF output across templates and paper sizes with long and Unicode content.
- **Phase 5 engineering:** backup import/export, platform-native save dialog, Linux Debian package build, and static privacy review are implemented. Release installation and disconnected desktop checks are deferred.

See [CURRENT_PLAN.md](CURRENT_PLAN.md) for the detailed history and verification record.

## Active roadmap

### 1. Career inventory and adaptive templates — active

**Goal:** Store the full range of useful career facts and let each draft select what to show, while all nine templates remain balanced with sparse or missing sections.

**Work:**

- Upgrade profile and draft schemas additively for optional contact details, profile summary, languages, awards, volunteer experience, publications/talks, memberships, licenses, references, and interests.
- Keep reference contacts private and excluded from new drafts by default. Let each draft include or omit sections and records, order them, and override summary text.
- Build synthetic fixtures for a complete resume, a sparse resume, and profiles missing each optional section.
- Refine the original [eight-direction gallery](docs/template-gallery/eight-directions.html): Modern Professional, Creative / Design, Academic / Research, Executive / Leadership, Classic / Traditional, Two-Column / Modern, Creative Minimal, and Modern Business.
- Keep the nine choices in the Rust template compiler, editor, and preview; keep existing template IDs stable for saved drafts.
- Include a reversed left-rail variation with a distinct dark-sidebar design, alongside the existing right-rail layout.
- Revisit all nine layouts for the expanded content inventory. Omit empty sections and collapse either empty rail in two-column layouts; refine typography, hierarchy, spacing, alignment, section rhythm, and page breaks.
- Keep the compatibility-oriented design single-column, with ordinary headings and selectable text. Make the two-column design an explicit visual option, not the default.
- Adapt reference designs to current profile data; do not assume portraits, decorative icons, charts, or portfolio images exist.
- Record the template design system and schema migration in [ADR-0003](docs/decisions/ADR-0003-complete-career-inventory.md).

**Status:** first implementation is in the app; user desktop review and PDF regression are the next checks.

**Acceptance:** all supported profile data round-trips through old and new backups; draft selection keeps profile facts unchanged; the nine templates work with full, sparse, and omitted sections without empty rails/headings or clipping; a single-column option remains available; A4 and Letter PDF text remains selectable and in a sensible reading order.

### 2. Static HTML + CSS export

**Goal:** Let users keep and share a browser-readable version of a resume without requiring the app or a network connection.

**Preferred artifact:** a ZIP containing `index.html`, `resume.css`, and any strictly local assets needed by the selected template.

**Acceptance:** opens locally in current desktop browsers, loads no remote assets, uses no JavaScript, preserves content and links, includes print styling, and visually matches the chosen template closely. Test extraction, Unicode, long URLs, and page printing.

### 3. Editable DOCX export

**Goal:** Provide a useful Word-compatible document users can continue editing.

**Work:**

- Prototype a local DOCX renderer from the shared resume model before choosing a long-term dependency.
- Map headings, paragraphs, lists, dates, links, fonts, spacing, and page settings into native editable Word structures.
- Keep reading order straightforward. Avoid layout tricks that make the file hard to edit or parse.

**Acceptance:** opens in LibreOffice and Microsoft Word where available; text, ordering, headings, lists, and links survive; pagination is reasonable; editing content does not corrupt the layout. DOCX need not be pixel-identical to WebKit PDF because Word has its own layout engine.

### 4. Release readiness — deferred

Resume after template and export acceptance:

- Run the packaged app offline through startup, edit, save/reload, preview, print, and backup.
- Test installation and launch of the Debian package on the selected Ubuntu target.
- Decide the production application identifier before public release and define migration behavior if it changes.
- Confirm publisher and license metadata; decide whether signing and a public update/distribution channel are in scope.

## Shared architecture rule

Profile and draft facts compile to one semantic resume document. Template design then produces a versioned visual document. PDF/HTML rendering should share the same HTML/CSS mapping where that improves consistency. DOCX should use a dedicated renderer over the shared structure and template tokens; it should not be generated by scraping the preview DOM or by treating arbitrary HTML as the source of truth.

Before implementing additional formats, inspect whether `VisualDocument` can represent the required structure (for example paragraphs, lists, inline links, and emphasis) without format-specific hacks. Add versioned model fields only where the new outputs need them.

## Design references

- [MIT Career Advising: Crafting an effective resume](https://capd.mit.edu/resources/career-toolkit-crafting-an-effective-resume/)
- [Harvard: Create a strong resume](https://careerservices.fas.harvard.edu/resources/create-a-strong-resume/)

These are references for readability and consistency principles, not sources to copy verbatim.

### Template presentation increment

Implemented: four palettes per template and two optional heading decoration treatments, selected in Preview and persisted with the draft. Desktop/PDF visual acceptance remains part of milestone 1.
