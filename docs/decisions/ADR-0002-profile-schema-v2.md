# ADR-0002: Expand CareerProfile to schema v2

- Status: Accepted for the MVP implementation
- Date: 2026-11-02

## Context

The MVP contract includes experience, education, skills, projects, and certifications. The shipped profile currently stores only experiences, so the Phase 4 UI cannot represent the complete local profile or run the full first-start acceptance scenario.

## Decision

Add typed collections for education, skills, projects, and certifications to CareerProfile and advance the profile schema from v1 to v2. Add draft-owned font family, font scale, accent color, and density overrides; advance ResumeDraft from v1 to v2. Keep the existing SQLite table layout and backup envelope schema; both hold versioned JSON records, so no SQL column migration is needed. On reading/importing v1 records, default the new fields and upgrade the in-memory records to v2. The next save/export writes v2. Extend SectionKind additively. Advance ResumeDocument to v2 because semantic section entries become generic, format-neutral content.

## Alternatives considered

1. Keep schema v1 and add optional fields: smaller migration, but the version would no longer identify the complete profile contract.
2. Store new categories as generic JSON: avoids typed migrations but weakens validation, source references, and compiler guarantees.
3. Defer these categories: avoids code and migration work but fails the MVP profile and first-start acceptance requirements.

## Impact and compatibility

- Existing SQLite records and v1 JSON backups remain readable; absent collections and style overrides receive defaults.
- New saves and exports contain profile/draft schema v2. The backup envelope stays v1 because its top-level structure does not change.
- Draft references remain scoped by SectionKind, so equal IDs in different categories do not collide.
- The semantic document entry shape changes and its schema version advances. Existing documents are compiled from their source profile and draft, not persisted.
- Draft style preferences travel with the draft and therefore survive restart and backup/restore.
- All categories stay local. No new service, network dependency, or file format is introduced.
- Tests must cover v1 upgrades, v2 round trips, typed reference validation, mixed-category compilation, and full backup restore.
