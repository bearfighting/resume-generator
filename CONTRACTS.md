# Active contracts — Phase 6

This file records the contracts currently implemented for the first profile → draft → compiled resume slice. Product and privacy rules remain governed by `MVP_SPEC.md`.

## Rust domain entities

- `CareerProfile` and `ResumeDraft` use schema version 3 and serialize to JSON with snake_case field names. V1/v2 profile and draft records upgrade on read; new fields default to empty collections, no summary override, and a hidden draft summary.
- `ResumeDocument` uses schema version 3 and stores typed sections with generic semantic entries (`heading`, `metadata`, and `body`).
- IDs are stable non-empty strings. IDs are unique within a profile category; section IDs and entry IDs are unique within a draft.
- A draft references exactly one `profile_id`. Every draft entry's `source_id` resolves within the section's `SectionKind`.
- A `DraftEntry` controls inclusion, order, and an optional `summary_override`. An override affects the compiled `ResumeDocument` only; it never writes to the profile.
- CareerProfile v3 contains contact and professional-summary fields plus experience, education, project, skill, certification, language, award, volunteering, publication/talk, membership, license, reference, and interest collections.
- ResumeDraft v3 controls summary inclusion and draft-only summary copy, per-entry inclusion/order/text overrides, and controlled font family, 80–140% scale, accent color, and density overrides.
- References are third-party contact details, stored locally and excluded from new drafts by default.
- `compile(profile, draft)` is pure. It validates references, applies inclusion and order, applies text overrides, and returns a semantic document without UI, PDF, or persistence instructions.
- `updated_at` is an RFC 3339 UTC timestamp string supplied by the application boundary.

## SQLite v1

- `PRAGMA user_version = 1` identifies the database schema. Newer versions are rejected with an actionable error rather than opened as if compatible.
- `career_profiles` and `resume_drafts` each store the indexed ID/version/time plus a JSON payload. `resume_drafts.profile_id` references `career_profiles.id` with delete restricted; the repository refuses to delete a profile while drafts refer to it.
- `save_workspace(profile, draft)` validates and serializes both records before writing, then upserts both in one SQLite transaction.
- A failed validation or write must leave the previously saved workspace intact. Loading a draft whose profile is missing or whose JSON is invalid returns an error.

## Versioned JSON backup

- `BackupEnvelope.schema_version` is independent from the profile/draft versions and is currently `1`.
- A backup contains all profiles and drafts. Import validates the complete file, unique IDs, schema versions, and every profile/draft/source reference before changing SQLite.
- `replace` clears and restores the full snapshot in one SQLite transaction; a database failure rolls back the prior data. `cancel` returns `cancelled` without reading or modifying the database.
- File export writes a same-directory temporary file, syncs it, then atomically replaces the destination. Import rejects files larger than 50 MiB.

## Tauri command boundary

- `save_workspace({ profile, draft }) -> ResumeDocument` validates/compiles, then persists the profile and draft atomically.
- `load_workspace({ draftId }) -> CompiledWorkspace | null` loads the referenced profile/draft pair and returns semantic and visual compilation.
- `list_profiles()` returns the stored profiles for the local profile selector.
- `list_drafts({ profileId })` returns saved drafts for the selected profile.
- `preview_workspace({ profile, draft }) -> { document, visual_document }` compiles without persistence for live preview.
- `export_backup({ path })` writes the versioned full-data JSON file locally.
- `import_backup_json({ json, strategy })` validates and atomically restores locally selected backup content; inputs above 50 MiB are rejected.
- `import_backup_json({ json, strategy })` imports locally supplied JSON after size/schema/reference validation.
- `import_backup({ path, strategy }) -> ImportOutcome` accepts `replace` or `cancel`.
- Errors cross the command boundary as visible strings in this spike; UI code must not open SQLite or duplicate domain validation/compilation.

## Deliberately limited in this slice

The desktop editor currently exposes one profile at a time; the domain and repository support multiple profiles. SQLite table schema remains v1 because records are versioned JSON payloads. Profile/draft JSON records upgrade additively to v3.

## Template compiler v3

- TemplateDefinition and VisualDocument are serde-compatible, versioned contracts; the current visual-document schema version is 3.
- `VisualBlock::Section` carries its optional `SectionKind`. The resume adapter sets it for every compiled resume; generic content documents may omit it. This transient classification lets renderers place sections by meaning rather than matching localized titles.
- `apply_template(content, template_id, overrides)` accepts the built-in `classic`, `modern`, `compact`, `creative_design`, `executive_leadership`, `two_column_modern`, `two_column_sidebar`, `creative_minimal`, and `modern_business` templates and is deterministic.
- The existing `classic`, `modern`, and `compact` IDs remain stable so saved drafts and backups keep resolving to a built-in template. User-facing names are Classic / Traditional, Modern Professional, and Academic / Research.
- The content input is format-neutral: identity, ordered sections, headings, metadata, and body text. The adapter apply_template_to_resume maps the semantic resume document to that input and rejects unsupported semantic document versions.
- Style overrides are bounded: font scale is 80–140%, accent color is #RRGGBB, and density/font families are controlled enums. The compiler rejects malformed required content and returns actionable errors.
- Long body text is preserved and emits a long_text warning diagnostic for review in the rendered output.
- Empty semantic sections are omitted during profile/draft compilation. Two-column UI rendering drops empty rails and expands the remaining content to one column.
- The generated visual document contains format-independent identity/section blocks, optional semantic section kinds, layout, page size, style tokens, and diagnostics. It contains no HTML, CSS, or PDF instructions and is not persisted, so the schema bump requires no SQLite or backup migration.

## Desktop editor — Phase 4 complete

- The desktop exposes Profile, Resumes, Editor, and Settings navigation. The Profile view edits multiple experiences; the Resumes view can create, select, reload, and save multiple independent drafts.
- Template choice is draft-specific and previewed through the Rust template compiler. Preview requests do not persist; Save changes persists the profile/draft pair.
- The preview is built with DOM text nodes from the visual document; resume content is never interpreted as HTML.
- Paper size is a local print setting; other style overrides persist with the selected draft.
- The user checked the Phase 4 multi-experience/multi-draft workflow and reported no issues. Phase 5 UI additions still need desktop verification.

## Palette and decoration extension

Draft style overrides gain optional `palette_id` and `heading_decoration` fields. Missing values preserve existing template appearance. Palette IDs `palette_1`–`palette_4` resolve within the selected template; a custom accent overrides only the palette accent. Resolved surface, rule, and dark background colors travel in the visual style. Decoration choices are `rule` and `minimal`, with template-specific treatment. This additive extension uses serde defaults and the current v3 JSON envelope; old records remain readable. The alternative of storing arbitrary color dictionaries is deferred in favor of validated built-in combinations.

## Local document exports

- `resume-export` consumes the validated, transient `VisualDocument`; it does not read SQLite or network resources.
- `export_html_bundle({ path, title, document, css })` builds `index.html` from escaped VisualDocument text and packages it with `resume.css`. The stylesheet is embedded from the local UI build; the artifact includes no JavaScript, remote assets, or remote fonts. URLs remain visible and are emitted as clickable HTML hyperlinks.
- `export_docx({ path, document, mode? })` writes an editable OOXML document with paragraph/heading styles, page size, and external hyperlink relationships. Omitted mode defaults to `standard`: single-column semantic paragraphs without layout tables. `design` retains the experimental template renderer with known spacing/pagination limitations. Any other mode is rejected. Export mode does not modify saved drafts or templates. Stable per-page design export is not yet implemented or accepted.
- Both commands write a temporary ZIP in the destination directory and atomically replace the selected file after successful archive creation. The native save dialog supplies the destination; cancellation writes nothing.
- Paper size for an export comes from the Preview page's current local setting. Template, palette, and layout choices are represented through the shared visual document; DOCX translates them into native editable tables, paragraphs, borders, and shading rather than embedding a screenshot.
