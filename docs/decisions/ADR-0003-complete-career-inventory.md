# ADR-0003: Complete career inventory and optional resume sections

**Status:** Accepted  
**Date:** 2026-11-02

## Context

The profile currently stores identity, email, experience, education, projects, skills, and certifications. Common resume facts such as phone and professional links, a profile summary, languages, awards, volunteer work, publications/talks, memberships, licenses, and references are missing. The current nine templates were initially designed around the smaller category set, so adding data without layout rules could create empty rails, headers, or awkward page balance when users omit sections.

## Decision

- Treat CareerProfile as a local career inventory. Add optional contact fields (phone, location, website, LinkedIn, GitHub) and a reusable professional summary.
- Add typed, stable-ID collections for languages, awards, volunteer experience, publications/talks, professional memberships, licenses, references, and interests.
- References may contain third-party contact data. Keep them local, exclude them from newly created drafts by default, and require explicit inclusion in each draft.
- A ResumeDraft decides whether to show the profile summary and may override its text without changing the profile. Other records remain selectable through per-draft section entries and existing inclusion/order/override controls.
- Use additive profile/draft/document schema upgrades. Keep the backup envelope version stable when its outer shape is unchanged; reject unsupported future record versions before modifying stored data.
- A renderer emits only non-empty sections. Two-column layouts collapse to one column when either content rail is absent. Templates must not leave empty headings, rails, dividers, or filler blocks. Each of the nine designs will be reviewed with full, sparse, and section-omission profiles.
- Preserve a single-column ATS-oriented option and selectable text. Do not add arbitrary user HTML or scripts.

## Data categories

Contact and summary fields are optional except the full name. Typed profile sections are experience, education, projects, skills, certifications, languages, awards, volunteer experience, publications/talks, memberships, licenses, references, and interests. Coursework and honors can initially be represented in education details.

## Consequences

- Profile, draft, and semantic document JSON schemas advance to v3; old v1/v2 profile/draft data upgrades with empty new collections and no summary override. The transient VisualDocument contract also advances to v3.
- Existing SQLite table layout remains unchanged because records are versioned JSON payloads.
- UI adds more profile forms and draft inclusion controls; group the forms by contact, career history, and additional qualifications to avoid an unmanageable single page.
- The semantic compiler and each template must handle missing optional fields and empty categories deterministically.
- Backup round-trip, old-backup compatibility, and PDF pagination must be rechecked before Phase 6 acceptance.

## Alternatives considered

- Add only a generic custom section: rejected for this pass because language proficiency, dates, issuers, and links need typed fields and useful template-specific presentation.
- Add every category directly to every template unconditionally: rejected because optional material must remain optional and empty sections harm layout.
- Keep references out of the profile: rejected because the user wants a comprehensive local inventory; instead they are opt-in and excluded by default.
