# ShvIA Mobile — Decisões (ADRs)

Formato ADR. Não relitigar direção já decidida dentro de um how-to — linkar o ADR.

---

## ADR-001 — Repo separado, stack Tauri, reusando a infra de loja da Blue3

- **Data:** 07/07/2026 · **Status:** Aceito
- **Contexto:** Levar o ShvIA para **iOS + Android**. O desktop (`SHVIA-DESKTOP`)
  já é Tauri 2 (shell fino que carrega `ia.blue3.com.br`). A Blue3 **já publica**
  app mobile em Flutter (`BLUE3-INTRANET-MOBILE`) — então conta Apple, Play Console,
  custódia de keystore e know-how de review **já existem**.
- **Decisão:**
  1. **Repo próprio `SHVIA-MOBILE`** (não estender o `SHVIA-DESKTOP`), seguindo a
     convenção da casa (`SSHVTERM-DESKTOP` + `SSHVTERM-MOBILE`). O desktop volta a
     ser **desktop-only**.
  2. **Stack = Tauri** (não Flutter): reusa a lógica do shell do desktop (tarja
     offline, roteio de link externo), um produto coeso Rust/Tauri.
  3. **Reusar a infra de loja da Blue3** — Apple **Team ID `S65UBCTPN5`**, Google
     Play Console, custódia de keystore. Reusa **identidades/assinatura**, **não** o
     build Flutter (o build aqui é Tauri: Gradle em `gen/android` + Xcode em `gen/apple`).
  4. **Bundle ID `cloud.blue3.shvia`** — o mesmo do desktop (consistência do produto).
- **Consequências:** shell mobile-only enxuto — **sem** menu / multi-janela /
  geometria / ponte WebKitGTK / TTS-espeak. TTS = `speechSynthesis` nativo do WebView
  (iOS/Android têm voz pt-BR). **iOS exige Mac**; Android builda no Linux. Fases em
  [../.continue/escopo-mobile.md](../.continue/escopo-mobile.md).
- **Alternativa descartada:** **Flutter** — reusaria o pipeline já publicado e a
  expertise do time, mas forkaria do shell desktop e duplicaria o wrapper em Dart.
