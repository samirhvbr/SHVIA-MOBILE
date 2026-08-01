//! Push remoto (APNs) do ShvIA Mobile — plugin interno, iOS-only por ora.
//!
//! Papel: obter o device token do APNs no lado nativo e entregá-lo ao Rust da
//! casca por um `Channel` (assíncrono — a Apple pode demorar, e a permissão
//! pode ser concedida minutos depois do launch). O Rust injeta o token na
//! página remota via `webview.eval` (`window.__shviaPushToken`), e o front do
//! ShvIA web faz o `POST /push/token` com a sessão. Tap em notificação chega
//! pelo mesmo canal como evento `route`.
//!
//! iOS: o app gerado pelo Tauri NÃO tem AppDelegate em Swift (o delegate é
//! classe ObjC criada pelo tao em runtime) — o Swift deste plugin ADICIONA os
//! seletores `didRegisterForRemoteNotifications...`/`didFail...` nessa classe
//! via `class_addMethod` no `load()` (não é swizzle: os seletores não existem
//! lá). Referências: PR tauri#11652 (draft oficial) e o padrão do plugin de
//! geolocation para `Channel` via `run_mobile_plugin`.
//!
//! Android (FCM) fica para depois — registrar este plugin só em
//! `#[cfg(target_os = "ios")]`.

#![cfg(mobile)]

use serde::Serialize;
use tauri::{
    ipc::Channel,
    plugin::{Builder, PluginHandle, TauriPlugin},
    Manager, Runtime,
};

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_shvia_push);

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    PluginInvoke(#[from] tauri::plugin::mobile::PluginInvokeError),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Acesso ao push nativo. Obtido via [`ShviaPushExt::shvia_push`].
pub struct ShviaPush<R: Runtime>(PluginHandle<R>);

#[derive(Serialize)]
struct WatchPayload {
    channel: Channel,
}

#[derive(serde::Deserialize)]
pub struct PermissionResponse {
    pub granted: bool,
}

impl<R: Runtime> ShviaPush<R> {
    /// Registra o canal por onde o Swift entrega eventos:
    /// `{"type":"token","token":"<hex>"}` quando o APNs responde,
    /// `{"type":"route","route":"/..."}` no tap de notificação,
    /// `{"type":"error","message":"..."}` se o registro falhar.
    pub fn watch_token(&self, channel: Channel) -> Result<()> {
        self.0
            .run_mobile_plugin("watchToken", WatchPayload { channel })
            .map_err(Into::into)
    }

    /// Pede a autorização de notificação ao usuário e, concedida, chama
    /// `registerForRemoteNotifications` (na main thread, no lado Swift).
    /// Idempotente: se já decidida, o iOS não mostra prompt de novo.
    pub fn request_permission(&self) -> Result<PermissionResponse> {
        self.0
            .run_mobile_plugin("requestPermission", ())
            .map_err(Into::into)
    }
}

pub trait ShviaPushExt<R: Runtime> {
    fn shvia_push(&self) -> &ShviaPush<R>;
}

impl<R: Runtime, T: Manager<R>> ShviaPushExt<R> for T {
    fn shvia_push(&self) -> &ShviaPush<R> {
        self.state::<ShviaPush<R>>().inner()
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("shvia-push")
        .setup(|app, api| {
            #[cfg(target_os = "ios")]
            let handle = api.register_ios_plugin(init_plugin_shvia_push)?;
            #[cfg(target_os = "android")]
            unreachable!("shvia-push ainda não tem lado Android (FCM) — registre só em cfg(target_os = \"ios\")");
            #[cfg(target_os = "ios")]
            app.manage(ShviaPush(handle));
            Ok(())
        })
        .build()
}
