# Repository instructions

Before making product or architecture changes, read `MVP_SPEC.md`. It is the authority for product scope, privacy constraints, architecture principles, and acceptance criteria.

Read `CURRENT_PLAN.md` to find the active phase and next task. Keep that file current when a phase or its verification status changes. Do not create a separate roadmap that duplicates it.

## Working rules

- Keep all core workflows local and usable offline. Do not add accounts, telemetry, remote services, external fonts, or network dependencies without an explicit product decision.
- Keep career facts separate from resume drafts. Draft edits must not silently change profile data.
- Keep business rules out of UI code. Use explicit interfaces and return actionable errors for expected failures.
- Do not build future-facing infrastructure outside the MVP scope. In particular, validate the PDF pipeline before committing to a rendering architecture.
- Treat changes to product scope, privacy, persistence semantics, or public cross-module contracts as proposals: document the impact and alternatives before implementing them.
- Verify changes with the relevant project checks and report commands and results accurately. Do not claim checks were run when they were not.
- At phase completion, record what was verified, what remains unverified, and the next phase entry conditions in `CURRENT_PLAN.md`.

## Source of truth

Follow the document precedence defined in `MVP_SPEC.md` §12. If active documents conflict, stop work on the affected change and propose one resolution; do not maintain parallel competing specifications.
