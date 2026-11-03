# Project Status

> Updated: 2026-11-02
> Status: Current product roadmap concluded; no active work item.

This file is the concise status overview. See [ROADMAP.md](ROADMAP.md) for the closure note and [MVP_SPEC.md](MVP_SPEC.md) for product requirements.

## Delivered features

- **Local resume workspace:** desktop app, SQLite persistence, profiles, multiple drafts, backup and restore.
- **Career profile:** contact details, summary, experience, education, skills, projects, credentials, and optional career sections including languages, awards, volunteering, publications, memberships, references, and interests.
- **Draft editing:** choose and order profile content, tailor resume copy, and keep draft changes separate from career facts.
- **Templates:** nine layouts for professional, creative, academic, leadership, traditional, business, and two-column resumes; sparse sections collapse cleanly. Palettes and heading decorations are saved per draft.
- **Preview and export:** local PDF, static HTML/CSS ZIP, and editable DOCX. Standard DOCX is single-column and paragraph-based; template-preserving DOCX is experimental.

## Verification snapshot

- User reviewed profile, draft, backup, and PDF workflows, including long and Unicode content and A4/Letter output.
- Rust formatting, workspace checks, Clippy, frontend typecheck/build, and export package checks passed during implementation.
- Standard DOCX: 36 fixtures rendered; content markers, page bounds, and empty pages inspected.

## Deferred

- HTML ZIP offline opening and print review.
- DOCX edit/save/reopen in LibreOffice and compatibility review in Microsoft Word.
- Packaged-app installation and disconnected desktop smoke check before release.
- Stable template-preserving DOCX layout; current design export is experimental.
- Release identity, publisher/license metadata, signing, and distribution decisions.
