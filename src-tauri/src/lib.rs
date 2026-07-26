//! ShvIA Mobile — shell fino Tauri 2 (iOS + Android).
//!
//! A janela abre a casca local (`src/`), que mostra um splash com a marca e
//! redireciona o WebView (WKWebView no iOS, System WebView no Android) para o
//! ShvIA hospedado (`https://ai.shvia.org`). A partir daí a UI é o próprio
//! Blade do ShvIA — "mesmas funções" (espelha a postura do SHVIA-DESKTOP).
//!
//! Postura de menor privilégio: **nenhum comando nativo é exposto à página
//! remota** — o servidor é a fonte da verdade; o cliente não abre banco nem
//! guarda segredo.
//!
//! **Mobile-only:** sem menu / multi-janela / geometria de janela (isso é
//! desktop). A leitura em voz (TTS) usa o `speechSynthesis` nativo do WebView
//! (iOS/Android já têm voz pt-BR), então **não** precisa da ponte espeak do
//! Linux/WebKitGTK. Links externos abrem no navegador do SO via `on_navigation`.

use tauri::{webview::PageLoadEvent, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_opener::OpenerExt;

/// Injetado em cada página carregada (`on_page_load`): uma **tarja "Sistema
/// Offline"** que aparece quando o WebView perde conexão (eventos `online`/
/// `offline`) e some ao reconectar; clicável para recarregar. O `eval` do Tauri
/// roda fora da CSP da página, então funciona na página remota do ShvIA.
const OFFLINE_BANNER_JS: &str = r#"(function () {
  if (window.__shviaOffline) return;
  window.__shviaOffline = true;
  var bar = document.createElement('div');
  bar.id = 'shvia-offline-bar';
  bar.textContent = '⚠  Sistema Offline — toque para recarregar';
  var s = bar.style;
  s.position='fixed'; s.top='0'; s.left='0'; s.right='0'; s.zIndex='2147483647';
  s.background='#c0392b'; s.color='#fff'; s.textAlign='center'; s.padding='calc(env(safe-area-inset-top,0px) + 8px) 12px 8px';
  s.font='600 14px system-ui,sans-serif'; s.letterSpacing='.02em'; s.cursor='pointer';
  s.boxShadow='0 2px 8px rgba(0,0,0,.35)';
  bar.addEventListener('click', function () { location.reload(); });
  function update(){ bar.style.display = navigator.onLine ? 'none' : 'block'; }
  function mount(){ var r=document.body||document.documentElement; if(r&&!document.getElementById('shvia-offline-bar')) r.appendChild(bar); }
  mount(); update();
  window.addEventListener('online', update);
  window.addEventListener('offline', update);
})();"#;

/// Host CANÔNICO do servidor do ShvIA — o destino da navegação da casca.
const SERVER_HOST: &str = "ai.shvia.org";

/// Hosts EXATOS aceitos como o servidor do ShvIA. Mantido em PARIDADE com
/// `SERVER_HOSTS` de SHVIA-DESKTOP/src-tauri/src/lib.rs — as duas cascas são
/// espelhadas de propósito.
///
/// Lista exata, **sem sufixo curinga**. O curinga anterior
/// (`host.ends_with(".blue3.com.br")`) aceitava QUALQUER subdomínio do domínio
/// corporativo dentro do app; o desktop fechou isso no 0.9.0 e o mobile herda a
/// mesma postura agora. Cada entrada foi verificada por DNS:
///
/// - `ai.shvia.org` — canônico (200.36.196.254).
/// - `ia.shvia.org` — CNAME de `ai.shvia.org`, mesma instância.
/// - `ia.blue3.com.br` — domínio legado, MESMO IP. Só durante a transição;
///   remover esta linha é tudo o que o desligamento dele exige.
///
/// O ápex `shvia.org` fica FORA de propósito: resolve para outro IP
/// (170.233.231.20) e serve a landing, não o app — tem de abrir no navegador.
const SERVER_HOSTS: &[&str] = &[SERVER_HOST, "ia.shvia.org", "ia.blue3.com.br"];

/// Uma navegação fica **no app** se for a casca local (localhost/tauri) ou um
/// host do servidor do ShvIA em https; qualquer outra origem é link externo e
/// abre no navegador do SO.
fn is_internal(url: &tauri::Url) -> bool {
    let host = match url.host_str() {
        Some(h) => h,
        None => return false,
    };
    // Casca local: `http` (Vite dev + prod Android) e `tauri` (prod iOS).
    // O servidor exige https.
    match url.scheme() {
        "https" => SERVER_HOSTS.contains(&host),
        "http" => host == "localhost" || host == "tauri.localhost",
        "tauri" => host == "localhost",
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::is_internal;

    fn internal(url: &str) -> bool {
        is_internal(&url.parse().expect("url de teste válida"))
    }

    /// A casca empacotada tem URL diferente por plataforma. Se qualquer uma
    /// deixar de ser interna, a navegação inicial é bloqueada e o app fica preso
    /// no splash — sem sintoma em `tauri dev` (que usa devUrl).
    #[test]
    fn casca_local_e_interna() {
        assert!(internal("tauri://localhost/index.html"));
        assert!(internal("http://tauri.localhost/index.html"));
        assert!(internal("http://localhost:1420/"));
    }

    /// Dual-host da migração (26/07): as três faces do servidor entram, e o dia
    /// em que o legado sair é só tirar a linha dele de SERVER_HOSTS.
    #[test]
    fn as_tres_faces_do_servidor_sao_internas() {
        assert!(internal("https://ai.shvia.org/chat"));
        assert!(internal("https://ia.shvia.org/chat"));
        assert!(internal("https://ia.blue3.com.br/chat"));
        assert!(!internal("http://ai.shvia.org/chat")); // servidor só em https
    }

    /// Fim do curinga: o ápex shvia.org é a LANDING (outro IP) e subdomínio
    /// qualquer de blue3.com.br não é o ShvIA. Nada disso carrega dentro do app.
    #[test]
    fn outras_origens_sao_externas() {
        assert!(!internal("https://shvia.org/"));
        assert!(!internal("https://www.shvia.org/"));
        assert!(!internal("https://evil.shvia.org/"));
        assert!(!internal("https://blue3.com.br/"));
        assert!(!internal("https://evil.blue3.com.br/"));
        assert!(!internal("https://ai.shvia.org.evil.com/"));
        assert!(!internal("file:///etc/passwd"));
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(unused_mut)] // no host o bloco `#[cfg(mobile)]` some → `mut` não usado
    let mut builder = tauri::Builder::default().plugin(tauri_plugin_opener::init());

    // Biometria (M3/ADR-002): plugin **mobile-only** — o crate é `#![cfg(mobile)]`
    // e nem existe no host (por isso o registro fica atrás de `#[cfg(mobile)]`).
    // Expõe `authenticate`/`status` à página LOCAL (`src/`), que faz o gate Face
    // ID antes de navegar pro ShvIA remoto. A página remota segue sem acesso nativo.
    #[cfg(mobile)]
    {
        builder = builder.plugin(tauri_plugin_biometric::init());
    }

    builder
        .setup(|app| {
            let handle = app.handle().clone();
            WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
                .title("ShvIA")
                .on_navigation(move |url| {
                    if is_internal(url) {
                        return true;
                    }
                    // link externo → navegador do SO, não dentro do app.
                    let _ = handle.opener().open_url(url.to_string(), None::<&str>);
                    false
                })
                // injeta a tarja "Sistema Offline" SÓ nas páginas remotas do
                // ShvIA: a casca local tem UI própria de offline (splash com
                // "Tentar novamente") — com a tarja junto ficava em dobro.
                // Também expõe a versão da casca (`window.__shviaShellVersion`)
                // para o ShvIA web poder exibi-la junto da versão do servidor.
                .on_page_load(|webview, payload| {
                    if let PageLoadEvent::Finished = payload.event() {
                        let host = payload.url().host_str().unwrap_or_default().to_owned();
                        if host != "localhost" && host != "tauri.localhost" {
                            let _ = webview.eval(format!(
                                "window.__shviaShellVersion={:?};{}",
                                env!("CARGO_PKG_VERSION"),
                                OFFLINE_BANNER_JS
                            ));
                        }
                    }
                })
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("erro ao executar o aplicativo ShvIA Mobile");
}
