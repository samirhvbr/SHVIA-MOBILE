// Push remoto (APNs) do ShvIA Mobile.
//
// O app iOS gerado pelo Tauri não tem AppDelegate em Swift — o delegate é uma
// classe ObjC criada pelo tao em runtime, e ela NÃO implementa os seletores de
// push. Por isso este plugin os ADICIONA via `class_addMethod` (adição, não
// swizzle) no `load()`, antes de qualquer `registerForRemoteNotifications`.
//
// Eventos sobem ao Rust por um `Channel` (registrado via comando `watchToken`):
//   {"type":"token","token":"<hex>"}   — device token do APNs
//   {"type":"route","route":"/..."}    — tap em notificação (userInfo.route)
//   {"type":"error","message":"..."}   — falha de registro
// O Rust injeta o token na página remota; a página nunca fala com este plugin.

import Tauri
import UIKit
import UserNotifications
import WebKit

struct WatchArgs: Decodable {
  let channel: Channel
}

class ShviaPushPlugin: Plugin, UNUserNotificationCenterDelegate {
  // O bloco C do class_addMethod não captura `self` do plugin diretamente —
  // referência estática (há um único plugin por app).
  static weak var shared: ShviaPushPlugin?

  private var tokenChannel: Channel?
  private var lastToken: String?
  private var lastError: String?
  private var pendingRoute: String?

  @objc public override func load(webview: WKWebView) {
    ShviaPushPlugin.shared = self
    injectAppDelegateHooks()
    UNUserNotificationCenter.current().delegate = self
  }

  /// Adiciona `didRegisterForRemoteNotifications.../didFail...` na classe do
  /// AppDelegate do tao. `class_addMethod` retorna false se o seletor já
  /// existir (ex.: uma futura versão do tao implementá-lo) — nesse caso NÃO
  /// sobrescrevemos e registramos o erro, para o problema aparecer no canal em
  /// vez de sumir silenciosamente.
  private func injectAppDelegateHooks() {
    guard let delegate = UIApplication.shared.delegate else { return }
    let cls: AnyClass = type(of: delegate)

    let didRegisterSel = sel_registerName("application:didRegisterForRemoteNotificationsWithDeviceToken:")
    let didRegisterBlock: @convention(block) (AnyObject, UIApplication, NSData) -> Void = { _, _, tokenData in
      let hex = (tokenData as Data).map { String(format: "%02.2hhx", $0) }.joined()
      ShviaPushPlugin.shared?.deliverToken(hex)
    }
    let addedRegister = class_addMethod(
      cls, didRegisterSel, imp_implementationWithBlock(didRegisterBlock as Any), "v@:@@")

    let didFailSel = sel_registerName("application:didFailToRegisterForRemoteNotificationsWithError:")
    let didFailBlock: @convention(block) (AnyObject, UIApplication, NSError) -> Void = { _, _, error in
      ShviaPushPlugin.shared?.deliverError(error.localizedDescription)
    }
    let addedFail = class_addMethod(
      cls, didFailSel, imp_implementationWithBlock(didFailBlock as Any), "v@:@@")

    if !addedRegister || !addedFail {
      deliverError("AppDelegate do tao já implementa os seletores de push — plugin precisa migrar de class_addMethod para swizzle")
    }
  }

  private func deliverToken(_ hex: String) {
    lastToken = hex
    lastError = nil
    try? tokenChannel?.send(["type": "token", "token": hex])
  }

  private func deliverError(_ message: String) {
    lastError = message
    try? tokenChannel?.send(["type": "error", "message": message])
  }

  private func deliverRoute(_ route: String) {
    if tokenChannel != nil {
      try? tokenChannel?.send(["type": "route", "route": route])
    } else {
      // Tap em notificação com o app FRIO: o canal ainda não foi registrado
      // pelo Rust — guarda a rota e entrega no watchToken.
      pendingRoute = route
    }
  }

  /// Comando (Rust): registra o canal de eventos. Reentrega token/rota que
  /// tenham chegado antes do canal existir (cold start).
  @objc public func watchToken(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(WatchArgs.self)
    tokenChannel = args.channel
    if let t = lastToken { try? tokenChannel?.send(["type": "token", "token": t]) }
    if let e = lastError { try? tokenChannel?.send(["type": "error", "message": e]) }
    if let r = pendingRoute {
      try? tokenChannel?.send(["type": "route", "route": r])
      pendingRoute = nil
    }
    invoke.resolve()
  }

  /// Comando (Rust): pede a autorização e, concedida, registra no APNs.
  /// Idempotente — permissão já decidida não gera novo prompt; o registro
  /// re-executado só renova o token (e o servidor faz upsert por token).
  @objc public func requestPermission(_ invoke: Invoke) throws {
    UNUserNotificationCenter.current().requestAuthorization(options: [.alert, .badge, .sound]) { granted, _ in
      if granted {
        DispatchQueue.main.async {
          UIApplication.shared.registerForRemoteNotifications()
        }
      }
      invoke.resolve(["granted": granted])
    }
  }

  // Notificação chegando com o app em PRIMEIRO plano: mostra mesmo assim
  // (banner + som) — sem isto o push some quando o usuário está no app.
  public func userNotificationCenter(
    _ center: UNUserNotificationCenter,
    willPresent notification: UNNotification,
    withCompletionHandler completionHandler: @escaping (UNNotificationPresentationOptions) -> Void
  ) {
    if #available(iOS 14.0, *) {
      completionHandler([.banner, .sound, .badge])
    } else {
      completionHandler([.alert, .sound, .badge])
    }
  }

  // Tap na notificação: extrai a rota do payload (`data.route` no servidor
  // vira `userInfo["route"]` aqui) e sobe pro Rust navegar.
  public func userNotificationCenter(
    _ center: UNUserNotificationCenter,
    didReceive response: UNNotificationResponse,
    withCompletionHandler completionHandler: @escaping () -> Void
  ) {
    let userInfo = response.notification.request.content.userInfo
    if let route = userInfo["route"] as? String {
      deliverRoute(route)
    }
    completionHandler()
  }
}

@_cdecl("init_plugin_shvia_push")
func initPluginShviaPush() -> Plugin {
  return ShviaPushPlugin()
}
