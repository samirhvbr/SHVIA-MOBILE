# ShvIA Mobile

Cliente **mobile (iOS + Android)** do ShvIA — **shell fino Tauri 2** que carrega o
ShvIA hospedado (`https://ai.shvia.org`) na WebView nativa (WKWebView / Android
System WebView). Irmão do `SHVIA-DESKTOP` (desktop) e do `SHVIA` (servidor Laravel).

> Antes de mexer: **`git pull`**. Convenções em [CLAUDE.md](CLAUDE.md) ·
> Roadmap em [.continue/escopo-mobile.md](.continue/escopo-mobile.md) ·
> Decisões em [docs/decisoes.md](docs/decisoes.md).

## Status

**M2 fechado** (15/07): o app roda no **iPhone físico** via TestFlight interno,
com biometria (Face ID/Touch ID) na 0.4.0 e o domínio próprio `ai.shvia.org` na
0.5.0. Android **builda** (APK/AAB no Linux), mas ainda não rodou em aparelho.

**Próximo: push (APNs)** — o lado servidor está pronto no SHVIA-WEB desde a
2.51.0 e falta o lado cliente; é ele que sustenta a defesa da regra **4.2** da
App Store. Estado completo e o que a revisão de 28/07 achou:
[.continue/escopo-mobile.md](.continue/escopo-mobile.md) §5.

## Build

```bash
npm install
# valida o Rust no host (sem toolchain mobile):
cargo check --manifest-path src-tauri/Cargo.toml
# Android (precisa Android SDK/NDK + JDK 17):
npm run tauri android init && npm run tauri android dev
# iOS (precisa macOS + Xcode):
npm run tauri ios init && npm run tauri ios dev
```

## Arquitetura

Shell fino: a WebView navega o FQDN real; o **servidor Laravel é a fonte da
verdade** (dados, auth por **cookie de sessão same-origin**). Nenhum banco nem
segredo moram no cliente. Links externos (fora dos hosts do servidor — `SERVER_HOSTS` em `src-tauri/src/lib.rs`) abrem no
navegador do SO. Tarja "Sistema Offline" injetada em cada página.

## Produção / lojas

Reusa a **infra da Blue3** (que já publica o app Flutter `BLUE3-INTRANET-MOBILE`):
Apple Developer **Team ID `S65UBCTPN5`**, Google Play Console, custódia de keystore.
Bundle ID **`cloud.blue3.shvia`** (mesmo do desktop). Detalhes e fases no roadmap.
