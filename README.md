# ShvIA Mobile

Cliente **mobile (iOS + Android)** do ShvIA — **shell fino Tauri 2** que carrega o
ShvIA hospedado (`https://ia.blue3.com.br`) na WebView nativa (WKWebView / Android
System WebView). Irmão do `SHVIA-DESKTOP` (desktop) e do `SHVIA` (servidor Laravel).

> Antes de mexer: **`git pull`**. Convenções em [CLAUDE.md](CLAUDE.md) ·
> Roadmap em [.continue/escopo-mobile.md](.continue/escopo-mobile.md) ·
> Decisões em [docs/decisoes.md](docs/decisoes.md).

## Status

**M0 — scaffold do shell mobile-only** (compila no host via `cargo check`).
Próximo: **M1** (Android — instalar SDK/NDK+JDK17, `tauri android init`, e o
**smoke-test crítico** do `/chat` streaming (SSE) na WebView Android).

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
segredo moram no cliente. Links externos (fora de `*.blue3.com.br`) abrem no
navegador do SO. Tarja "Sistema Offline" injetada em cada página.

## Produção / lojas

Reusa a **infra da Blue3** (que já publica o app Flutter `BLUE3-INTRANET-MOBILE`):
Apple Developer **Team ID `S65UBCTPN5`**, Google Play Console, custódia de keystore.
Bundle ID **`cloud.blue3.shvia`** (mesmo do desktop). Detalhes e fases no roadmap.
