# ADR-0001: PDF pipeline for the Linux MVP

- Status: Accepted for the first supported platform
- Date: 2026-11-02
- Platform: Ubuntu 26.04.1 LTS x86_64

## Context

The MVP needs offline PDF export that stays close to the local HTML preview, keeps text selectable and searchable, handles Unicode and long content, and does not require a remote service. Tauri WebView printing needed a desktop test before selecting the pipeline.

## Decision

Use the Tauri window's local WebView print pipeline (`window.print()`) for the Linux MVP. The user chooses a PDF destination in the native print dialog. On the first supported target, WebKitGTK renders both the preview and printed document from the same local HTML/CSS. Keep the fixture based Chrome headless run as a separate repeatable layout and text extraction diagnostic; it is not the production renderer.

Do not use a remote renderer or require a separate Chrome installation to export a resume. Keep the export path local and ship all required assets with the app.

## Evidence

- `cargo check` passes for the Tauri 2.12.5 + bundled SQLite shell; `pnpm build` passes for the local Vite preview.
- The user manually verified that the Tauri desktop window launches, SQLite data survives close/reopen and reload, A4 and Letter print with expected page counts, and the exported PDF text is selectable and searchable. The user reported no issue with preview consistency.
- The synthetic Chrome headless fixture completes offline for both A4 and Letter. Each output has three natural flow pages. Extracted non whitespace text per page: A4 2,174 / 2,058 / 1,107 characters; Letter 1,934 / 2,062 / 1,343 characters.
- The fixture PDF is unencrypted and tagged. Extracted text includes the long URL and CJK content; `pdffonts` shows embedded Liberation Sans, Noto CJK, and Noto Arabic fonts. Visual inspection of the A4 final page showed no obvious clipping or overlap.
- The agent sandbox cannot initialize a GTK desktop window, so app level runtime checks were performed by the user in the Ubuntu desktop session.

## Consequences and limitations

- The first formal acceptance target is Ubuntu 26.04.1 LTS x86_64. macOS and Windows WebView print behavior remain unverified and must not be implied by the Linux result.
- PDF creation uses the operating system print dialog. The user chooses the local destination. If the product later requires a programmatic destination path, unattended export, or structured renderer diagnostics, that requirement needs a new spike and decision.
- Font fallback and pagination can vary with installed system fonts and WebKitGTK versions. Keep the fixed fixture in PDF regression checks and test supported runtime versions before broadening platform support.
- Print cancellation, unavailable print backend, unwritable destination, and disk full recovery need explicit handling and acceptance tests in the export implementation.
- Chrome headless results are useful diagnostic evidence but are not proof of WebKit output consistency.

## Revisit when

Revisit this decision before supporting another desktop platform, requiring direct to path export, or changing the preview renderer. Re run the fixture and compare extracted text, page count, margins, overflow, Unicode, and long URL wrapping.
