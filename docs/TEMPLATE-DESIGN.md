# Resume template design direction

## Product goal

Templates should make a reader find role, organization, dates, and achievements quickly. Use visual design to clarify hierarchy and improve reading rhythm; do not make decoration compete with the candidate's experience. Design from synthetic data and keep user facts out of screenshots and fixtures.

## Research principles

- Use a straightforward, readable layout with consistent section headings and spacing.
- Use bold and color sparingly to direct attention to meaningful information.
- Check that typography and layout still work when printed or converted to PDF.
- Keep an ATS-oriented single-column option with ordinary section headings and selectable text.
- Treat external templates as references for general patterns only. Do not copy a proprietary layout, graphic, or source file.

These principles are informed by [MIT Career Advising's resume guidance](https://capd.mit.edu/resources/career-toolkit-crafting-an-effective-resume/) and [Harvard's resume guide](https://careerservices.fas.harvard.edu/resources/create-a-strong-resume/).

## Candidate template roles

The user's reference sheet suggests eight useful audience/style directions. These are original adaptations, not exact replicas. A ninth option adds a reversed sidebar variation:

1. **Modern Professional** — strong name and role hierarchy, restrained blue accent, readable single-column chronology.
2. **Creative / Design** — expressive type pairing and a subtle color field; portfolio imagery remains optional until the profile model supports media.
3. **Academic / Research** — publication-friendly density, explicit dates, education and research work easy to scan.
4. **Executive / Leadership** — confident dark header, summary-led opening, achievements remain text-based instead of icon cards or charts.
5. **Classic / Traditional** — restrained editorial typography, fine rules, and a conventional single-column flow.
6. **Two-Column / Modern** — a visual option with a narrow skills/contact rail and a primary experience column; clearly distinguish it from ATS-oriented single-column output.
7. **Creative Minimal** — generous whitespace, a small accent marker, and a concise summary callout.
8. **Modern Business** — polished corporate typography, subtle banner/color, and balanced section rhythm.
9. **Two-Column / Left Rail** — move skills, education, and certifications into a dark left rail; keep experience and projects in the wider right column.

Keep dates, section titles, and content in ordinary selectable text. Do not require profile photos, icons, charts, or manually configured infographic values. At least one single-column design must remain the recommended compatibility-oriented option. Avoid reducing font size to force an arbitrary page count.

The first three-style synthetic study is available at [`template-gallery/index.html`](template-gallery/index.html). Education and Skills are top-level sections, like Experience and Selected project. The [eight-direction board](template-gallery/eight-directions.html) adapts the supplied reference sheet; the left-rail option is an additional variant built from that design direction. All nine templates are now in the application; typography, spacing, page flow, and cross-export fidelity remain to be refined from desktop review.

## Expanded content and sparse profiles

The profile may contain more material than any one resume should show. Each draft owns its summary choice, selected entries, and ordering. The compiler omits a section when it has no included entries; the renderer omits empty body paragraphs. Templates must not reserve space for omitted sections.

| Template | Content treatment |
|---|---|
| Modern Professional | Single-column chronology; summary callout; skills and languages flow in compact text lines. |
| Creative / Design | Expressive summary panel, two-column skill cards, and compact language badges. |
| Academic / Research | Publication-friendly single column; education and publications stay easy to scan; skills use two columns. |
| Executive / Leadership | Summary-led opening and horizontal capability rows; achievements and leadership remain text-first. |
| Classic / Traditional | Conventional single-column history; compact two-column skills and inline language details. |
| Two-Column / Modern | Experience, projects, awards, volunteer work, and publications in the main column; education, skills, credentials, languages, affiliations, references, and interests in the right rail. |
| Two-Column / Left Rail | The same semantic grouping with the narrow information rail on the left and a distinct dark treatment. |
| Creative Minimal | Restrained summary accent and an unboxed two-column skills list. |
| Modern Business | Corporate summary panel and compact three-column skills grid. |

For either two-column template, a missing main or side column causes the remaining content to expand as a single column. References remain excluded from newly created drafts and require an explicit user choice.

## Export behavior

- PDF and standalone HTML should share the same HTML structure, template styles, and content mapping so print and browser output remain close.
- Static HTML export should be a local bundle (HTML plus CSS, packaged together), use no JavaScript or remote assets, and include print styles.
- DOCX should reuse semantic content and template tokens through a dedicated editable document renderer. Preserve text, order, headings, lists, and links; do not promise pixel-identical pagination between Word/LibreOffice and WebKit.
- Keep a readable single-column layout for the compatibility-oriented template. Treat decorative columns, text boxes, and icon-only labels as optional design features, not the primary content structure.
