# Linux build and release notes

## Current target

The first acceptance target is Ubuntu 26.04.1 LTS on x86_64. This project produces a Debian package for local installation; it does not currently publish signed packages or a public download.

## Build

Install the prerequisites listed in the [Tauri Linux guide](https://v2.tauri.app/start/prerequisites/), then run from the repository root:

```sh
pnpm install --frozen-lockfile
pnpm typecheck
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
pnpm desktop:build:deb
```

The Debian artifact is written to `target/release/bundle/deb/`. Install a locally built artifact with:

```sh
sudo apt install ./target/release/bundle/deb/*.deb
```

The package uses system GTK and WebKitGTK libraries. Build and verify it on the target distribution; do not assume a package built on a newer Linux distribution works on older distributions.

## Privacy and offline review

- Static source review found no application-level HTTP client, `fetch`, WebSocket, telemetry, crash reporter, external font, or remote content integration.
- The Content Security Policy restricts the WebView to packaged app resources, inline styles used by the local preview, and Tauri IPC (`ipc:` / `http://ipc.localhost`).
- Career data is stored in the app-local SQLite database. Backup and PDF writes occur only after a user action and use a path selected through a native dialog or print dialog.
- The PDF spike's Chrome crash-reporting flags apply only to the diagnostic script; Chrome is not the production PDF renderer.
- Core app behavior has not yet been independently packet-captured during a disconnected desktop session. Confirm startup, editing, save/reload, preview, print, and backup while offline before treating the offline acceptance check as complete.

## Known release limits

- The packaged UI exposes one career profile, although the storage model supports multiple profiles.
- PDF output depends on the system WebKitGTK and installed fonts; layout can vary slightly across distributions.
- The bundle identifier remains `local.resume.studio.phase0` to preserve the current app-data location. Choose and freeze the production identifier before a first public release, with an explicit data migration plan if it changes.
- No signing, update channel, automated release pipeline, or public distribution process is configured.
- Confirm licensing and publisher metadata before public distribution.
