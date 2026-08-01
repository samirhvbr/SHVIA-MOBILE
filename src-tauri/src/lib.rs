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

use std::sync::Mutex;

use tauri::{webview::PageLoadEvent, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_opener::OpenerExt;

#[cfg(target_os = "ios")]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(target_os = "ios")]
use tauri_plugin_shvia_push::ShviaPushExt;

/// Último device token do APNs (M3/push). Vive em estado da casca para ser
/// REINJETADO em cada page load da página remota — o token pode chegar antes
/// da página (cold start) ou a página pode recarregar depois do token.
struct PushToken(Mutex<Option<String>>);

/// Pede a permissão de notificação UMA vez por execução, e só no primeiro
/// load REMOTO (usuário logado = momento com contexto, não no splash).
#[cfg(target_os = "ios")]
struct PushPermissionAsked(AtomicBool);

/// JS que entrega o token à página remota. O front do ShvIA web
/// (`registerPushToken` no app.js) escuta o evento e faz o POST /push/token
/// com a sessão. Token validado como hex antes de entrar no JS.
fn push_token_js(token: &str) -> Option<String> {
    if token.is_empty() || !token.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    Some(format!(
        "window.__shviaPushPlatform='ios';window.__shviaPushToken='{token}';window.dispatchEvent(new Event('shvia:push-token'));"
    ))
}

/// Injetado em cada página carregada (`on_page_load`): uma **tarja "Sistema
/// Offline"** clicável para recarregar. O `eval` do Tauri roda fora da CSP da
/// página, então funciona na página remota do ShvIA.
///
/// v2 (01/08/2026, espelhando o desktop): `navigator.onLine` vira só GATILHO DE
/// SUSPEITA — a tarja apenas aparece se uma sonda REAL ao `/up` (health barato
/// do Laravel) falhar, e re-sonda a cada 15 s até o servidor voltar. O bug que
/// motivou foi no WebKitGTK do desktop (GNetworkMonitor mente com VPN/rotas
/// incomuns); o WKWebView é mais confiável, mas as cascas são espelhadas e a
/// sonda é estritamente melhor nos dois.
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
  function mount(){ var r=document.body||document.documentElement; if(r&&!document.getElementById('shvia-offline-bar')) r.appendChild(bar); }
  var checking=false, timer=null;
  function show(){ bar.style.display='block'; if(!timer) timer=setInterval(check, 15000); }
  function hide(){ bar.style.display='none'; if(timer){ clearInterval(timer); timer=null; } }
  function check(){
    if (checking) return; checking = true;
    var ctl = ('AbortController' in window) ? new AbortController() : null;
    var to = setTimeout(function(){ if (ctl) ctl.abort(); }, 5000);
    fetch('/up', { cache:'no-store', signal: ctl && ctl.signal })
      .then(function(r){ if (r.ok) { hide(); } else { show(); } })
      .catch(function(){ show(); })
      .finally(function(){ clearTimeout(to); checking = false; });
  }
  function update(){ if (navigator.onLine) { hide(); } else { check(); } }
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
///
/// No MESMO IP do ápex vive `mem.shvia.org` (servidor ai-memory, desde
/// 31/07/2026) — o melhor argumento contra o curinga que esta lista recusa: é
/// `.shvia.org`, é nosso, é legítimo, e mesmo assim **não pode entrar**. Serve
/// wiki markdown **escrita por agentes**, e o curinga o teria admitido sozinho
/// no dia em que o DNS subiu, sem ninguém decidir nada.
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
    ///
    /// `mem.shvia.org` (ai-memory, mesmo IP do ápex) é o caso REAL, não
    /// hipotético: subdomínio nosso, legítimo, no ar, servindo wiki escrita por
    /// agentes. É o que o curinga deixaria entrar sem ninguém decidir nada.
    #[test]
    fn outras_origens_sao_externas() {
        assert!(!internal("https://shvia.org/"));
        assert!(!internal("https://www.shvia.org/"));
        assert!(!internal("https://mem.shvia.org/"));
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

    // Push (M3/ADR-002): plugin interno iOS-only. O token do APNs sobe do
    // Swift por Channel e é injetado na página remota; a página continua sem
    // NENHUM comando nativo exposto (quem fala com o plugin é o Rust daqui).
    // Android (FCM) fica para depois — por isso `target_os = "ios"`, não `mobile`.
    #[cfg(target_os = "ios")]
    {
        builder = builder.plugin(tauri_plugin_shvia_push::init());
    }

    builder
        .setup(|app| {
            app.manage(PushToken(Mutex::new(None)));
            #[cfg(target_os = "ios")]
            app.manage(PushPermissionAsked(AtomicBool::new(false)));

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
                            let mut js = format!(
                                "window.__shviaShellVersion={:?};{}",
                                env!("CARGO_PKG_VERSION"),
                                OFFLINE_BANNER_JS
                            );
                            // Token de push conhecido? Reinjetar a cada load —
                            // o front (registerPushToken) é idempotente.
                            if let Some(tok) =
                                webview.state::<PushToken>().0.lock().unwrap().clone()
                            {
                                if let Some(tjs) = push_token_js(&tok) {
                                    js.push_str(&tjs);
                                }
                            }
                            let _ = webview.eval(js);

                            // 1º load remoto = usuário chegou ao ShvIA logado:
                            // hora de pedir a permissão de notificação (com
                            // contexto, não no splash). `requestPermission`
                            // BLOQUEIA até o usuário decidir → thread própria.
                            #[cfg(target_os = "ios")]
                            {
                                let asked = webview.state::<PushPermissionAsked>();
                                if !asked.0.swap(true, Ordering::SeqCst) {
                                    let wv = webview.clone();
                                    std::thread::spawn(move || {
                                        let _ = wv.shvia_push().request_permission();
                                    });
                                }
                            }
                        }
                    }
                })
                .build()?;

            // Canal de eventos do push (iOS): o Swift entrega token/rota/erro
            // aqui; token vai pro estado + eval; tap navega (rota sanitizada).
            #[cfg(target_os = "ios")]
            {
                let handle = app.handle().clone();
                let channel = tauri::ipc::Channel::new(move |event| {
                    let tauri::ipc::InvokeResponseBody::Json(json) = event else {
                        return Ok(());
                    };
                    let Ok(v) = serde_json::from_str::<serde_json::Value>(&json) else {
                        return Ok(());
                    };
                    match v.get("type").and_then(|t| t.as_str()) {
                        Some("token") => {
                            if let Some(tok) = v.get("token").and_then(|t| t.as_str()) {
                                if let Some(js) = push_token_js(tok) {
                                    *handle.state::<PushToken>().0.lock().unwrap() =
                                        Some(tok.to_string());
                                    if let Some(w) = handle.get_webview_window("main") {
                                        let _ = w.eval(js);
                                    }
                                }
                            }
                        }
                        Some("route") => {
                            // Tap em notificação. Só caminho relativo simples:
                            // nada de URL absoluta/esquema/escape — a rota vem
                            // do payload do push e não é confiável por padrão.
                            if let Some(route) = v.get("route").and_then(|r| r.as_str()) {
                                let segura = route.starts_with('/')
                                    && !route.starts_with("//")
                                    && route.chars().all(|c| {
                                        c.is_ascii_graphic()
                                            && !matches!(c, '"' | '\'' | '\\' | '<' | '>' | '`')
                                    });
                                if segura {
                                    if let Some(w) = handle.get_webview_window("main") {
                                        let _ = w.eval(format!("location.assign({route:?})"));
                                    }
                                }
                            }
                        }
                        Some("error") => {
                            // Sem token não há push; o app segue normal.
                            eprintln!(
                                "shvia-push: {}",
                                v.get("message").and_then(|m| m.as_str()).unwrap_or("erro")
                            );
                        }
                        _ => {}
                    }
                    Ok(())
                });
                let _ = app.shvia_push().watch_token(channel);
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("erro ao executar o aplicativo ShvIA Mobile");
}
