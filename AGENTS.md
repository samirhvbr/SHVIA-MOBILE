# ShvIA Mobile — Guia do Agente (AGENTS.md)

> Espelho de [CLAUDE.md](CLAUDE.md) abaixo do H1 — **editar os dois**. Veja também
> [README.md](README.md), [.continue/escopo-mobile.md](.continue/escopo-mobile.md)
> (roadmap M0–M5) e [docs/decisoes.md](docs/decisoes.md) (ADRs).

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

Formato: `versão - comentário em português`. A versão **sempre** vem de `version.md`
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

## PS — Commits: a skill COMMITTER cuida disso

**Existe `.committer.yml` na raiz deste repositório** — é o opt-in da skill
**COMMITTER**, que roda em ciclo (cron, via `~/x/GIT/run.sh`). Enquanto esse arquivo
existir com `enabled: true`, **commitar e pushar não é trabalho seu**.

**O que muda para você:**

- **Não commite nem pushe por padrão.** Conclua a entrega bumpando o `version.md`
  **com a entrada de changelog** e deixe a árvore pronta. É dali que a mensagem do
  commit sai — o changelog virou o artefato de handoff entre você e a skill.
- A skill monta `X.Y.Z - descrição`, commita e pusha a branch atual sozinha. Ela
  **nunca bumpa versão** (isso continua sendo julgamento seu) e nunca inventa
  mensagem: sem entrada de changelog ela cai num fallback Sonnet, e sem conseguir
  descrever com honestidade ela aborta e espera.

**Você ainda commita quando:**

- o Samir pedir explicitamente;
- a tarefa exigir o SHA na hora (deploy, abrir PR, referência cruzada);
- o `.committer.yml` sumir ou estiver `enabled: false` — aí vale o fluxo antigo,
  você bumpa, commita e pusha.

**Por que isso existe:** tirar de um modelo caro (Opus/Fable) o trabalho mecânico de
empacotar commit, que um Sonnet — ou, na maioria das vezes, nenhum modelo — resolve.
Economiza token e devolve tempo de desenvolvimento.

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
