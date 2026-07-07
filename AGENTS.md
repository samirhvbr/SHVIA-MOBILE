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
`https://ia.blue3.com.br`. **Shell fino Tauri 2:** a WebView nativa (WKWebView no
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
- Servidor/fonte da verdade: `~/x/SHVIA` (`https://ia.blue3.com.br`)
- Desktop irmão: `~/x/SHVIA-DESKTOP` · Base de reuso de loja: `~/x/BLUE3-INTRANET-MOBILE`
