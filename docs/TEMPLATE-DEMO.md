# Seed the template demo resume

The demo is a separate synthetic profile named **Alex Chen**. It includes contact links, a professional summary, three work experiences, education, two projects, seven skills, certifications, languages, an award, volunteer work, a talk, a membership, a license, references, and interests. Optional profile fields are populated with plausible sample values to exercise metadata and link layout. The draft includes the résumé sections and keeps reference contact details excluded. Its IDs use the `template-demo-` prefix; the user's existing profile and drafts are not changed.

On this Linux setup, close the app and run from the repository root:

```sh
cargo run -p resume-storage-sqlite --example seed-template-demo -- ~/.local/share/local.resume.studio.phase0/resume-studio.sqlite3
```

The command validates and upserts the demo through `SqliteResumeRepository::save_workspace`. It is safe to run again to restore the sample after editing it.

Reopen the desktop app, go to **Profile**, select **Alex Chen**, then open **Editor** and choose any of the nine templates. The demo draft starts with Modern Professional.
