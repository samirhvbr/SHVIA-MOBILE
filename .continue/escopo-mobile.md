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
  0.3.5: itens 1–3 e 8 do smoke-test ✓ no simulador com a 2.17.11 em produção,
  item 9 portrait ok; faltam 4–7 com humano) · [ ] M3 · [ ] M4 ·
  [~] M5 (2.17.11 matou o drawer-vidro/safe-area; sobras: auto-zoom no composer,
  leve overflow horizontal em portrait, landscape segue desktop)
- **Próximo:** itens 4–7 do smoke-test com humano (TTS/mic/anexos/links) e as
  sobras do SHVIA-WEB acima. Depois: **TestFlight** (M2 final). Android
  on-device continua pendente (mesma sentada com aparelho USB).
