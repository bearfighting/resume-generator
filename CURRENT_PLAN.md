# Project Status

> Updated: 2026-11-03
> Status: Cascade deletion and confirmation dialogs implemented; desktop interaction verification remains.

This file is the concise status overview. See [ROADMAP.md](ROADMAP.md) for the closure note and [MVP_SPEC.md](MVP_SPEC.md) for product requirements.

## Current iteration — deleting profiles and resume drafts

- **Implemented:** delete a saved resume with a confirmation dialog that states the impact and irreversibility, warns about unsaved edits, then selects the next resume or shows the empty state.
- **Implemented:** delete a profile and its saved resume drafts together after one confirmation dialog that lists all affected resumes and states the deletion is irreversible. Storage performs the cascade in one transaction; exported files outside the app are left untouched.
- **Polished:** the app now uses shared color tokens for page surfaces, text, borders, focus states, and destructive actions; the confirmation dialog follows the same palette.
- **Implemented:** deleting the last profile opens a blank new-profile form so the user can create another profile.
- **Verified:** `pnpm typecheck`, `pnpm build`, `cargo fmt --check`, `cargo check --workspace`, and `git diff --check` passed after the deletion flow changes; `pnpm typecheck`, `pnpm build`, and `git diff --check` passed after the color refinement.
- **Not verified:** manual desktop interaction for dialogs, cascade deletion, unsaved edits, deleting the last profile, and retrying a storage error. Automated tests were not run.
- **Next entry condition:** run the desktop app and check those deletion paths before treating this iteration as fully verified.

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
