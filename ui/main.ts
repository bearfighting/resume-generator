import { invoke } from "@tauri-apps/api/core";
import { save } from "@tauri-apps/plugin-dialog";
import stylesheet from "./style.css?inline";
import layoutTokens from "../design/resume-layout.json";
import "./style.css";

interface CareerProfile {
  id: string;
  schema_version: number;
  personal: {
    full_name: string; headline: string | null; email: string | null; phone: string | null;
    location: string | null; website: string | null; linkedin: string | null; github: string | null;
    professional_summary: string | null;
  };
  experiences: Experience[];
  education: Education[];
  projects: Project[];
  skills: Skill[];
  certifications: Certification[];
  languages: Language[]; awards: Award[]; volunteering: VolunteerExperience[];
  publications: Publication[]; memberships: Membership[]; licenses: License[]; references: ProfessionalReference[];
  interests: Interest[];
  updated_at: string;
}
interface Experience { id: string; role: string; organization: string; period: string; summary: string }
interface Education { id: string; institution: string; degree: string | null; field_of_study: string | null; period: string | null; summary: string }
interface Project { id: string; name: string; role: string | null; period: string | null; url: string | null; summary: string }
interface Skill { id: string; name: string; level: string | null; summary: string }
interface Certification { id: string; name: string; issuer: string; period: string | null; credential_url: string | null }
interface Language { id: string; name: string; proficiency: string | null; certification: string | null }
interface Award { id: string; name: string; issuer: string | null; period: string | null; summary: string; url: string | null }
interface VolunteerExperience { id: string; role: string; organization: string; period: string | null; summary: string }
interface Publication { id: string; title: string; venue: string | null; period: string | null; url: string | null; summary: string }
interface Membership { id: string; organization: string; role: string | null; period: string | null }
interface License { id: string; name: string; issuer: string | null; period: string | null; credential_url: string | null }
interface ProfessionalReference { id: string; name: string; role: string | null; organization: string | null; email: string | null; phone: string | null }
interface Interest { id: string; name: string }
type ProfileValue = Partial<Experience> | Partial<Education> | Partial<Project> | Partial<Skill> | Partial<Certification> | Partial<Language> | Partial<Award> | Partial<VolunteerExperience> | Partial<Publication> | Partial<Membership> | Partial<License> | Partial<ProfessionalReference> | Partial<Interest>;
const collectionFields: Record<CollectionKind, Array<[string, string, string]>> = {
  experience: [["role", "Role", "text"], ["organization", "Organization", "text"], ["period", "Period", "text"], ["summary", "Career fact summary", "textarea"]],
  education: [["institution", "Institution", "text"], ["degree", "Degree", "text"], ["field_of_study", "Field of study", "text"], ["period", "Period", "text"], ["summary", "Notes", "textarea"]],
  project: [["name", "Project name", "text"], ["role", "Role", "text"], ["period", "Period", "text"], ["url", "Project URL", "url"], ["summary", "Description", "textarea"]],
  skill: [["name", "Skill", "text"], ["level", "Level", "text"], ["summary", "Notes", "textarea"]],
  certification: [["name", "Certification", "text"], ["issuer", "Issuer", "text"], ["period", "Date / validity", "text"], ["credential_url", "Credential URL", "url"]],
  language: [["name", "Language", "text"], ["proficiency", "Proficiency", "text"], ["certification", "Test / certification", "text"]],
  award: [["name", "Award", "text"], ["issuer", "Issuing organization", "text"], ["period", "Date", "text"], ["summary", "Details", "textarea"], ["url", "Link", "url"]],
  volunteering: [["role", "Role", "text"], ["organization", "Organization", "text"], ["period", "Period", "text"], ["summary", "Contribution", "textarea"]],
  publication: [["title", "Title / talk", "text"], ["venue", "Publication / venue", "text"], ["period", "Date", "text"], ["url", "Link", "url"], ["summary", "Details", "textarea"]],
  membership: [["organization", "Organization", "text"], ["role", "Membership / role", "text"], ["period", "Period", "text"]],
  license: [["name", "License", "text"], ["issuer", "Issuing organization", "text"], ["period", "Date / validity", "text"], ["credential_url", "Credential URL", "url"]],
  reference: [["name", "Name", "text"], ["role", "Role", "text"], ["organization", "Organization", "text"], ["email", "Email", "email"], ["phone", "Phone", "tel"]],
  interest: [["name", "Interest", "text"]],
};
const optionalCollectionFields: Partial<Record<CollectionKind, string[]>> = {
  education: ["degree", "field_of_study", "period"],
  project: ["role", "period", "url"],
  skill: ["level"],
  certification: ["period", "credential_url"],
  language: ["proficiency", "certification"],
  award: ["issuer", "period", "url"],
  volunteering: ["period"],
  publication: ["venue", "period", "url"],
  membership: ["role", "period"],
  license: ["issuer", "period", "credential_url"],
  reference: ["role", "organization", "email", "phone"],
};
interface ResumeDraft {
  id: string; schema_version: number; profile_id: string; title: string; target_role: string | null;
  summary_included: boolean; summary_override: string | null;
  sections: DraftSection[]; template_id: string; template_version: string; updated_at: string;
  style_overrides: { palette_id?: string | null; heading_decoration?: "rule" | "minimal" | null; font_scale_percent: number | null; accent_color: string | null; density: "comfortable" | "compact" | null; font_family: "sans_serif" | "serif" | null };
}
type CollectionKind = "experience" | "education" | "project" | "skill" | "certification" | "language" | "award" | "volunteering" | "publication" | "membership" | "license" | "reference" | "interest";
type SectionKind = CollectionKind | "summary";
interface DraftSection { id: string; kind: CollectionKind; title: string; order: number; entries: DraftEntry[] }
interface DraftEntry { id: string; source_id: string; included: boolean; order: number; summary_override: string | null }
interface ResumeDocument {
  schema_version: number; title: string; target_role: string | null; personal: CareerProfile["personal"];
  sections: Array<{ kind: SectionKind; title: string; entries: SemanticEntry[] }>;
}
interface SemanticEntry { source_id: string; heading: string; metadata: string[]; body: string }
interface VisualEntry { heading: string; metadata: string[]; body: string }
type VisualBlock =
  | { type: "identity"; title: string; subtitle: string | null; eyebrow: string | null; contact: string | null }
  | { type: "section"; kind: SectionKind | null; title: string; entries: VisualEntry[] };
interface VisualDocument {
  schema_version: number; template_id: string; template_version: string; page_size: "a4" | "letter";
  layout: "single_column" | "accent_header" | "compact_stack" | "two_column";
  style: { presentation?: {surface: string; rule: string; dark: string; heading_decoration: "rule" | "minimal" | null} | null; font_family: "sans_serif" | "serif"; body_size_pt: number; heading_scale_percent: number; accent_color: string; density: "comfortable" | "compact" };
  blocks: VisualBlock[];
  diagnostics: Array<{ severity: "info" | "warning"; code: string; message: string; path: string | null }>;
}
interface StyleOverrides {
  font_scale_percent: number | null;
  accent_color: string | null;
  density: "comfortable" | "compact" | null;
  font_family: "sans_serif" | "serif" | null;
}
interface CompiledWorkspace { profile: CareerProfile; draft: ResumeDraft; document: ResumeDocument; visual_document: VisualDocument }
interface PreviewResult { document: ResumeDocument; visual_document: VisualDocument }

const DEFAULT_PROFILE_ID = "profile-local";
const EXPERIENCE_ID = "experience-primary";
const DEFAULT_DRAFT_ID = "resume-primary";
let preview = document.querySelector<HTMLElement>("#resume-preview");
const draftPicker = document.querySelector<HTMLSelectElement>("#draft-picker");
const previewDraftPicker = document.querySelector<HTMLSelectElement>("#preview-draft-picker");
const profilePicker = document.querySelector<HTMLSelectElement>("#profile-picker");
const newDraftProfilePicker = document.querySelector<HTMLSelectElement>("#new-draft-profile-picker");
const draftEmpty = document.querySelector<HTMLElement>("#draft-empty");
let selectedDraftId = DEFAULT_DRAFT_ID;
let selectedProfileId = DEFAULT_PROFILE_ID;
let activeDraftProfile: CareerProfile | undefined;
let previewRequest = 0;
let previewTimer: number | undefined;
let hasSavedDraft = false;
let profileDirty = false;
let draftDirty = false;
let knownDrafts: ResumeDraft[] = [];
let knownProfiles: CareerProfile[] = [];
let currentDraft: ResumeDraft | undefined;
let includeByDefault = true;
const removedFromDraft = new Set<string>();
const entryIdBySource = new Map<string, string>();
const draftEditorState = new Map<string, { included: boolean; override: string }>();
const collectionKinds: CollectionKind[] = ["experience", "education", "project", "skill", "certification", "language", "award", "volunteering", "publication", "membership", "license", "reference", "interest"];
const sectionTitles: Record<CollectionKind, string> = {
  experience: "Experience", education: "Education", project: "Projects", skill: "Skills", certification: "Certifications",
  language: "Languages", award: "Awards", volunteering: "Volunteer Experience", publication: "Publications & Talks",
  membership: "Professional Memberships", license: "Licenses", reference: "References", interest: "Interests",
};

function element<T extends HTMLElement>(selector: string): T | null {
  return document.querySelector<T>(selector);
}
function value(id: string): string {
  return element<HTMLInputElement | HTMLTextAreaElement>("#" + id)?.value ?? "";
}
function setValue(id: string, nextValue: string | null): void {
  const field = element<HTMLInputElement | HTMLTextAreaElement>("#" + id);
  if (field) field.value = nextValue ?? "";
}
function updateDraftSourceLabels(): void {
  const name = activeDraftProfile?.personal.full_name || activeDraftProfile?.id;
  const text = name ? "Source profile: " + name : "Choose a draft to load its source profile.";
  const editorLabel = element<HTMLElement>("#editor-source-context");
  const previewLabel = element<HTMLElement>("#preview-source-context");
  if (editorLabel) editorLabel.textContent = text;
  if (previewLabel) previewLabel.textContent = text;
}
function syncPreviewDraftPicker(): void {
  if (!draftPicker || !previewDraftPicker) return;
  previewDraftPicker.replaceChildren(...Array.from(draftPicker.options, (option) => option.cloneNode(true)));
  previewDraftPicker.value = selectedDraftId;
}
function setStatus(message: string, isError = false, pages?: string[]): void {
  for (const status of document.querySelectorAll<HTMLOutputElement>(".page-status")) {
    if (pages && !pages.includes(status.dataset.statusPage ?? "")) continue;
    status.value = message;
    status.dataset.state = isError ? "error" : "normal";
  }
}
function hasUnsavedChanges(): boolean {
  return profileDirty || draftDirty;
}
function updateSaveButtons(): void {
  const saveProfile = element<HTMLButtonElement>("#save-profile");
  const saveDraft = element<HTMLButtonElement>("#save-draft");
  const saveResume = element<HTMLButtonElement>("#save-resume");
  const exportHtml = element<HTMLButtonElement>("#export-html");
  const exportDocx = element<HTMLButtonElement>("#export-docx");
  if (saveProfile) saveProfile.disabled = !profileDirty;
  if (saveDraft) saveDraft.disabled = !draftDirty;
  if (saveResume) saveResume.disabled = !draftDirty;
  if (exportHtml) exportHtml.disabled = !activeDraftProfile;
  if (exportDocx) exportDocx.disabled = !activeDraftProfile;
  const newDraft = element<HTMLButtonElement>("#new-draft");
  if (newDraft) newDraft.disabled = knownProfiles.length === 0;
}


function setSectionDigest(group: Element | null, text: string): void {
  const summary = group?.querySelector<HTMLElement>(":scope > summary");
  if (!summary) return;
  let digest = summary.querySelector<HTMLElement>(".section-digest");
  if (!digest) {
    digest = document.createElement("span");
    digest.className = "section-digest";
    summary.append(digest);
  }
  digest.textContent = text;
  digest.title = text;
}

function updateSectionDigests(): void {
  const personal = element<HTMLElement>("#full-name")?.closest("details");
  setSectionDigest(personal ?? null, [value("full-name"), value("headline")].filter(v => v.trim()).join(" · ") || "Not filled in");
  const details = element<HTMLElement>("#resume-title")?.closest("details");
  setSectionDigest(details ?? null, [value("resume-title"), value("target-role")].filter(v => v.trim()).join(" · ") || "Not filled in");
  const summaryGroup = element<HTMLElement>("#summary-override")?.closest("details");
  const included = element<HTMLInputElement>("#include-summary")?.checked;
  setSectionDigest(summaryGroup ?? null, !included ? "Hidden" : value("summary-override").trim() ? "Included · Draft-specific summary" : activeDraftProfile?.personal.professional_summary?.trim() ? "Included · Profile summary" : "Included · No summary text");
  const nouns: Record<CollectionKind, [string, string]> = {
    experience: ["experience", "experiences"], education: ["education record", "education records"],
    project: ["project", "projects"], skill: ["skill", "skills"], certification: ["certification", "certifications"],
    language: ["language", "languages"], award: ["award", "awards"], volunteering: ["volunteer role", "volunteer roles"],
    publication: ["publication / talk", "publications / talks"], membership: ["membership", "memberships"],
    license: ["license", "licenses"], reference: ["reference", "references"], interest: ["interest", "interests"],
  };
  for (const kind of collectionKinds) {
    const profileList = element<HTMLElement>("#" + profileListId(kind));
    const cards = Array.from(profileList?.querySelectorAll<HTMLElement>(".profile-item-card") ?? []);
    const filled = cards.filter(card => Array.from(card.querySelectorAll<HTMLInputElement | HTMLTextAreaElement>("[data-field]")).some(input => input.value.trim()));
    const names = filled.slice(0, 2).map(card => card.querySelector<HTMLInputElement>("[data-field]")?.value.trim() || "Untitled");
    const count = filled.length;
    const description = count ? `${count} ${nouns[kind][count === 1 ? 0 : 1]}` : "Not filled in";
    const labels = kind !== "reference" && names.length ? " · " + names.join(", ") + (count > 2 ? "…" : "") : "";
    setSectionDigest(profileList?.closest("details") ?? null, description + labels);
    const draftList = element<HTMLElement>("#" + draftListId(kind));
    const entries = Array.from(draftList?.querySelectorAll<HTMLElement>(".draft-item-card") ?? []);
    const selected = entries.filter(card => {
      const checkbox = card.querySelector<HTMLInputElement>("[data-selected]");
      return checkbox?.checked && !checkbox.disabled;
    }).length;
    setSectionDigest(draftList?.closest("details") ?? null, entries.length ? `${selected} / ${entries.length} selected` : "No source records");
  }
}

function createId(prefix: string): string {
  return typeof crypto.randomUUID === "function"
    ? prefix + "-" + crypto.randomUUID()
    : prefix + "-" + Date.now() + "-" + Math.random().toString(16).slice(2);
}

function createProfileExperience(experience: Partial<Experience> = {}): void {
  createProfileItem("experience", experience);
}

function profileListId(kind: CollectionKind): string {
  return kind === "experience" ? "experience-list" : kind + "-list";
}

function draftListId(kind: CollectionKind): string {
  return "draft-" + (kind === "project" ? "project" : kind) + "-list";
}

function sourceItems(profile: CareerProfile, kind: CollectionKind): Array<Record<string, string | null>> {
  const items = kind === "experience" ? profile.experiences
    : kind === "education" ? profile.education
      : kind === "project" ? profile.projects
        : kind === "skill" ? profile.skills
          : kind === "certification" ? profile.certifications
            : kind === "language" ? profile.languages
              : kind === "award" ? profile.awards
                : kind === "volunteering" ? profile.volunteering
                  : kind === "publication" ? profile.publications
                    : kind === "membership" ? profile.memberships
                      : kind === "license" ? profile.licenses
                        : kind === "reference" ? profile.references
                          : profile.interests;
  return items as unknown as Array<Record<string, string | null>>;
}

function createProfileItem(kind: CollectionKind, data: ProfileValue = {}): void {
  const list = element<HTMLElement>("#" + profileListId(kind));
  if (!list) return;
  const id = data.id ?? createId(kind);
  const card = document.createElement("div");
  card.className = "profile-item-card experience-card";
  card.dataset.kind = kind;
  card.dataset.sourceId = id;
  for (const [name, title, type] of collectionFields[kind]) {
    const fieldControl = document.createElement("div");
    fieldControl.className = "field-control" + (type === "textarea" ? " field-control-wide" : "");
    const label = document.createElement("label");
    label.textContent = title;
    const field = type === "textarea" ? document.createElement("textarea") : document.createElement("input");
    if (field instanceof HTMLInputElement) field.type = type;
    const fieldId = "experience-" + id + "-" + name;
    label.htmlFor = fieldId;
    field.id = fieldId;
    field.dataset.field = name;
    field.setAttribute("aria-label", title);
    const initial = (data as Record<string, string | null | undefined>)[name];
    field.value = typeof initial === "string" ? initial : "";
    if (type === "textarea") (field as HTMLTextAreaElement).rows = 3;
    fieldControl.append(label, field);
    card.append(fieldControl);
  }
  const actions = document.createElement("div");
  actions.className = "experience-actions";
  const remove = document.createElement("button");
  remove.type = "button";
  remove.className = "text-button";
  const referenced = knownDrafts.some((draft) => draft.sections.some((section) =>
    section.kind === kind && section.entries.some((entry) => entry.source_id === id)));
  remove.disabled = referenced;
  remove.title = referenced
    ? "Remove this experience from every saved draft before deleting the career fact."
    : "Delete this career fact";
  remove.textContent = referenced ? "In use by a saved draft" : "Delete " + kind;
  remove.addEventListener("click", () => {
    if (remove.disabled) return;
    card.remove();
    renderDraftContent();
    schedulePreview("profile");
  });
  actions.append(remove);
  card.append(actions);
  list.append(card);
  renderDraftContent();
}

function updateProfileItemDeleteStates(): void {
  for (const card of document.querySelectorAll<HTMLElement>(".profile-item-card")) {
    const kind = card.dataset.kind as CollectionKind | undefined;
    const sourceId = card.dataset.sourceId ?? "";
    const remove = card.querySelector<HTMLButtonElement>(".experience-actions button");
    if (!kind || !remove) continue;
    const referenced = knownDrafts.some((draft) => draft.sections.some((section) =>
      section.kind === kind && section.entries.some((entry) => entry.source_id === sourceId)));
    remove.disabled = referenced;
    remove.title = referenced
      ? "Remove this item from every saved draft before deleting the career fact."
      : "Delete this career fact";
    remove.textContent = referenced ? "In use by a saved draft" : "Delete " + kind;
  }
}

function renderDraftContent(): void {
  for (const existingCard of document.querySelectorAll<HTMLElement>(".draft-item-card")) {
    const sourceId = existingCard.dataset.sourceId ?? "";
    const kind = existingCard.dataset.kind as CollectionKind;
    draftEditorState.set(sourceId, {
      included: existingCard.querySelector<HTMLInputElement>("[data-selected]")?.checked ?? false,
      override: existingCard.querySelector<HTMLTextAreaElement>("[data-override]")?.value ?? "",
    });
    if (kind) draftEditorState.set(kind + ":" + sourceId, draftEditorState.get(sourceId)!);
  }
  for (const kind of collectionKinds) {
    const list = element<HTMLElement>("#" + draftListId(kind));
    const profileItems = activeDraftProfile ? sourceItems(activeDraftProfile, kind) : [];
    if (!list) continue;
    list.replaceChildren();
    if (!profileItems.length) {
      const emptyHint = document.createElement("p");
      emptyHint.className = "draft-empty-hint";
      emptyHint.textContent = "No " + sectionTitles[kind].toLowerCase() + " in this source profile yet. Add them on the Profile page.";
      list.append(emptyHint);
      continue;
    }
    const sourceEntries = currentDraft?.sections.find((section) => section.kind === kind)?.entries ?? [];
    for (const profileItem of profileItems) {
      const sourceId = profileItem.id ?? "";
      const values = collectionFields[kind].map(([field]) => profileItem[field] ?? "");
      const heading = values.slice(0, 2).filter(Boolean).join(" — ") || "Untitled " + kind;
      const existing = sourceEntries.find((entry) => entry.source_id === sourceId);
      const key = kind + ":" + sourceId;
      if (existing && !entryIdBySource.has(key)) entryIdBySource.set(key, existing.id);
      if (!entryIdBySource.has(key)) entryIdBySource.set(key, createId("entry"));
      const removed = removedFromDraft.has(key);
      const editorState = draftEditorState.get(key);
      const card = document.createElement("div");
      card.className = "draft-experience-card draft-item-card";
      card.dataset.kind = kind;
      card.dataset.sourceId = sourceId;
      const label = document.createElement("label");
      label.className = "check-label";
      const checkbox = document.createElement("input");
      checkbox.type = "checkbox";
      checkbox.dataset.selected = "true";
      checkbox.checked = removed ? false : editorState?.included ?? existing?.included ?? (kind !== "reference" && includeByDefault);
      checkbox.disabled = removed;
      const text = document.createElement("span");
      text.dataset.experienceLabel = "true";
      text.textContent = heading;
      label.append(checkbox, text);
      const overrideLabel = document.createElement("label");
      overrideLabel.textContent = "Draft-only text override";
      const override = document.createElement("textarea");
      override.id = "draft-override-" + kind + "-" + sourceId;
      overrideLabel.htmlFor = override.id;
      override.rows = 3;
      override.dataset.override = "true";
      override.value = removed ? "" : editorState?.override ?? existing?.summary_override ?? "";
      override.disabled = removed;
      const overrideField = document.createElement("div");
      overrideField.className = "draft-override-field";
      overrideField.append(overrideLabel, override);
      const controls = document.createElement("div");
      controls.className = "experience-actions";
      const upButton = document.createElement("button");
      upButton.type = "button";
      upButton.className = "text-button";
      upButton.textContent = "Move up";
      upButton.disabled = list.children.length === 0;
      upButton.setAttribute("aria-label", "Move " + heading + " up in this draft");
      upButton.addEventListener("click", () => {
        const previous = card.previousElementSibling;
        if (previous) list.insertBefore(card, previous);
        renderDraftContent();
        schedulePreview();
      });
      const downButton = document.createElement("button");
      downButton.type = "button";
      downButton.className = "text-button";
      downButton.textContent = "Move down";
      downButton.disabled = profileItems.length < 2 || list.children.length === profileItems.length - 1;
      downButton.setAttribute("aria-label", "Move " + heading + " down in this draft");
      downButton.addEventListener("click", () => {
        const next = card.nextElementSibling;
        if (next) list.insertBefore(next, card);
        renderDraftContent();
        schedulePreview();
      });
      const removeButton = document.createElement("button");
      removeButton.type = "button";
      removeButton.className = "text-button";
      removeButton.textContent = removed ? "Restore to this draft" : "Remove from this draft";
      removeButton.addEventListener("click", () => {
        if (removedFromDraft.has(key)) removedFromDraft.delete(key);
        else removedFromDraft.add(key);
        renderDraftContent();
        schedulePreview();
      });
      controls.append(upButton, downButton, removeButton);
      card.append(label, overrideField, controls);
      list.append(card);
    }
  }
  updateSectionDigests();
}

function readProfileForm(): CareerProfile {
  const now = new Date().toISOString();
  const experiences = Array.from(document.querySelectorAll<HTMLElement>("#experience-list .experience-card"))
    .map((card) => ({
      id: card.dataset.sourceId ?? createId("experience"),
      role: card.querySelector<HTMLInputElement>('[data-field="role"]')?.value ?? "",
      organization: card.querySelector<HTMLInputElement>('[data-field="organization"]')?.value ?? "",
      period: card.querySelector<HTMLInputElement>('[data-field="period"]')?.value ?? "",
      summary: card.querySelector<HTMLTextAreaElement>('[data-field="summary"]')?.value ?? "",
    }));
  const readItems = (kind: Exclude<CollectionKind, "experience">) =>
    Array.from(document.querySelectorAll<HTMLElement>("#" + profileListId(kind) + " .profile-item-card"))
      .map((card): Record<string, string | null> => {
        const result: Record<string, string | null> = { id: card.dataset.sourceId ?? createId(kind) };
        for (const [field, , type] of collectionFields[kind]) {
          const raw = card.querySelector<HTMLInputElement | HTMLTextAreaElement>('[data-field="' + field + '"]')?.value.trim() ?? "";
          result[field] = optionalCollectionFields[kind]?.includes(field) && !raw ? null : raw;
        }
        return result;
      });
  return {
    id: selectedProfileId, schema_version: 3,
    personal: {
      full_name: value("full-name"), headline: value("headline").trim() || null, email: value("email").trim() || null,
      phone: value("phone").trim() || null, location: value("location").trim() || null,
      website: value("website").trim() || null, linkedin: value("linkedin").trim() || null,
      github: value("github").trim() || null, professional_summary: value("professional-summary").trim() || null,
    },
    experiences,
    education: readItems("education") as unknown as Education[],
    projects: readItems("project") as unknown as Project[],
    skills: readItems("skill") as unknown as Skill[],
    certifications: readItems("certification") as unknown as Certification[],
    languages: readItems("language") as unknown as Language[],
    awards: readItems("award") as unknown as Award[],
    volunteering: readItems("volunteering") as unknown as VolunteerExperience[],
    publications: readItems("publication") as unknown as Publication[],
    memberships: readItems("membership") as unknown as Membership[],
    licenses: readItems("license") as unknown as License[],
    references: readItems("reference") as unknown as ProfessionalReference[],
    interests: readItems("interest") as unknown as Interest[],
    updated_at: now,
  };
}

function makeWorkspace(): { profile: CareerProfile; draft: ResumeDraft } {
  if (!activeDraftProfile) throw new Error("Select a saved source profile before editing a resume draft.");
  const now = new Date().toISOString();
  const buildSection = (kind: CollectionKind, order: number): DraftSection => {
    const currentSection = currentDraft?.sections.find((section) => section.kind === kind);
    const cards = Array.from(document.querySelectorAll<HTMLElement>(
      "#" + draftListId(kind) + " .draft-item-card",
    ));
    const entries = cards.flatMap((card, index) => {
      const sourceId = card.dataset.sourceId ?? "";
      const key = kind + ":" + sourceId;
      if (removedFromDraft.has(key)) return [];
      const existing = currentSection?.entries.find((entry) => entry.source_id === sourceId);
      const included = card.querySelector<HTMLInputElement>("[data-selected]")?.checked ?? false;
      if (!existing && !included) return [];
      return [{
        id: entryIdBySource.get(key) ?? createId("entry"),
        source_id: sourceId,
        included,
        order: index,
        summary_override: card.querySelector<HTMLTextAreaElement>("[data-override]")?.value.trim() || null,
      }];
    });
    return {
      id: currentSection?.id ?? "section-" + kind,
      kind,
      title: currentSection?.title ?? sectionTitles[kind],
      order: currentSection?.order ?? order,
      entries,
    };
  };
  const draft: ResumeDraft = {
    id: selectedDraftId, schema_version: 3, profile_id: activeDraftProfile.id,
    title: value("resume-title"), target_role: value("target-role").trim() || null,
    summary_included: element<HTMLInputElement>("#include-summary")?.checked ?? false,
    summary_override: value("summary-override").trim() || null,
    sections: collectionKinds.map(buildSection),
    template_id: element<HTMLSelectElement>("#template")?.value ?? "classic",
    template_version: "1.1.0",
    style_overrides: {
      palette_id: value("palette") || null,
      heading_decoration: (value("heading-decoration") || null) as "rule" | "minimal" | null,
      font_scale_percent: Number(value("font-scale")) === 100 ? null : Number(value("font-scale")),
      accent_color: element<HTMLInputElement>("#use-accent")?.checked
        ? value("accent-color").toUpperCase()
        : null,
      density: value("density") === "template"
        ? null
        : value("density") as "comfortable" | "compact",
      font_family: value("font-family") === "template"
        ? null
        : value("font-family") as "sans_serif" | "serif",
    },
    updated_at: now,
  };
  return { profile: activeDraftProfile, draft };
}

function createResumeElement(model: VisualDocument): HTMLElement {
  const resume = document.createElement("article");
  resume.id = "resume-preview";
  resume.setAttribute("aria-label", "Compiled resume preview");
  resume.setAttribute("aria-live", "polite");
  resume.className = "resume template-" + model.template_id + " density-" + model.style.density;
  resume.style.setProperty("--resume-accent", model.style.accent_color);
  resume.style.setProperty("--resume-body-size", model.style.body_size_pt + "pt");
  resume.style.setProperty("--resume-section-size", layoutTokens.sectionSizePt + "pt");
  resume.style.setProperty("--resume-line-height", String(model.style.density === "compact" ? layoutTokens.compactLineHeight : layoutTokens.comfortableLineHeight));
  resume.style.setProperty("--resume-entry-gap", layoutTokens.entryGapMm + "mm");
  resume.style.setProperty("--resume-paragraph-gap", layoutTokens.paragraphGapMm + "mm");
  resume.style.setProperty("--resume-heading-scale", model.style.heading_scale_percent + "%");
  resume.style.setProperty("--resume-heading-size", (model.style.body_size_pt * model.style.heading_scale_percent / 100) + "pt");
  resume.dataset.layout = model.layout;
  const presentation = model.style.presentation;
  resume.dataset.palette = presentation?.surface ? "selected" : "default";
  resume.dataset.decoration = presentation?.heading_decoration ?? "default";
  for (const token of ["surface", "rule", "dark"] as const) {
    if (presentation?.[token]) resume.style.setProperty("--resume-" + token, presentation[token]);
  }
  resume.style.setProperty("--resume-font", model.style.font_family === "serif" ? "Georgia, serif" : "Liberation Sans, Arial, sans-serif");
  const fragment = document.createDocumentFragment();
  const columns = model.layout === "two_column" ? document.createElement("div") : null;
  const mainColumn = columns ? document.createElement("div") : null;
  const sideColumn = columns ? document.createElement("aside") : null;
  if (columns && mainColumn && sideColumn) {
    columns.className = "resume-columns";
    mainColumn.className = "resume-column-main";
    sideColumn.className = "resume-column-side";
  }
  for (const block of model.blocks) {
    if (block.type === "identity") {
      const identity = document.createElement("header");
      identity.className = "resume-identity";
      const name = document.createElement("h2");
      name.textContent = block.title;
      identity.append(name);
      if (block.subtitle) {
        const subtitle = document.createElement("p");
        subtitle.className = "resume-subtitle";
        subtitle.textContent = block.subtitle;
        identity.append(subtitle);
      }
      if (block.eyebrow) {
        const eyebrow = document.createElement("p");
        eyebrow.className = "resume-eyebrow";
        eyebrow.textContent = block.eyebrow;
        identity.append(eyebrow);
      }
      if (block.contact) {
        const contact = document.createElement("p");
        contact.className = "resume-contact";
        contact.textContent = block.contact;
        identity.append(contact);
      }
      fragment.append(identity);
    } else {
      if (!block.entries.length) continue;
      const section = document.createElement("section");
      section.className = "resume-section";
      if (block.kind) section.dataset.kind = block.kind;
      const heading = document.createElement("h3");
      heading.textContent = block.title;
      section.append(heading);
      for (const entry of block.entries) {
        const article = document.createElement("article");
        article.className = "resume-entry";
        const title = document.createElement("h4");
        title.textContent = entry.heading;
        article.append(title);
        if (entry.metadata.length) {
          const metadata = document.createElement("p");
          metadata.className = "resume-metadata";
          metadata.textContent = entry.metadata.join(" · ");
          article.append(metadata);
        }
        if (entry.body.trim()) {
          const body = document.createElement("p");
          body.className = "resume-body";
          body.textContent = entry.body;
          article.append(body);
        }
        section.append(article);
      }
      if (columns && mainColumn && sideColumn) {
        const column = block.kind === "education" || block.kind === "skill" || block.kind === "certification"
          || block.kind === "language" || block.kind === "membership" || block.kind === "license" || block.kind === "reference" || block.kind === "interest"
          ? sideColumn
          : mainColumn;
        column.append(section);
      } else {
        fragment.append(section);
      }
    }
  }
  if (columns && mainColumn && sideColumn) {
    const hasMain = mainColumn.childElementCount > 0;
    const hasSide = sideColumn.childElementCount > 0;
    if (hasMain && hasSide) {
      columns.append(mainColumn, sideColumn);
      fragment.append(columns);
    } else if (hasMain || hasSide) {
      columns.classList.add("is-single-column", hasMain ? "columns-only-main" : "columns-only-side");
      columns.append(hasMain ? mainColumn : sideColumn);
      fragment.append(columns);
    }
  }
  resume.append(fragment);
  return resume;
}

function renderDocument(model: VisualDocument): void {
  if (!preview) return;
  const nextPreview = createResumeElement(model);
  preview.replaceWith(nextPreview);
  preview = nextPreview;
  document.querySelector(".preview-notice")?.remove();
  const warnings = model.diagnostics.filter((item) => item.severity === "warning");
  if (warnings.length) {
    const notice = document.createElement("p");
    notice.className = "preview-notice";
    notice.textContent = warnings.map((item) => item.message).join(" ");
    preview.before(notice);
  }
}

function fillProfileForm(profile: CareerProfile): void {
  selectedProfileId = profile.id;
  if (profilePicker) profilePicker.value = selectedProfileId;
  setValue("full-name", profile.personal.full_name);
  setValue("headline", profile.personal.headline);
  setValue("email", profile.personal.email);
  setValue("phone", profile.personal.phone);
  setValue("location", profile.personal.location);
  setValue("website", profile.personal.website);
  setValue("linkedin", profile.personal.linkedin);
  setValue("github", profile.personal.github);
  setValue("professional-summary", profile.personal.professional_summary);
  for (const kind of collectionKinds) element<HTMLElement>("#" + profileListId(kind))?.replaceChildren();
  for (const experience of profile.experiences) createProfileExperience(experience);
  for (const education of profile.education) createProfileItem("education", education);
  for (const project of profile.projects) createProfileItem("project", project);
  for (const skill of profile.skills) createProfileItem("skill", skill);
  for (const certification of profile.certifications) createProfileItem("certification", certification);
  for (const language of profile.languages) createProfileItem("language", language);
  for (const award of profile.awards) createProfileItem("award", award);
  for (const volunteering of profile.volunteering) createProfileItem("volunteering", volunteering);
  for (const publication of profile.publications) createProfileItem("publication", publication);
  for (const membership of profile.memberships) createProfileItem("membership", membership);
  for (const license of profile.licenses) createProfileItem("license", license);
  for (const reference of profile.references) createProfileItem("reference", reference);
  for (const interest of profile.interests) createProfileItem("interest", interest);
  profileDirty = false;
  updateSaveButtons();
  updateSectionDigests();
}

function fillWorkspace(workspace: CompiledWorkspace): void {
  currentDraft = workspace.draft;
  activeDraftProfile = workspace.profile;
  updateDraftSourceLabels();
  entryIdBySource.clear();
  draftEditorState.clear();
  removedFromDraft.clear();
  setValue("resume-title", workspace.draft.title);
  setValue("target-role", workspace.draft.target_role);
  if (element<HTMLInputElement>("#include-summary")) element<HTMLInputElement>("#include-summary")!.checked = workspace.draft.summary_included;
  setValue("summary-override", workspace.draft.summary_override);
  knownDrafts = knownDrafts.length ? knownDrafts : [workspace.draft];
  const template = element<HTMLSelectElement>("#template");
  if (template) template.value = workspace.draft.template_id;
  void refreshPaletteOptions(workspace.draft.style_overrides.palette_id ?? "");
  setValue("heading-decoration", workspace.draft.style_overrides.heading_decoration ?? "");
  const fontScale = element<HTMLInputElement>("#font-scale");
  if (fontScale) fontScale.value = String(workspace.draft.style_overrides?.font_scale_percent ?? 100);
  const scaleValue = element<HTMLOutputElement>("#font-scale-value");
  if (scaleValue) scaleValue.value = String(workspace.draft.style_overrides?.font_scale_percent ?? 100) + "%";
  const useAccent = element<HTMLInputElement>("#use-accent");
  if (useAccent) useAccent.checked = Boolean(workspace.draft.style_overrides?.accent_color);
  const accentColor = element<HTMLInputElement>("#accent-color");
  if (accentColor && workspace.draft.style_overrides?.accent_color) {
    accentColor.value = workspace.draft.style_overrides.accent_color;
  }
  const density = element<HTMLSelectElement>("#density");
  if (density) density.value = workspace.draft.style_overrides?.density ?? "template";
  const fontFamily = element<HTMLSelectElement>("#font-family");
  if (fontFamily) fontFamily.value = workspace.draft.style_overrides?.font_family ?? "template";
  selectedDraftId = workspace.draft.id;
  includeByDefault = false;
  renderDraftContent();
  draftDirty = false;
  updateSaveButtons();
  renderDocument(workspace.visual_document);
  updateSectionDigests();
}

async function refreshDraftList(): Promise<void> {
  if (!draftPicker) return;
  const drafts = await invoke<ResumeDraft[]>("list_all_drafts");
  knownDrafts = drafts;
  draftPicker.replaceChildren();
  for (const draft of drafts) {
    const option = document.createElement("option");
    option.value = draft.id;
    const sourceName = knownProfiles.find((profile) => profile.id === draft.profile_id)?.personal.full_name || draft.profile_id;
    option.textContent = draft.title + " · " + sourceName;
    draftPicker.append(option);
  }
  if (!drafts.some((draft) => draft.id === selectedDraftId)) {
    if (draftDirty && selectedDraftId) {
      const option = document.createElement("option");
      option.value = selectedDraftId;
      const sourceName = activeDraftProfile?.personal.full_name || activeDraftProfile?.id || "unsaved profile";
      option.textContent = (value("resume-title") || "New resume draft") + " (unsaved) · " + sourceName;
      draftPicker.append(option);
    } else {
      selectedDraftId = drafts[0]?.id ?? "";
    }
  }
  draftPicker.value = selectedDraftId;
  syncPreviewDraftPicker();
  hasSavedDraft = drafts.length > 0;
  if (draftEmpty) draftEmpty.hidden = hasSavedDraft || (draftDirty && Boolean(selectedDraftId));
  updateProfileItemDeleteStates();
}

async function refreshProfileList(): Promise<void> {
  const profiles = await invoke<CareerProfile[]>("list_profiles");
  knownProfiles = profiles;
  if (!profiles.some((profile) => profile.id === selectedProfileId)) {
    selectedProfileId = profiles.find((profile) => profile.id === DEFAULT_PROFILE_ID)?.id
      ?? profiles[0]?.id
      ?? DEFAULT_PROFILE_ID;
  }
  profilePicker?.replaceChildren();
  for (const profile of profiles) {
    const option = document.createElement("option");
    option.value = profile.id;
    option.textContent = profile.personal.full_name || profile.id;
    profilePicker?.append(option);
  }
  if (profilePicker) profilePicker.value = selectedProfileId;
  const newDraftProfileId = newDraftProfilePicker?.value;
  newDraftProfilePicker?.replaceChildren();
  for (const profile of profiles) {
    const option = document.createElement("option");
    option.value = profile.id;
    option.textContent = profile.personal.full_name || profile.id;
    newDraftProfilePicker?.append(option);
  }
  if (newDraftProfilePicker) {
    newDraftProfilePicker.value = profiles.some((profile) => profile.id === newDraftProfileId)
      ? newDraftProfileId!
      : selectedProfileId;
  }
  const selectedProfile = profiles.find((profile) => profile.id === selectedProfileId);
  if (selectedProfile) fillProfileForm(selectedProfile);
  updateSaveButtons();
}

async function reloadWorkspace(draftId = selectedDraftId): Promise<void> {
  setStatus("Loading draft from local SQLite…", false, ["editor", "preview"]);
  try {
    const workspace = await invoke<CompiledWorkspace | null>("load_workspace", { draftId });
    if (!workspace) {
      hasSavedDraft = false;
      setStatus("No saved draft yet. Save a profile, then create a draft from it.", false, ["editor", "preview"]);
      if (draftEmpty) draftEmpty.hidden = false;
      return;
    }
    fillWorkspace(workspace);
    hasSavedDraft = true;
    await refreshDraftList();
    setStatus("Loaded “" + workspace.draft.title + "” and its source profile.", false, ["editor", "preview"]);
  } catch (error) {
    setStatus("Could not load this draft: " + String(error), true, ["editor", "preview"]);
  }
}

async function updatePreview(): Promise<void> {
  const request = ++previewRequest;
  try {
    const workspace = makeWorkspace();
    const result = await invoke<PreviewResult>("preview_workspace", workspace);
    if (request !== previewRequest) return;
    renderDocument(result.visual_document);
    setStatus(draftDirty ? "Preview updated locally · current draft has unsaved changes" : "Preview updated locally", false, ["editor", "preview"]);
  } catch (error) {
    if (request === previewRequest) setStatus("Preview needs attention: " + String(error), true, ["editor", "preview"]);
  }
}
function schedulePreview(kind: "profile" | "draft" | null = "draft"): void {
  updateSectionDigests();
  if (kind === "profile") {
    profileDirty = true;
    updateSaveButtons();
    setStatus("Unsaved profile changes.", false, ["profile"]);
    return;
  }
  if (kind === "draft") {
    draftDirty = true;
    updateSaveButtons();
    setStatus("Unsaved draft changes · updating preview…", false, ["editor", "preview"]);
  }
  window.clearTimeout(previewTimer);
  previewTimer = window.setTimeout(() => void updatePreview(), 250);
}

async function saveProfile(): Promise<void> {
  const button = element<HTMLButtonElement>("#save-profile");
  if (button) button.disabled = true;
  setStatus("Saving profile…", false, ["profile"]);
  try {
    const profile = readProfileForm();
    await invoke("save_profile", { profile });
    knownProfiles = knownProfiles.map((known) => known.id === profile.id ? profile : known);
    if (!knownProfiles.some((known) => known.id === profile.id)) knownProfiles.push(profile);
    profileDirty = false;
    if (activeDraftProfile?.id === profile.id) {
      activeDraftProfile = profile;
      renderDraftContent();
      await updatePreview();
    }
    await refreshProfileList();
    await refreshDraftList();
    updateSaveButtons();
    setStatus("Profile saved. The current resume updates only when it uses this profile.", false, ["profile"]);
  } catch (error) {
    setStatus("Profile save failed. Your edits are still on screen: " + String(error), true, ["profile"]);
  }
  updateSaveButtons();
}

async function saveDraft(): Promise<void> {
  const button = element<HTMLButtonElement>("#save-draft");
  if (button) button.disabled = true;
  setStatus("Saving draft…", false, ["editor"]);
  try {
    const workspace = makeWorkspace();
    await invoke("save_draft", { draft: workspace.draft });
    currentDraft = workspace.draft;
    draftDirty = false;
    hasSavedDraft = true;
    await refreshDraftList();
    updateSaveButtons();
    setStatus("Draft saved. Its source profile remains unchanged.", false, ["editor", "preview"]);
  } catch (error) {
    setStatus("Draft save failed. Your edits are still on screen: " + String(error), true, ["editor"]);
  }
  updateSaveButtons();
}

async function saveResume(): Promise<void> {
  const button = element<HTMLButtonElement>("#save-resume");
  if (button) button.disabled = true;
  setStatus("Saving resume draft and its source profile…", false, ["preview"]);
  try {
    const workspace = makeWorkspace();
    await invoke<ResumeDocument>("save_workspace", workspace);
    const output = await invoke<PreviewResult>("preview_workspace", workspace);
    renderDocument(output.visual_document);
    currentDraft = workspace.draft;
    await refreshDraftList();
    hasSavedDraft = true;
    draftDirty = false;
    updateSaveButtons();
    setStatus("Resume and its source profile saved to local storage.", false, ["preview"]);
  } catch (error) {
    setStatus("Resume save failed. Your edits are still on screen: " + String(error), true, ["preview"]);
  }
  updateSaveButtons();
}

function exportFilenameBase(title: string): string {
  return title.trim().replace(/[<>:"/\\|?*\u0000-\u001f]/g, "-").replace(/\s+/g, " ").slice(0, 100) || "resume";
}

async function exportResume(format: "html" | "docx"): Promise<void> {
  const button = element<HTMLButtonElement>(format === "html" ? "#export-html" : "#export-docx");
  if (button) button.disabled = true;
  try {
    const workspace = makeWorkspace();
    const output = await invoke<PreviewResult>("preview_workspace", workspace);
    const pageSize = element<HTMLSelectElement>("#paper-size")?.value ?? "A4";
    const visualDocument = {
      ...output.visual_document,
      page_size: pageSize.toLowerCase() as "a4" | "letter",
    };
    const filename = exportFilenameBase(workspace.draft.title || workspace.profile.personal.full_name);
    const docxMode = value("docx-mode") === "design" ? "design" : "standard";
    const path = await save({
      defaultPath: filename + (format === "html" ? ".zip" : `-${docxMode}.docx`),
      filters: [format === "html"
        ? { name: "Static HTML + CSS bundle", extensions: ["zip"] }
        : { name: "Editable Word document", extensions: ["docx"] }],
    });
    if (!path) return;
    if (format === "html") {
      const css = stylesheet + `\n@page { size: ${pageSize}; margin: 18mm; }\n`;
      await invoke("export_html_bundle", {
        path,
        title: workspace.draft.title,
        document: visualDocument,
        css,
      });
      setStatus("Static HTML + CSS ZIP exported locally.", false, ["preview"]);
    } else {
      await invoke("export_docx", { path, document: visualDocument, mode: docxMode });
      setStatus(docxMode === "standard" ? "Standard DOCX exported with a single-column editable layout." : "Experimental design DOCX exported. Review spacing and page breaks before sharing.", false, ["preview"]);
    }
  } catch (error) {
    setStatus(`Resume ${format.toUpperCase()} export failed: ${String(error)}`, true, ["preview"]);
  } finally {
    updateSaveButtons();
  }
}

function showPage(page: string): void {
  for (const button of document.querySelectorAll<HTMLButtonElement>(".tab")) {
    const active = button.dataset.page === page;
    button.classList.toggle("active", active);
    if (active) button.setAttribute("aria-current", "page");
    else button.removeAttribute("aria-current");
  }
  for (const section of document.querySelectorAll<HTMLElement>(".page")) {
    const active = section.id === "page-" + page;
    section.hidden = !active;
    section.classList.toggle("active", active);
  }
}

function createDraft(): void {
  const sourceProfileId = newDraftProfilePicker?.value ?? "";
  const sourceProfile = knownProfiles.find((profile) => profile.id === sourceProfileId);
  if (!sourceProfile) {
    setStatus("Save a profile before creating a draft.", true, ["editor"]);
    return;
  }
  if (draftDirty && !window.confirm("Discard unsaved changes to the current draft and create a new one?")) return;
  if (profileDirty && selectedProfileId === sourceProfileId) {
    setStatus("This draft will use the last saved profile. Save the Profile page first to include its edits.", false, ["profile", "editor"]);
  }
  selectedDraftId = typeof crypto.randomUUID === "function"
    ? crypto.randomUUID()
    : "resume-" + Date.now() + "-" + Math.random().toString(16).slice(2);
  setValue("resume-title", "New resume draft");
  setValue("target-role", "");
  if (element<HTMLInputElement>("#include-summary")) {
    element<HTMLInputElement>("#include-summary")!.checked = Boolean(sourceProfile.personal.professional_summary?.trim());
  }
  setValue("summary-override", "");
  if (element<HTMLInputElement>("#font-scale")) element<HTMLInputElement>("#font-scale")!.value = "100";
  if (element<HTMLInputElement>("#use-accent")) element<HTMLInputElement>("#use-accent")!.checked = false;
  if (element<HTMLSelectElement>("#density")) element<HTMLSelectElement>("#density")!.value = "template";
  if (element<HTMLSelectElement>("#font-family")) element<HTMLSelectElement>("#font-family")!.value = "template";
  setValue("palette", "");
  setValue("heading-decoration", "");
  currentDraft = undefined;
  activeDraftProfile = sourceProfile;
  updateDraftSourceLabels();
  removedFromDraft.clear();
  entryIdBySource.clear();
  draftEditorState.clear();
  includeByDefault = true;
  renderDraftContent();
  if (draftPicker) {
    const option = document.createElement("option");
    option.value = selectedDraftId;
    option.textContent = "New resume draft (unsaved) · " + (sourceProfile.personal.full_name || sourceProfile.id);
    draftPicker.add(option);
    draftPicker.value = selectedDraftId;
  }
  syncPreviewDraftPicker();
  if (draftEmpty) draftEmpty.hidden = true;
  draftDirty = false;
  setStatus("New draft created from “" + (sourceProfile.personal.full_name || sourceProfile.id) + "”.", false, ["editor"]);
  schedulePreview("draft");
  showPage("editor");
}

for (const button of document.querySelectorAll<HTMLButtonElement>(".tab")) {
  button.addEventListener("click", () => showPage(button.dataset.page ?? "profile"));
}
for (const selector of [
  "#full-name", "#headline", "#email", "#phone", "#location", "#website", "#linkedin", "#github",
  "#professional-summary",
]) {
  element<HTMLInputElement | HTMLTextAreaElement>(selector)?.addEventListener("input", () => schedulePreview("profile"));
  element<HTMLInputElement | HTMLTextAreaElement>(selector)?.addEventListener("change", () => schedulePreview("profile"));
}
for (const selector of [
  "#include-summary", "#summary-override", "#resume-title", "#target-role", "#template",
  "#font-scale", "#use-accent", "#accent-color", "#density", "#font-family", "#palette", "#heading-decoration",
]) {
  element<HTMLInputElement | HTMLTextAreaElement>(selector)?.addEventListener("input", () => schedulePreview("draft"));
  element<HTMLInputElement | HTMLTextAreaElement>(selector)?.addEventListener("change", () => schedulePreview("draft"));
}
element<HTMLInputElement>("#font-scale")?.addEventListener("input", (event) => {
  const output = element<HTMLOutputElement>("#font-scale-value");
  if (output) output.value = (event.currentTarget as HTMLInputElement).value + "%";
});
for (const listSelector of collectionKinds.flatMap((kind) => [
  "#" + profileListId(kind),
  "#" + draftListId(kind),
])) {
  const list = element<HTMLElement>(listSelector);
  list?.addEventListener("input", (event) => {
    const target = event.target as HTMLElement;
    if (listSelector.startsWith("#draft-")) {
      const card = target.closest<HTMLElement>(".draft-item-card");
      const kind = card?.dataset.kind as CollectionKind | undefined;
      const sourceId = card?.dataset.sourceId;
      if (card && kind && sourceId) {
        draftEditorState.set(kind + ":" + sourceId, {
          included: card.querySelector<HTMLInputElement>("[data-selected]")?.checked ?? false,
          override: card.querySelector<HTMLTextAreaElement>("[data-override]")?.value ?? "",
        });
      }
    } else {
      renderDraftContent();
    }
    schedulePreview(listSelector.startsWith("#draft-") ? "draft" : "profile");
  });
  list?.addEventListener("change", () => schedulePreview(listSelector.startsWith("#draft-") ? "draft" : "profile"));
}
element<HTMLButtonElement>("#add-experience")?.addEventListener("click", () => {
  createProfileExperience();
  schedulePreview("profile");
});
for (const button of document.querySelectorAll<HTMLButtonElement>(".add-profile-item")) {
  button.addEventListener("click", () => {
    const kind = button.dataset.kind as CollectionKind | undefined;
    if (kind && kind !== "experience") {
      createProfileItem(kind);
      schedulePreview("profile");
    }
  });
}
element<HTMLButtonElement>("#save-profile")?.addEventListener("click", () => void saveProfile());
element<HTMLButtonElement>("#save-draft")?.addEventListener("click", () => void saveDraft());
element<HTMLButtonElement>("#save-resume")?.addEventListener("click", () => void saveResume());
element<HTMLButtonElement>("#export-html")?.addEventListener("click", () => void exportResume("html"));
element<HTMLButtonElement>("#export-docx")?.addEventListener("click", () => void exportResume("docx"));
element<HTMLButtonElement>("#expand-profile-groups")?.addEventListener("click", () => {
  for (const group of document.querySelectorAll<HTMLDetailsElement>(".profile-group")) group.open = true;
});
element<HTMLButtonElement>("#collapse-profile-groups")?.addEventListener("click", () => {
  for (const group of document.querySelectorAll<HTMLDetailsElement>(".profile-group")) group.open = false;
});
element<HTMLButtonElement>("#expand-editor-groups")?.addEventListener("click", () => {
  for (const group of document.querySelectorAll<HTMLDetailsElement>(".editor-group")) group.open = true;
});
element<HTMLButtonElement>("#collapse-editor-groups")?.addEventListener("click", () => {
  for (const group of document.querySelectorAll<HTMLDetailsElement>(".editor-group")) group.open = false;
});
element<HTMLButtonElement>("#reload-workspace")?.addEventListener("click", () => {
  if (draftDirty && !window.confirm("Discard unsaved draft changes and reload it?")) return;
  void reloadWorkspace();
});
element<HTMLButtonElement>("#new-draft")?.addEventListener("click", createDraft);
profilePicker?.addEventListener("change", () => {
  const nextProfile = knownProfiles.find((profile) => profile.id === profilePicker.value);
  if (!nextProfile) return;
  if (profileDirty && !window.confirm("Discard unsaved changes to this profile and open another profile?")) {
    profilePicker.value = selectedProfileId;
    return;
  }
  fillProfileForm(nextProfile);
  setStatus("Editing “" + (nextProfile.personal.full_name || nextProfile.id) + "”. The current resume is unchanged.", false, ["profile"]);
});
function selectDraft(nextDraftId: string): void {
  if (draftDirty && !window.confirm("Discard unsaved draft changes and switch drafts?")) {
    if (draftPicker) draftPicker.value = selectedDraftId;
    if (previewDraftPicker) previewDraftPicker.value = selectedDraftId;
    return;
  }
  selectedDraftId = nextDraftId;
  if (draftPicker) draftPicker.value = selectedDraftId;
  if (previewDraftPicker) previewDraftPicker.value = selectedDraftId;
  if (selectedDraftId) void reloadWorkspace(selectedDraftId);
}
draftPicker?.addEventListener("change", () => selectDraft(draftPicker.value));
previewDraftPicker?.addEventListener("change", () => selectDraft(previewDraftPicker.value));
element<HTMLButtonElement>("#print")?.addEventListener("click", () => window.print());
element<HTMLButtonElement>("#export-backup")?.addEventListener("click", async () => {
  try {
    const path = await save({
      defaultPath: "resume-backup.json",
      filters: [{ name: "JSON backup", extensions: ["json"] }],
    });
    if (!path) return;
    await invoke("export_backup", { path });
    setStatus("Backup exported locally.", false, ["settings"]);
  } catch (error) {
    setStatus("Backup export failed: " + String(error), true, ["settings"]);
  }
});
element<HTMLInputElement>("#import-backup")?.addEventListener("change", (event) => {
  const input = event.currentTarget as HTMLInputElement;
  const file = input.files?.[0];
  const selection = element<HTMLElement>("#import-backup-selection");
  const importButton = element<HTMLButtonElement>("#confirm-import-backup");
  if (!file) {
    if (selection) selection.textContent = "No file selected.";
    if (importButton) importButton.disabled = true;
    return;
  }
  const isTooLarge = file.size > 50 * 1024 * 1024;
  if (selection) {
    selection.textContent = isTooLarge
      ? `${file.name} is too large (maximum 50 MiB).`
      : `Selected: ${file.name} (${(file.size / 1024).toFixed(1)} KiB)`;
  }
  if (importButton) importButton.disabled = isTooLarge;
});
element<HTMLButtonElement>("#confirm-import-backup")?.addEventListener("click", async () => {
  const input = element<HTMLInputElement>("#import-backup");
  const file = input?.files?.[0];
  if (!file) {
    setStatus("Choose a backup file before importing.", true, ["settings"]);
    return;
  }
  if (file.size > 50 * 1024 * 1024) {
    setStatus("Backup import failed: the file is larger than the 50 MiB limit.", true, ["settings"]);
    return;
  }
  const unsavedWarning = hasUnsavedChanges() ? " Unsaved page edits will be discarded." : "";
  if (!window.confirm("Importing a backup replaces all current profiles and drafts." + unsavedWarning + " Continue?")) return;
  const button = element<HTMLButtonElement>("#confirm-import-backup");
  if (button) button.disabled = true;
  setStatus("Validating and importing backup…", false, ["settings"]);
  try {
    const json = await file.text();
    await invoke("import_backup_json", { json, strategy: "replace" });
    selectedDraftId = "";
    currentDraft = undefined;
    activeDraftProfile = undefined;
    updateDraftSourceLabels();
    profileDirty = false;
    draftDirty = false;
    updateSaveButtons();
    await refreshProfileList();
    await refreshDraftList();
    if (hasSavedDraft) {
      await reloadWorkspace(selectedDraftId);
      setStatus("Backup imported and local data reloaded.", false, ["settings"]);
    } else {
      setStatus("Backup imported. It contains no resume drafts.", false, ["settings"]);
    }
    if (input) input.value = "";
    const selection = element<HTMLElement>("#import-backup-selection");
    if (selection) selection.textContent = "No file selected.";
  } catch (error) {
    setStatus("Backup import failed. Existing local data was preserved: " + String(error), true, ["settings"]);
  } finally {
    if (button) button.disabled = !input?.files?.length;
  }
});
element<HTMLSelectElement>("#paper-size")?.addEventListener("change", (event) => {
  const paper = (event.currentTarget as HTMLSelectElement).value;
  const style = element<HTMLStyleElement>("#page-size-style");
  if (style) style.textContent = "@page { size: " + paper + "; margin: 18mm; }";
});

void refreshProfileList().then(() => refreshDraftList()).then(() => {
  if (hasSavedDraft) return reloadWorkspace(selectedDraftId);
  if (knownProfiles.length === 0) {
    createProfileExperience({
      id: EXPERIENCE_ID,
      role: "Staff Engineer",
      organization: "Northstar Systems",
      period: "2021–2026",
      summary: "Led a team building reliable offline desktop software.",
    });
    profileDirty = true;
    updateSaveButtons();
    setStatus("Create your first saved career profile before making a draft.", false, ["profile", "editor"]);
  } else {
    setStatus("No saved drafts yet. Choose a source profile and create a draft.", false, ["editor"]);
  }
  if (draftEmpty) draftEmpty.hidden = false;
}).catch((error: unknown) => setStatus("Could not load the draft list: " + String(error), true));

interface Palette { id: string; name: string; accent: string; surface: string; rule: string; dark: string }
let paletteRequest = 0;
async function refreshPaletteOptions(selected = value("palette")): Promise<void> {
  const request = ++paletteRequest;
  const picker = element<HTMLSelectElement>("#palette");
  if (!picker) return;
  if (selected && !Array.from(picker.options).some(option => option.value === selected)) {
    picker.add(new Option("Loading palette…", selected));
  }
  picker.value = selected;
  try {
    const palettes = await invoke<Palette[]>("list_template_palettes", { templateId: value("template") });
    if (request !== paletteRequest) return;
    picker.replaceChildren(new Option("Template default", ""));
    for (const palette of palettes) picker.add(new Option(palette.name, palette.id));
    picker.value = palettes.some(p => p.id === selected) ? selected : "";
    const swatches = element<HTMLElement>("#palette-swatches");
    swatches?.replaceChildren();
    for (const palette of palettes) {
      const button = document.createElement("button");
      button.type = "button";
      button.className = "palette-swatch";
      button.style.background = `linear-gradient(90deg, ${palette.accent} 0 50%, ${palette.surface} 50% 75%, ${palette.dark} 75%)`;
      button.title = palette.name;
      button.setAttribute("aria-label", palette.name);
      button.setAttribute("aria-pressed", String(picker.value === palette.id));
      button.addEventListener("click", () => {
        picker.value = palette.id;
        picker.dispatchEvent(new Event("change", { bubbles: true }));
      });
      swatches?.append(button);
    }
  } catch (error) { setStatus("Could not load palettes: " + String(error), true, ["preview"]); }
}
element<HTMLSelectElement>("#template")?.addEventListener("change", () => void refreshPaletteOptions());
element<HTMLSelectElement>("#palette")?.addEventListener("change", () => void refreshPaletteOptions());
