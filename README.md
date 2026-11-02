# Local Resume Studio

Local-first desktop resume editor. It stores career facts and resume drafts in local SQLite, compiles them in Rust, previews them in the local WebView, and prints selectable-text PDFs through WebKitGTK.

## Requirements

- Ubuntu 26.04.1 LTS x86_64 for the current acceptance target
- Rust stable and Cargo
- Node.js 22+ and pnpm 11
- Tauri Linux development dependencies, including WebKitGTK 4.1 and GTK 3 development libraries

Tauri's official Linux prerequisites are listed in the [Tauri v2 documentation](https://v2.tauri.app/start/prerequisites/).

## Build and run

```sh
pnpm install
pnpm typecheck
pnpm desktop:dev
```

Build the Linux Debian package with `pnpm desktop:build:deb`. The artifact is written under `target/release/bundle/deb/`. See the [product roadmap](ROADMAP.md) for the current priorities and [Linux release notes](docs/RELEASE-LINUX.md) before distributing it.

The desktop has Profile, Editor, Preview, and Settings views. The Profile selector only changes the profile being edited. Editor lists drafts with their source profile and lets a new draft choose its profile; Preview follows the selected draft. Profile saves career facts independently, Editor saves draft content and ordering, and Preview saves the selected draft with its linked source profile, template, and style choices. Preview updates locally as you edit, and print uses the WebKit print dialog for A4 or Letter PDF output.

The editor can switch between stored career profiles; profile creation/deletion controls are not yet exposed. It includes nine built-in templates: Modern Professional, Creative / Design, Academic / Research, Executive / Leadership, Classic / Traditional, Two-Column / Modern, Two-Column / Left Rail, Creative Minimal, and Modern Business. The domain and repository support multiple records, CRUD, and versioned full-data JSON backup/restore. Profile and draft JSON schema v1/v2 data upgrades automatically to v3; the SQLite table schema remains v1. Profiles include optional contact details and typed experience, education, projects, skills, certifications, languages, awards, volunteering, publications/talks, memberships, licenses, references, and interests. References are excluded from new drafts by default. IDs and schema versions are explicit; business validation and compilation live in `crates/resume-domain`, while SQLite access lives in `crates/resume-storage-sqlite`. The Tauri frontend only calls the Rust command boundary.

To seed an isolated, synthetic profile for template review, follow [TEMPLATE-DEMO.md](docs/TEMPLATE-DEMO.md).

Settings opens a native save dialog for JSON backup export and accepts a selected JSON backup for import with replace confirmation. Export uses atomic replacement. Storage checks cover legacy v1 upgrades, round-trip restore, cancel behavior, and rollback after a simulated database write failure.

The app does not need a network connection for its core workflows. Resume data stays in the app-local SQLite database unless the user explicitly exports a PDF or JSON backup to a local path. There are no accounts, telemetry, remote fonts, or application-level network requests. The WebView Content Security Policy allows local app resources and Tauri IPC only.

## Checks

```sh
cargo fmt --all -- --check
cargo test --workspace
pnpm typecheck
pnpm build
pnpm desktop:build:deb
```

The browser PDF diagnostic remains available:

```sh
bash scripts/pdf-spike.sh /tmp/resume-pdf-spike
```

It checks the system Chrome headless output; the production Linux MVP print path is WebKitGTK as recorded in [ADR-0001](docs/decisions/ADR-0001-pdf-pipeline.md).
