# ShvIA Mobile — Roadmap de Produção (M0–M5)

> Este repo é o cliente **mobile** (iOS + Android) do ShvIA, em **Tauri 2** (ADR-001).
> Irmãos: `SHVIA-DESKTOP` (desktop) e `SHVIA` (servidor Laravel). WIP mora aqui;
> quando amadurece, migra para `docs/`.
>
> **Decisões travadas (07/07/2026):** stack **Tauri**; repo separado; publicar nas
> **lojas** (App Store + Play) reusando a infra da Blue3; bundle **`cloud.blue3.shvia`**.

---

## 1. Premissa

**Shell fino Tauri 2** carregando `https://ia.blue3.com.br` no WebView nativo
(iOS = WKWebView, Android = System WebView). Servidor = fonte da verdade. Não é
reescrita — é o mesmo conceito do desktop, mobile-only.

**Auth (comportamento aprovado — Samir, 07/07/2026):** cookie de sessão
same-origin; sessão válida → **entra direto, sem pedir credencial** (validado no
desktop Linux). Sem autofill de usuário/senha no cliente (segredo zero no shell,
ADR-001). No iOS, conferir a persistência real do cookie entre reinícios no
smoke-test (item 2 de [../docs/smoke-test.md](../docs/smoke-test.md)).

## 2. Barreiras (e o que já está resolvido)

1. **iOS só builda em macOS + Xcode.** Android builda no Linux. (Há Mac/CI macOS.)
2. **App Store 4.2 (web wrapper).** WebView que só abre um site pode ser reprovado →
   **M3 (valor nativo) é pré-requisito de submissão iOS.** Play é mais tolerante.
3. **Contas/assinatura/loja — JÁ EXISTEM.** A Blue3 publica `BLUE3-INTRANET-MOBILE`
   (Flutter): Apple **Team ID `S65UBCTPN5`**, Play Console, custódia de keystore.
   Reusar (ver §3). Deixa de ser barreira.
4. **UX mobile = responsividade do ShvIA web.** Como é shell fino, o celular mostra
   o Blade remoto → trabalho no repo **`SHVIA`** (Laravel), não neste (M5).

## 3. Reuso da infra Blue3 (não é greenfield)

- **Apple:** mesma conta/App Store Connect; Team ID `S65UBCTPN5`, signing
  `automatic`, method `app-store-connect` → `tauri.ios.conf.json`
  (`bundle.iOS.developmentTeam`).
- **Google Play:** mesma org; keystore via `key.properties` (não versionado) + Play
  App Signing → replicar no `gen/android`.
- **Não** transfere o *build* Flutter (Runner/Xcode + `flutter build`) ≠ Tauri
  (Gradle em `gen/android` + Xcode em `gen/apple`). Reusa identidades, não pipeline.
- **Build number** monotônico (iOS `CFBundleVersion` / Android `versionCode`):
  padrão deles `X.Y.Z+<AAAAMMDDHHMM>`; adotar equivalente a partir do `version.md`.

## 4. Fases

### M0 — Scaffold do shell mobile-only ✅ (feito)
Repo criado espelhando o desktop, mas mobile-only: `src-tauri` enxuto (só tarja
offline + roteio de link externo; sem menu/multi-janela/WebKitGTK/TTS-espeak),
`cargo check` no host **passa**. Targets Rust Android instalados no host.

### M1 — Android (no Linux) — **build OK ✅**, smoke-test on-device pendente
- ✅ Toolchain **reusado da máquina** (dev Flutter da Blue3), zero download: JDK 17
  (`~/Android/jdk-17`), SDK (`~/Android/Sdk`, platforms 34-36, build-tools, adb),
  **NDK 28.2**. Requer `JAVA_HOME`/`ANDROID_HOME`/`NDK_HOME` no ambiente (documentar
  em `docs/build.md` ou num `build-local` de Android).
- ✅ `tauri android init` → `gen/android` (versionado); `tauri android build --debug`
  produz **APK + AAB** (arm64), Rust compilado via NDK. Cadeia validada de ponta a ponta.
  `applicationId = cloud.blue3.shvia`.
- ⏳ **SMOKE-TEST #1 (crítico):** `/chat` **streaming (SSE)** na WebView Android + login
  → cookie persistente + mic — **precisa de aparelho/emulador** (esta máquina não tem
  KVM/device). `tauri android dev` com um device via adb, ou emulador com KVM.
- ⏳ Keystore (reuso do padrão Blue3: `key.properties` + Play App Signing), **AAB
  assinado** (release), script `build-local` de Android.

### M2 — iOS (no Mac) — **em andamento (foco atual, 14/07)**
- ✅ **Pré-config feita do Linux (0.2.6):** `bundle.iOS.developmentTeam =
  S65UBCTPN5` no `tauri.conf.json` (o init já sai assinável) +
  `src-tauri/Info.ios.plist` com mic/câmera (o Tauri mescla no Info.plist
  gerado) + ícones iOS prontos desde 0.2.2.
- **MacBook disponível (07/07)** — o desktop 0.5.5 rodou lá com `npm run tauri dev`.
- ✅ **Lado Mac feito (0.3.0, 14/07):** `tauri ios init` → `gen/apple` gerado
  (Team + bundle conferidos no `project.pbxproj`; mic/câmera mesclados no
  Info.plist) + `tauri icon` (Xcode assets) + `tauri ios dev` → **app abriu
  logado no simulador iPhone 17 Pro Max**. Atenção: `tauri icon` reseta o
  `ic_launcher_background.xml` do Android p/ `#fff` — restaurar o navy `#0D1B2A`
  após rodar (feito em 0.3.0).
- ⏳ Checklist do smoke-test (§iOS itens 3–9: SSE ⭐, TTS, mic, anexos, links,
  teclado, rotação) + **safe-area**: a topbar remota sobe por baixo da status
  bar/Dynamic Island no simulador (`viewport-fit`/`env(safe-area-inset-top)` —
  provável ajuste no SHVIA-WEB, ver M3/M5).
- Depois do smoke-test: primeiro build no **TestFlight**.

### M3 — Valor nativo (destrava App Store 4.2)
Push (FCM/APNs), biometria, compartilhar, deep-link `shvia://`, tela offline nativa,
safe-area/status bar/splash. Melhora UX e justifica o app além do wrapper.

### M4 — Publicação nas lojas
Listings, `PrivacyInfo` (iOS) / data-safety (Play), screenshots, submissão/review.
Release local (Android no Linux, iOS no Mac) ou runner macOS de CI só p/ iOS.

### M5 — Responsividade mobile do ShvIA web (repo `SHVIA`)
`viewport`, `env(safe-area-inset-*)`, teclado, nav/sidebar colapsável, alvos de toque.
Pré-requisito de qualidade p/ submeter.

## 5. Estado

- [x] M0 · [~] M1 (build OK; falta smoke-test on-device) · [~] M2 (**foco atual**;
  0.3.0: `gen/apple` no repo e app rodando logado no simulador — falta o
  checklist completo do smoke-test) · [ ] M3 · [ ] M4 ·
  [x] M5 (núcleo: topbar/colapso ok no SHVIA-WEB 2.15.3–2.15.4; sobras menores =
  max-height de modais com teclado, faixa 720–768px)
- **Próximo:** rodar o checklist do smoke-test iOS (§iOS itens 3–9) no simulador
  que já está de pé, e resolver a safe-area da topbar (status bar/Dynamic
  Island). Android on-device continua pendente (mesma sentada com aparelho USB).
