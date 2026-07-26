# ShvIA Mobile — Roadmap de Produção (M0–M5)

> Este repo é o cliente **mobile** (iOS + Android) do ShvIA, em **Tauri 2** (ADR-001).
> Irmãos: `SHVIA-DESKTOP` (desktop) e `SHVIA` (servidor Laravel). WIP mora aqui;
> quando amadurece, migra para `docs/`.
>
> **Decisões travadas (07/07/2026):** stack **Tauri**; repo separado; publicar nas
> **lojas** (App Store + Play) reusando a infra da Blue3; bundle **`cloud.blue3.shvia`**.

---

## 1. Premissa

**Shell fino Tauri 2** carregando `https://ai.shvia.org` no WebView nativo
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
- ✅ **Smoke-test parcial no simulador (0.3.1, 14/07):**
  - **Item 1 ✓** — offline nativo: relaunch mostrou "Sem conexão — reconectando…"
    e **entrou sozinho** sem clique (auto-retry ok).
  - **Item 2 ✓** — cookie **persistiu** após `simctl terminate` + relaunch (a
    dúvida do WKWebView está respondida: entra logado direto).
  - **Item 3 ✓ (0.3.4, re-teste pós-2.17.11)** — **streaming token a token
    confirmado visualmente**: 3 frames seguidos com o raciocínio crescendo
    palavra a palavra (cursor de digitação visível) e timer ao vivo; resposta
    final 1.239 tok @ 22 tok/s. Enter do teclado físico **envia** (form submit
    funciona mesmo com o quirk de digitação do simulador).
  - **Item 4 ~** — botão "ouvir" presente; áudio precisa de ouvido humano.
  - **Itens 5–7** (mic, anexos, links externos) pendentes — precisam de humano.
  - **Item 8 ✓ (re-teste)** — teclado virtual sobe e o composer levanta junto,
    campo focado visível, sem cobertura; digitação pelas teclas na tela funciona.
  - **Item 9 ~ (re-teste)** — portrait ok (rotação DURANTE streaming não quebrou
    a geração); landscape segue layout desktop (sobra conhecida da 2.17.11).
- 🐞 **BUG BLOQUEADOR — CORRIGIDO no SHVIA-WEB 2.17.11 (14/07), aguardando
  deploy.** Causa-raiz achada com o CSS de produção (md5 == repo): em ≤768 px o
  painel do meio vira drawer `position:fixed` **default-aberto** com fundo
  `--bg-panel` = rgba 2,5% → um **vidro invisível por cima do chat**. O
  "hit-testing deslocado ~400 px" era isso: o chat aparecia ATRAVÉS do drawer,
  mas os toques acertavam o drawer. Fix (validado em harness 440×956 com o CSS
  real): drawer inicia FECHADO no mobile, fundo sólido `#11161e`, backdrop
  toque-fora-fecha, chip "Projeto" da topbar abre/fecha (o stub de reabrir é
  display:none no mobile — não havia porta), safe-area na topbar e no drawer
  (a regra mobile antiga da topbar era letra morta: a base "pit-wall" vinha
  depois no arquivo e vencia a cascata). Cadastro: iOS dava **zoom automático**
  ao focar campo com fonte <16 px (o "corte lateral") → inputs a 16 px +
  `viewport-fit=cover` + safe-area no layout auth. **Sobra conhecida:**
  landscape (item 9) segue layout desktop (956 px de largura > breakpoint) —
  avaliar `max-height` como critério de mobile depois do re-teste.
- ✅ **2.17.11 deployado (14/07 ~11h)** — confirmado via `/api/v1/health`
  (`version.app`). Chat em coluna única no iPhone, toques funcionando (Samir
  digitou e conversou no simulador). Sobra nova: **auto-zoom do iOS ao focar o
  composer do chat** (o fix de fonte 16 px da 2.17.11 cobriu o auth, não o
  composer) — mesma correção, outro campo.
- 🔧 **Falso "Sem conexão" resolvido na casca (0.3.4):** o ping de saúde saiu de
  `/api/v1/health` (sonda 14 provedores em série; em cache frio >6 s = estourava
  o timeout da casca) para o **`/up`** nativo do Laravel (~30 ms). Diagnóstico
  completo no commit. Pendências do LADO SERVIDOR reveladas pelo health:
  `redis: status error`, `gemini`/`perplexity` com `http_404` na sonda, e
  (opcional) desacoplar a sonda pesada do request (cache maior/refresh em
  background) para o health não depender do humor dos provedores.
- Quirk de teste (não é bug do app): teclado de hardware do simulador não digita
  no campo do chat (WKWebView); **colar** (long-press → Paste) funciona. Testar
  teclado virtual em aparelho físico.
- Depois do smoke-test: primeiro build no **TestFlight**.

### M3 — Valor nativo (destrava App Store 4.2) — **OBRIGATÓRIO (decisão 15/07)**
Push (APNs), biometria (Face ID), compartilhar, deep-link `shvia://`, tela offline
nativa, safe-area/status bar/splash. **Samir escolheu o Caminho B — App Store PÚBLICA,
mesmo trilho do `~/x/BLUE3-INTRANET-MOBILE`** (`app-store-connect`, conta S65UBCTPN5,
custo zero novo). Como o BLUE3 é Flutter NATIVO ele passou a 4.2 fácil; o ShvIA é
**shell fino (wrapper) → a 4.2 se aplica**, então o M3 deixa de ser opcional. Push =
reusar a infra APNs da Blue3 (.p8/App Group já existem). Ver checklist §2.1.

**Progresso (ADR-002 — sequência por tratabilidade, não por impacto):**
- [x] **Biometria (Face ID / Touch ID) — mobile 0.4.0 (16/07).** `tauri-plugin-biometric`
  como plugin **mobile-only** (`[target.'cfg(ios/android)']` + registro atrás de
  `#[cfg(mobile)]`; capability `mobile.json` escopada a iOS/Android — declarar no
  `default` quebraria o build host). Gate na casca **LOCAL** (`src/main.ts`, a única
  com bridge nativo): opt-in na 1ª execução, desbloqueio no cold-start antes de
  navegar pro remoto, "Desativar bloqueio" exige re-autenticar; `NSFaceIDUsageDescription`
  no `Info.ios.plist`; passcode do aparelho como fallback. **Não** substitui o cookie
  de sessão (é acesso LOCAL, modelo Blue3). **Verificado no host:** `cargo check`
  (host **e** `aarch64-apple-ios`) + `tsc`/`vite build` passam. **Pendente:** Face ID
  real no simulador/aparelho (novo item do smoke-test — só se valida com device).
- [ ] **Push (APNs)** — bloqueado em recurso externo (Samir): `.p8` + capability Push +
  App Group + entitlement no portal, **e** endpoint no SHVIA-WEB (guardar token + enviar).
- [ ] **Universal Links** — bloqueado em `apple-app-site-association` no SHVIA-WEB +
  entitlement `associated-domains`.
- [~] Câmera+mic (feito) · tela offline nativa (feita).

### M4 — Publicação nas lojas → **checklist executável em [../docs/testflight-checklist.md](../docs/testflight-checklist.md)**
Doc criado (15/07) em 2 partes: **Parte 1 = TestFlight interno** (fecha o M2, sem
4.2) e **Parte 2 = App Store pública** (4.2, nutrition label, screenshots). Já
adiantado do Linux (mobile 0.3.11): `PrivacyInfo.xcprivacy` (sem tracking,
UserDefaults CA92.1), `ITSAppUsesNonExemptEncryption=false`, `minimumSystemVersion
14.0` + `category productivity`. Falta o lado Mac (build/upload) e itens `[ASC]`.

### M5 — Responsividade mobile do ShvIA web (repo `SHVIA`)
`viewport`, `env(safe-area-inset-*)`, teclado, nav/sidebar colapsável, alvos de toque.
Pré-requisito de qualidade p/ submeter.

## 5. Estado

- [x] M0 · [~] M1 (build OK; falta smoke-test on-device) · [x] **M2 FECHADO 🎉**
  (15/07: app instalado e RODANDO no iPhone físico via TestFlight interno, build
  0.3.13/casca 0.3.14; smoke 1–3, 8, 9-portrait ✓ no simulador; itens 4–7
  TTS/mic/anexos/links agora dá pra fechar NO APARELHO REAL) · [ ] M3 · [ ] M4 ·
  [x] M5 (2.17.11 drawer-vidro/safe-area ✓ · 2.17.12 auto-zoom do composer ✓
  e topbar 2 linhas + versão visível · 2.17.13 fonte por usuário ·
  **2.17.16 clip lateral ✓** — era min-width:auto nos itens da grade
  .chat-area, #messages esticava a 687px/440; diagnóstico por badge injetado
  pela casca no WKWebView · **2.19.2 landscape ✓** — ver abaixo)
- **DECISÃO landscape (15/07): é BUG p/ telefone, FEATURE p/ tablet.** O gatilho
  mobile era só `max-width` (768px); deitado o iPhone tem ~932px de largura e
  caía nas 3 colunas de desktop (sidebar 320px comendo a tela, thread com ~268px
  de altura útil). Fix (SHVIA-WEB **2.19.2**, validado em harness 932×430 com o
  CSS/JS reais): adicionado `(max-height: 480px)` como 2º gatilho de mobile aos
  17 blocos `@media` (8×768 + 9×720) e aos 3 `matchMedia` do app.js. 480px
  separa telefone-deitado (~430px alto) de tablet-deitado (iPad landscape
  744–834px) — **iPad em paisagem segue em colunas de propósito**. Não-regressão
  confirmada: portrait (390×844) e desktop (1280×800) idênticos. **2.19.2
  DEPLOYADA (15/07)** — confirmado: `/api/v1/health` version.app=2.19.2 e os
  assets em produção têm o gatilho (CSS 17× / JS 3× `max-height: 480px`).
- **Próximo (M2 já fechado):** fechar smoke-test itens 4–7 (TTS/mic/anexos/links)
  AGORA no iPhone físico via TestFlight — mic/câmera reais que o simulador fingia.
  Depois: **M3 (valor nativo)** rumo à loja pública (push APNs reusando a .p8 da
  Blue3, deep-link, biometria — ver checklist §2.1). Android on-device pendente.
  **Gotchas da 1ª submissão iOS documentados** em
  [../docs/testflight-checklist.md](../docs/testflight-checklist.md): buildar SÓ
  pelo terminal (Xcode GUI = `npm: command not found`), ícone 1024 sem canal alpha
  (`magick -alpha remove`), upload via Transporter, teste interno não usa código.
