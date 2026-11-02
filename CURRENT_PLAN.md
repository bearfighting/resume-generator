# Current Plan

> Status date: 2026-11-02  
> Active phase: Phase 6 — Professional Templates and Export Formats
> Overall status: Phases 0–4 complete; Phase 5 product checks passed. Public release is deferred while template quality and export formats are improved.

See [ROADMAP.md](ROADMAP.md) for the milestone sequence and acceptance goals.

## Active goal

Expand the local career inventory, let drafts select each optional section, and redesign the nine templates to adapt to complete or sparse content before adding HTML/DOCX exports.

## Phase 0 tasks (complete)

1. Confirm the first formal acceptance platform and record its OS/runtime assumptions.
2. Create the smallest runnable Tauri desktop shell and document reproducible development/build commands.
3. Verify local SQLite access and persistence across application restart.
4. Build one fixed, synthetic resume fixture and a local HTML preview.
5. Export that fixture to PDF using a candidate offline pipeline. Check A4 and Letter, font fallback, lists, heading/page breaks, long URLs, Unicode, and a document longer than two pages.
6. Compare preview and PDF for extracted text/order, page count, margins, overflow, and obvious layout drift. Record evidence and known limitations.
7. Write `ADR-0001-pdf-pipeline.md` with the selected approach, dependencies/licensing, offline behavior, platform limits, preview consistency, and test strategy.

## Phase 0 acceptance gate

Phase 0 is complete only when the spike can be run from documented commands on the selected platform, stores and reloads local data, and produces a PDF whose text is selectable and whose fixture content is complete. The PDF checks and any remaining layout differences must be recorded. If the candidate pipeline fails, evaluate an alternative and update the ADR before proceeding.

Do not implement the full domain model, three-template engine, or polished editor before this gate passes. Use the spike findings to decide which architectural boundaries are justified.

## Phase 1 tasks (complete)

1. Define versioned profile/draft types, validation rules, and a pure draft compiler.
2. Store profiles and drafts with a SQLite migration and atomic workspace upsert.
3. Add a Tauri command boundary for save, reload, and compile; keep SQL and business rules out of UI code.
4. Exercise the vertical slice from the UI, including restart recovery and draft-only overrides.
5. Verify multiple drafts/experiences and versioned JSON backup/restore before closing Phase 1.

### Phase 1 acceptance gate (passed)

Domain and storage checks must cover schema/serialization, duplicate IDs, invalid references, sorting, inclusion, overrides, persistence after reopening the repository, and preservation of prior data after invalid saves. The desktop slice must be manually confirmed to save/reload/compile after app restart. JSON backup/restore is still required before Phase 1 is complete.

## Verified / unverified

- Verified: `MVP_SPEC.md` defines the product goals, privacy constraints, proposed architecture, and end-to-end acceptance scenarios.
- Verified: the repository began with no application source, build configuration, README, or ADR. A minimal Tauri shell with SQLite save/load commands, local preview, print controls, a fixed synthetic HTML fixture, and `scripts/pdf-spike.sh` now cover the intended A4/Letter, multilingual text, lists, long URL, and pagination checks.
- Environment observed: Ubuntu 26.04.1 LTS x86_64; Rust 1.96.0, Cargo 1.96.0, Node 26.2.0, pnpm 11.5.2, GTK 3.24.52, WebKitGTK 4.1 version 2.52.6, and SQLite library 3.46.1 are present. Google Chrome 155 and PDF inspection tools are installed.
- Verified: `cargo check` succeeds for the Tauri 2.12.5 + bundled rusqlite shell; Tauri dependencies can be downloaded after crates.io access was restored.
- Implemented: `resume-domain` and `resume-storage-sqlite` workspace crates, profile/draft schema v2 entities, pure compile/validation, transactional profile/draft CRUD, reference-safe updates/deletes, versioned JSON backup import/export with replace/cancel behavior, and Tauri commands for save/load/backup.
- Verified: `cargo fmt --all -- --check`, `cargo test --workspace` (7 domain + 7 SQLite + 2 backup-file tests), `cargo clippy --workspace --all-targets -- -D warnings`, TypeScript strict check, and Vite production build pass after the CRUD/backup additions.
- Verified externally by user: the Phase 1 form saves and compiles; after closing and reopening, the draft reloads. The draft-only override remains separate from the profile fact; the user reported no problem.
- Implemented in Phase 4 so far: Profile/Resumes/Editor/Settings navigation, multiple-draft selection and creation, draft-specific built-in template selection, Rust-backed live preview, multiple experience editing, per-draft inclusion/overrides, reference-safe experience deletion, save/load feedback, and accessible empty/error states. The UI still edits only one profile.
- Verified externally by user: the Tauri desktop app launches; SQLite data survives close/reopen and reload; the app's PDF output has expected page counts, selectable and searchable text, and preview consistency, with no reported problem.
- Verified externally by user: the final `bash scripts/pdf-spike.sh /tmp/resume-pdf-spike` run generated A4 (594.96 × 841.92 pt) and Letter (612 × 792 pt) PDFs, each 3 naturally flowing pages. Extracted non-whitespace text per page: A4 2,174 / 2,058 / 1,107 characters; Letter 1,934 / 2,062 / 1,343 characters. PDF metadata reports unencrypted tagged output. Extracted text includes the long URL and CJK content; `pdffonts` shows embedded Liberation Sans, Noto CJK, and Noto Arabic fonts. Visual inspection of the A4 final page showed no obvious clipping or overlap.
- Fixture review: visual inspection of the first PDFs found the final two pages were nearly empty because forced page breaks were used to reach three pages. Forced breaks were removed and the resume was lengthened. The revised A4 output has 3 natural-flow pages. Python-based extraction now reports 2,174 / 2,058 / 1,107 non-whitespace characters per page and passes the 500-character minimum.
- Follow-up: the earlier page-count script had an incompatible awk form-feed separator. Replacing it with a Python helper resolved the issue; the final full A4/Letter script run passes.
- Blocked in agent sandbox: Google Chrome headless crashes before rendering because the sandbox denies a crashpad socket operation (`setsockopt: Operation not permitted`). Tauri compiles, but launching it fails to initialize GTK, so no desktop window can be opened here. Firefox as a Snap also cannot create its runtime directory in the restricted filesystem.
- Platform decision: Ubuntu 26.04.1 LTS x86_64 is the first formal acceptance target.
- PDF pipeline decision: accepted for the Linux MVP in `docs/decisions/ADR-0001-pdf-pipeline.md` (local WebKitGTK print dialog).

## Phase 0 result

Phase 0 acceptance passed on the user's Ubuntu desktop session. The selected MVP path is WebKitGTK printing from the same local WebView used for preview. Chrome headless is a diagnostic tool only.

## Phase 1 result

Phase 1 acceptance passed. Domain validation/compilation, SQLite CRUD and migrations, reference safety, versioned backup/restore, and the desktop save → compile → restart → reload path are verified.

## Phase 2 tasks (complete)

1. Compile profile facts and draft selections into a semantic ResumeDocument.
2. Apply draft ordering, inclusion flags, and per-entry text overrides without mutating profile facts.
3. Reject dangling references and malformed draft/profile associations.
4. Cover empty content, sparse sections, tied ordering, long Unicode text, and multiple drafts against the same profile.

### Phase 2 acceptance gate (passed)

Pure compiler tests cover selection, ordering, hidden entries, overrides, invalid references, empty profile/draft content, omission of empty sections, long Unicode text, multiple independent drafts, and preservation of the profile/draft inputs. Workspace tests and lint pass.

## Phase 2 result

Phase 2 acceptance passed. The compiler remains a pure function and returns semantic content only; it applies stable (order, id) ordering, omits hidden entries and sections with no visible entries, preserves long Unicode overrides, and leaves the source profile and drafts unchanged. Multiple drafts can compile independently from one profile.

## Phase 3 tasks (complete)

1. Define a serializable, versioned template schema and format-independent visual document nodes.
2. Add Classic, Modern, and Compact built-in definitions with controlled typography, color, density, page size, and layout tokens.
3. Compile semantic resume content with validated style overrides and actionable errors.
4. Preserve all rendered resume text and emit diagnostics for unusually long content; cover built-ins and serialization with deterministic tests.

### Phase 3 acceptance gate (passed)

The same semantic document compiles with all three built-ins into valid visual documents without losing identity, contact, section, entry, metadata, or body content. Tests cover schema round-trip, invalid template/style inputs, bounded overrides, and long-text diagnostics.

## Phase 3 result

Phase 3 acceptance passed for the Rust template compiler. Phase 4 connects its visual documents to the local WebView preview.

## Phase 4 tasks (complete)

1. Provide Profile, Resumes, Editor, and Settings views for normal desktop use.
2. Support multiple experience facts, draft selection, per-draft inclusion/removal, and draft-only copy.
3. Connect all built-in templates to the Rust compiler and render the visual document in a live local preview.
4. Provide empty, loading, unsaved, save, and actionable error feedback with keyboard-operable controls.
5. Verify multi-draft isolation, restart reload, template switching, and print in the desktop WebKitGTK session.

### Phase 4 acceptance gate (passed)

The desktop workflow lets a user edit multiple experience facts and drafts without JSON, switch templates while preserving content, and reload saved drafts. The user manually checked the expanded multi-experience and multi-draft workflow and reported no issues. At the Phase 4 handoff, education, skills, projects, and certifications remained a profile-schema gap; Phase 5 adds those types, with the new UI still awaiting desktop verification.

## Next action

Verify the newly added profile categories and style controls in the desktop app, then check backup round-trips and template-specific PDF output in WebKitGTK.

## Phase 4 result

- Implemented: Tauri list-drafts and non-persisting preview commands; load now returns a compiled visual document.
- Implemented: four keyboard-operable navigation views; profile facts and multiple experience CRUD; create/select/reload multiple drafts; per-draft inclusion/removal and text overrides; live preview for all three built-ins; local print paper setting; empty, loading, save, and error feedback.
- Verified: pnpm typecheck, pnpm build, cargo fmt --all, cargo test --workspace, and cargo clippy --workspace --all-targets -- -D warnings pass.
- Verified externally by user: the updated desktop workflow, including multiple experiences and drafts, was checked with no issues reported.
- Phase 4 scope note: the editor exposes one profile; paper-size preference remains a local print setting.

## Phase 5 entry checklist

1. (Done) Resolve the profile schema gap and version/backup compatibility.
2. (User-verified) Add usable backup export/import controls and validate malformed/unknown-version backup feedback from the UI.
3. (User-verified) Run PDF regression on WebKitGTK with template switching, A4/Letter, long content, Unicode, and selectable/searchable text.
4. (Static review recorded; desktop offline check deferred) Review offline/privacy behavior and document known limits.
5. (Deferred) Validate and distribute the Linux release artifact after the template/export work is accepted.

## Phase 5 progress

- Decision: ADR-0002 accepts profile/draft schema v2, format-neutral semantic entries, and lazy v1-to-v2 compatibility upgrades; SQLite table layout and backup envelope schema remain unchanged.
- Implemented: typed education, project, skill, and certification records; category-scoped source validation; schema v2 semantic compilation; old JSON profile/draft upgrade on read and backup import.
- Implemented: Tauri commands for file-backed atomic backup export and bounded JSON-content import; platform-native save dialog for choosing the export location; Settings UI controls for backup, style overrides, and paper size; Profile/Editor fields for all five categories.
- Verified: domain tests cover mixed-category compilation and v1 upgrades; storage tests cover legacy SQLite/backup upgrades and round-trip; workspace format, tests, and Clippy pass. Frontend typecheck and Vite production build pass after profile/style/backup wiring.
- Verified externally by user: selecting a backup now shows the selected file and the explicit “Import selected backup” action works; the imported data reloads in the app.
- Verified externally by user: all profile categories and style persistence, backup export/import including invalid/unsupported backup handling, and PDF output across templates and page sizes with long/Unicode text and selectable/searchable text were checked without obvious issues.
- Verified by static review: no app-level network client, remote font/content, telemetry, or crash-reporting integration was found; CSP limits WebView connections to Tauri IPC.
- Verified: Linux Debian package builds and its metadata lists GTK 3 and WebKitGTK 4.1 runtime dependencies.
- Remaining manual checks: run the packaged application, then confirm startup, editing, save/reload, preview, print, and backup while disconnected from the network.
- Known release limits: package is unsigned and has no update channel; the bundle identifier remains `local.resume.studio.phase0` to preserve existing app data; public publisher/license metadata needs confirmation.
- Backup export opens the platform-native save dialog with `resume-backup.json` as the suggested filename; the operating system supplies the location UI. The backup content remains on-device and the Rust export path uses atomic replacement.
- Added a profile selector so a separate template-demo profile can be selected without mixing synthetic facts into the user's existing profile. The seed example uses repository validation and upserts stable demo IDs.

## Phase 6 — Professional Templates and Export Formats (active)

1. (Done) Record the expanded profile inventory, per-draft inclusion rules, reference privacy default, schema migration, and empty-section layout contract in ADR-0003.
2. (Implemented; compatibility checks still needed) Upgrade CareerProfile, ResumeDraft, ResumeDocument, and transient VisualDocument contracts to v3 with backward-compatible v1/v2 profile/draft upgrade and backup import behavior.
3. (Implemented) Add optional contact fields and profile summary, plus typed languages, awards, volunteer experience, publications/talks, memberships, licenses, references, and interests to profile editing and storage.
4. (Implemented) Add draft-level summary include/override and selection/order controls for every category; references start excluded in new drafts.
5. (Implemented; desktop visual review pending) Rework all nine templates for full and sparse resumes. Empty sections disappear; two-column layouts collapse cleanly when either rail has no selected content.
6. Implement static HTML+CSS export from the shared document model, with no scripts, CDN, or remote assets; verify offline opening and print styling.
7. Implement editable DOCX export from the shared semantic/visual model; verify text, order, headings, lists, links, and page breaks in LibreOffice and Microsoft Word where available.
8. Re-run PDF output checks after visual changes; preserve selectable text and verify A4/Letter across all nine templates and sparse fixtures.

### Phase 6 progress

- Accepted: ADR-0003 defines the expanded local career inventory, draft-specific selection rules, reference privacy default, schema upgrades, and adaptive layout behavior.
- Implemented: schema v3 profile/draft/document contracts and v3 transient visual-document contract; profile/draft v1/v2 upgrades are additive and the SQLite table layout remains unchanged.
- Implemented: Profile form fields for contact, summary, and all typed career categories; independently collapsible Profile and Editor sections with expand/collapse-all actions; Editor inclusion panels cover every profile category; draft summary include/override; per-entry inclusion and ordering controls; reference items start excluded in new drafts.
- Implemented: Editor grid items align to content height, avoiding tall blank category panels when neighboring sections contain more entries; empty categories explain that their records can be added from Profile.
- Implemented: compiler support for every added category; empty body paragraphs are omitted, empty sections are omitted by compilation, and two-column previews collapse when a rail has no content.
- Updated: every optional profile field in the synthetic template demo now has sample data; references remain unselected in its draft. Re-running the seed command updates the existing demo IDs.
- Checked: `cargo fmt --all -- --check`, `cargo check --workspace --all-targets`, `cargo clippy --workspace --all-targets -- -D warnings`, `pnpm typecheck`, and `pnpm build` pass. The automated test suites and desktop visual/PDF checks have not been run for this schema/template expansion.
- Next: review all nine templates in desktop preview with full and sparse data; verify old backup import/restore, then run PDF page and selectable-text checks before proceeding with HTML and DOCX export.

## Deferred release work

After Phase 6 acceptance, run a disconnected desktop smoke check on the packaged build, finalize application identity and publisher/license metadata, then decide whether to sign and distribute the Linux package.

## Template palette and heading decoration increment

- Added four curated palettes for each of the nine templates, resolved in Rust to accent, surface, rule, and dark-background colors. Preview offers named choices and color swatches.
- Added optional fine-divider and template-specific minimal heading decoration; custom accent retains precedence over palette accent.
- Both choices belong to draft style overrides and persist through SQLite and JSON backups. Missing fields retain existing template appearance; no SQL migration is required.
- Validation: frontend type checking/build, Rust compilation, and domain tests are run for this increment. Desktop visual acceptance across palettes, sparse sections, A4/Letter, and dark sidebars remains pending.
