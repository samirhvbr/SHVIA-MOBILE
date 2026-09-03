# ShvIA Mobile — Instruções para Claude Code

> **Leia também:** [README.md](README.md) · [.continue/escopo-mobile.md](.continue/escopo-mobile.md)
> (roadmap M0–M5) · [docs/decisoes.md](docs/decisoes.md) (ADRs).
>
> `AGENTS.md` e `CLAUDE.md` são o MESMO texto abaixo do H1 — editou um, edite o outro.
> O teste `agents_e_claude_sao_espelho` reprova a divergência (achado F-21).

---

## 🔄 Antes de começar: `git pull`

**SEMPRE** verifique atualizações remotas antes de escrever ou alterar qualquer
coisa neste repositório:

```bash
git pull          # já está pré-autorizado (allow)
```

Trabalhar sobre base desatualizada gera conflito. Puxe primeiro, sempre.

---

## O que é este repo

Cliente **mobile (iOS + Android)** do **ShvIA** — a plataforma de IA da Blue3 em
`https://ai.shvia.org`. **Shell fino Tauri 2:** a WebView nativa (WKWebView no
iOS, System WebView no Android) carrega o ShvIA web (Blade) remoto; o servidor é a
fonte da verdade. **Irmão** do `SHVIA-DESKTOP` (desktop) e do `SHVIA` (servidor
Laravel) — mesmo produto, repos separados (convenção SSHVTERM-DESKTOP/-MOBILE).

Reusa a **infra de loja da Blue3** (já publica o app Flutter `BLUE3-INTRANET-MOBILE`):
conta Apple Developer (**Team ID `S65UBCTPN5`**), Google Play Console, custódia de
keystore. O *build* aqui é Tauri (não Flutter) — reusa identidades, não o pipeline.

---

## Padrão de Commits (obrigatório)

Formato: `X.Y.Z - description in English (US)`. A versão **sempre** vem de `version.md`
(bumpe no mesmo commit; `scripts/sync-version.mjs` propaga p/ package.json /
tauri.conf.json / Cargo.*). Critério: **Z** = UI/tela/ícone/ajuste de build; **Y** =
nova capacidade de runtime/IPC; **X** = estável. Proibido `feat:`/`fix:`/`chore:`.

---

## Mobile-only (não copiar hábitos do desktop)

- **Sem** menu nativo, multi-janela ou geometria de janela — isso é desktop.
- **TTS** = `speechSynthesis` nativo do WebView (iOS/Android têm voz pt-BR); **não**
  há a ponte espeak do Linux/WebKitGTK (ADR-009 do desktop não se aplica aqui).
- **iOS só builda em macOS + Xcode.** Android builda no Linux.
- **App ID/bundle:** `cloud.blue3.shvia` (mesmo do desktop — consistência).
- Stores exigem **build number monotônico** (iOS `CFBundleVersion` / Android
  `versionCode`) — além do `version.md`.

---

## Stack & comandos

- **Tauri 2** (Rust) + WebView nativo do SO. Casca web mínima (Vite/TS).
- `npm run tauri android dev` · `npm run tauri ios dev` (no Mac) · `cargo check`
  (host) · `npm run tauri [android|ios] init` (gera `src-tauri/gen/*`).

---

## Servidor remoto é a fonte da verdade

Nenhum banco/segredo no cliente. Auth = **cookie de sessão same-origin** (a WebView
navega o FQDN real; o login é a tela do próprio ShvIA). Chaves de assinatura/keystore
vivem em secrets/custódia, **nunca** versionadas.

---

## Referências rápidas

- Versão: `version.md` · Roadmap/fases: [.continue/escopo-mobile.md](.continue/escopo-mobile.md)
- Decisões: [docs/decisoes.md](docs/decisoes.md) (ADR)
- Servidor/fonte da verdade: `~/x/SHVIA/SHVIA-WEB` (`https://ai.shvia.org`)
- Desktop irmão: `~/x/SHVIA/SHVIA-DESKTOP` · Base de reuso de loja:
  `~/x/BLUE3/BLUE3-INTRANET-MOBILE`

---

<!-- COMMIT-RULE:repodocs -->

## Commits — you commit, and nothing is delivered until you have

> Marked echo. The single source is **[samirhvbr/repodocs](https://github.com/samirhvbr/repodocs/blob/master/docs/versioning.md#who-commits-and-when)**
> — change it there, not here. This block is regenerated.

**Committing is your job.** Not "leave the tree ready and something downstream
packages it" — you run `git commit`, and `git push`, as the last step of the work
you were asked to do. The COMMITTER skill that used to commit on an agent's
behalf is `enabled: false` in every repository of this fleet since 03/09/2026;
what is left of it is a kill-switch, not a scheduler. **If you do not commit,
nobody does.**

**Do not report a task as finished before the commit exists.** "Done",
"delivered", "concluded" mean the work is in `git log` — never that it is sitting
uncommitted where only this session can see it. The commit is the last step *of
the task*, not a follow-up for someone else. If you are about to write
"finished", commit first, then write it.

**Every commit obeys the versioning rules**, with no exception:

- Subject `X.Y.Z - short description in English (US)`, the version taken from
  `version.md` and **bumped in the same commit**.
- The `CHANGELOG.md` entry is written first — its `## X.Y.Z - description`
  heading *is* the subject.
- No Conventional Commits prefix (`feat:`, `fix:`, `chore:`) and no vague
  subject ("update", "ajuste", "wip", "changes", "several improvements").

**The bump is the one clause a repository may override — in writing.** If this
repository's own documentation says the version is stamped some other way, and says
why, follow that. Otherwise the line above applies to you. An override nobody wrote
down is not an exception. Nothing else in this block bends: the changelog entry, the
subject, the language, one subject per commit, and committing before you report done
all hold regardless.

**One subject per commit.** The subject has to describe the whole commit
honestly. The moment your description needs an "and" to be true, it is two
commits.

**Split a large delivery into blocks.** A complex task is committed as a series
of commits grouped by subject, each small enough to be described in one line and
read on its own. They may share a version — bump `version.md` in the first and
repeat the number in the rest; two commits carrying one version is expected, not
a mistake. **Splitting is the default** for anything non-trivial, because the
history is the documentation of *how* the work was done, and one commit touching
six unrelated subjects documents none of them.

**The standard you are keeping:** someone reading `git log` alone — a year from
now, without the conversation that produced the work — can say what happened,
when, why, and at which version. If your commit would fail that test, it is too
big or its subject is too vague, and both are fixed the same way.

<!-- /COMMIT-RULE -->

---

<!-- RELEASES-RULE:repodocs -->

## Releases — the `version.md` on GitHub is what the Releases show

> Marked echo. The single source is **[samirhvbr/repodocs](https://github.com/samirhvbr/repodocs/blob/master/docs/versioning.md)**
> — change it there, not here. This block is regenerated.

**The `version.md` of the default branch, on GitHub, is what the GitHub Releases
must show.** The local checkout does not enter the calculation: it can be behind,
ahead or mid-work, and none of that is published — GitHub cannot tag a commit it
does not have.

**The bump and the Release are one act.** A commit that bumps `version.md` is not
finished until that version has a tag, a published Release, and the **`Latest`
badge on it** — the same push, not "later". A badge sitting on an older release
tells whoever looks that the project is at a version it is not.

- `.github/workflows/release.yml` does it on any push that touches `version.md`.
- `./tools/release.sh` does it by hand. It is **idempotent and self-healing**:
  it publishes whatever is missing and moves a drifted badge back. Running it is
  always safe, so it is both the check and the fix.

A PR publishes nothing while it is a PR. The moment it merges, the push moves
`version.md` on the default branch and the Release becomes that version.

Tag and Release title are the **bare version — no `v` prefix**.

## Language — English (US), everywhere in the repository

**Everything that lives in this repository, or in GitHub's interface around it,
is written in English (US)**: documents, **commit messages**, pull request titles
and bodies, issues, code comments, changelog entries, release notes.

Commit format: `X.Y.Z - short description in English`. The version comes from
`version.md` and is bumped in the same commit. Conventional Commits prefixes
(`feat:`, `fix:`, `chore:`) and vague one-word messages are forbidden.

**Exactly one carve-out:** end-user-facing strings — UI text, transactional
email, product copy. That is product i18n for a Brazilian audience, not
repository content.

History is not rewritten: Portuguese messages already in the log stay as they
are.

<!-- /RELEASES-RULE -->
