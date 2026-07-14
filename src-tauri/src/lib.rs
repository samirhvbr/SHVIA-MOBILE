//! ShvIA Mobile — shell fino Tauri 2 (iOS + Android).
//!
//! A janela abre a casca local (`src/`), que mostra um splash com a marca e
//! redireciona o WebView (WKWebView no iOS, System WebView no Android) para o
//! ShvIA hospedado (`https://ia.blue3.com.br`). A partir daí a UI é o próprio
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

/// Uma navegação fica **no app** se for a casca local (localhost/tauri) ou o
/// ShvIA hospedado (`*.blue3.com.br`); qualquer outra origem é considerada um
/// link externo e abre no navegador do SO.
fn is_internal(url: &tauri::Url) -> bool {
    let host = url.host_str().unwrap_or_default();
    host == "localhost"
        || host == "tauri.localhost"
        || host == "blue3.com.br"
        || host.ends_with(".blue3.com.br")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
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
