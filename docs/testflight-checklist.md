# TestFlight & App Store — checklist de submissão iOS

> Fecha o M2 (TestFlight) e prepara o M4 (loja). O app é **shell fino Tauri 2**
> (`cloud.blue3.shvia`) carregando `ia.blue3.com.br`. Conta Apple da Blue3 já
> existe — **Team ID `S65UBCTPN5`**. Build **só no Mac** (Xcode).
>
> Legenda: `[x]` feito · `[ ]` falta · `[Mac]` exige o MacBook · `[ASC]` no App
> Store Connect (web).

---

## 🔑 Insight que muda a ordem

**TestFlight interno NÃO passa pela review da regra 4.2.** Testers internos (até
100 membros da equipe/Blue3, adicionados por e-mail no App Store Connect) recebem
o build **sem beta review**. A temida rejeição "4.2 web-wrapper" só acontece na
**submissão à App Store pública** (Parte 2) — e mesmo lá há um atalho (ver §2.1).

Ou seja: dá pra **subir pro TestFlight interno agora**, testar no aparelho de
verdade, e só depois decidir o caminho da loja. As duas partes abaixo refletem isso.

---

## Parte 1 — TestFlight interno (objetivo AGORA, fecha o M2)

### 1.1 Identidade & assinatura
- [x] `bundle.iOS.developmentTeam = S65UBCTPN5` (`tauri.conf.json`)
- [x] Bundle ID `cloud.blue3.shvia` (mesmo do desktop)
- [x] `minimumSystemVersion: "14.0"` + `category: productivity` (0.3.11)
- [x] Ícones iOS gerados (`src-tauri/icons/ios/`, desde 0.2.2)
- [ ] `[Mac]` Xcode logado na conta Apple da Blue3 (Settings ▸ Accounts) e, na
      1ª vez, abrir `gen/apple/*.xcodeproj` ▸ Signing & Capabilities pra confirmar
      o time e deixar o Xcode criar o provisioning automático.

### 1.2 Privacidade & conformidade (exigido pelo App Store Connect no upload)
- [x] `NSMicrophoneUsageDescription` + `NSCameraUsageDescription` (`Info.ios.plist`,
      textos em pt-BR) — mesclados no Info.plist do build
- [x] `ITSAppUsesNonExemptEncryption = false` (`Info.ios.plist`) — só HTTPS padrão;
      evita a pergunta de exportação a **cada** upload
- [x] `PrivacyInfo.xcprivacy` criado (`gen/apple/shvia-mobile_iOS/`) — sem tracking,
      sem tipos de dado coletados pelo binário, UserDefaults (CA92.1)
- [ ] `[Mac]` **Conferir que o `PrivacyInfo.xcprivacy` está no target** do Xcode
      (TARGETS ▸ Build Phases ▸ Copy Bundle Resources). Arquivo criado no disco
      não entra no bundle sozinho — se faltar, arrastar pro navegador do Xcode
      marcando o target `shvia-mobile_iOS`.

### 1.3 Versão / build number
- [ ] `[Mac]` **Confirmar que o `.ipa` sai com a versão certa.** O
      `Info.plist` gerado ainda tem `CFBundleShortVersionString`/`CFBundleVersion`
      **literais `0.2.6`** (congelados no `tauri ios init`). O `tauri ios build`
      normalmente reescreve a partir do `version.md`, mas **verificar no Organizer
      do Xcode** que subiu como a versão atual (0.3.x), não 0.2.6.
- Política de build number: cada upload ao TestFlight precisa de `CFBundleVersion`
  **único e crescente**. Como todo commit do repo bumpa o `version.md`, cada build
  já tende a ser único. Regra: **bump o Z antes de cada upload**, mesmo sem código
  novo (rebuild). Se preferir travar, setar `bundle.iOS.bundleVersion` (inteiro
  que você incrementa à mão) no `tauri.conf.json`.

### 1.4 Build & upload
- [ ] `[Mac]` Smoke-test final no simulador (itens 4–7: TTS/mic/anexos/links) —
      ver [smoke-test.md](smoke-test.md). Landscape (item 9) já validado com a
      SHVIA-WEB 2.19.2 no ar.
- [ ] `[Mac]` Build de distribuição:
      ```bash
      npm run tauri ios build -- --export-method app-store-connect
      ```
- [ ] `[Mac]` Upload (uma das duas):
      - **Xcode Organizer** (Window ▸ Organizer ▸ Distribute App) — mais simples na 1ª vez; ou
      - `altool` com chave de API do App Store Connect:
        ```bash
        xcrun altool --upload-app --type ios \
          --file "src-tauri/gen/apple/build/arm64/ShvIA.ipa" \
          --apiKey "$APPLE_API_KEY_ID" --apiIssuer "$APPLE_API_ISSUER"
        ```
- [ ] `[ASC]` Adicionar testers internos (aba TestFlight ▸ Internal Testing) por
      e-mail. Processam **sem** beta review.
- [ ] Instalar o app TestFlight no iPhone físico e rodar o smoke-test on-device
      (o simulador não cobre mic real, câmera real, nem persistência de cookie
      igual ao aparelho).

> ✅ Com isso o **M2 fecha**: build assinado rodando no aparelho via TestFlight.

---

## Parte 2 — App Store pública (M4, quando for a hora)

### 2.1 DECISÃO (15/07): Caminho B — App Store pública, igual ao BLUE3-INTRANET-MOBILE
**Samir decidiu**: mesmo trilho do app Flutter da Blue3, que publica na **App Store
pública** via `method: app-store-connect` na conta **`S65UBCTPN5`** (confirmado no
`~/x/BLUE3-INTRANET-MOBILE/docs/Runner .../ExportOptions.plist`). **Sem custo novo:**
a conta Apple Developer paga da Blue3 já cobre todos os apps da organização.
Detalhe reusável dali: `manageAppVersionAndBuildNumber: true` (Xcode cuida do build
number → resolve o gotcha do §1.3) e `signingStyle: automatic`, `teamID S65UBCTPN5`.

⚠️ **PORÉM — o precedente do BLUE3 não elimina a 4.2 pro ShvIA.** O BLUE3 é **Flutter
nativo** (push, secure storage, Live Activities) → passou pela 4.2 sem esforço porque
NÃO é web-wrapper. O ShvIA é **shell fino** (WebView do site) → **a regra 4.2 SE
APLICA a ele**. Logo, ir pra App Store pública **torna o M3 (valor nativo)
obrigatório** — não é mais opcional. (Distribuição privada via Business Manager, que
pularia a 4.2, foi descartada por essa decisão.)

**Valor nativo mínimo pra não bater na 4.2 (= M3):**
  - [ ] Push (APNs) — o gancho mais forte de "não é só um site". **Reusar a infra
        APNs da Blue3** (o BLUE3 já usa `.p8` KeyID/TeamID, App Group, flag
        `APNS_PRODUCTION`; ver `BLUE3-INTRANET-MOBILE/docs/.../SERVICOS_ESPORTES.md`)
  - [ ] Deep-link `shvia://` + Universal Links (abre conversa/projeto direto)
  - [ ] Biometria (Face ID) pra desbloquear o app / re-login
  - [x] Câmera + microfone (já declarados/usados)
  - [~] Tela offline nativa (a casca já tem a tarja + auto-retry — reforçar)
  - [ ] Compartilhar (share sheet) recebendo conteúdo de outros apps
  - Na review, **descrever esses recursos nativos nas notas** + conta de teste,
    deixando claro que é ferramenta corporativa, não um webview genérico.

### 2.2 Ficha na loja `[ASC]`
- [ ] Nome, subtítulo, descrição, palavras-chave, categoria (productivity)
- [ ] **Nutrition label** (App Privacy): declarar que os dados digitados vão pro
      servidor Blue3 (first-party). É AQUI, não no `PrivacyInfo.xcprivacy`.
- [ ] Ícone 1024×1024 da loja (`brand/shvia-desktop-icon-1024.png` como base)
- [ ] Screenshots por tamanho de tela (6.9"/6.5"/5.5" + iPad se suportar) —
      capturar no simulador com os fluxos reais (chat, projeto, resposta streamando)
- [ ] URL de política de privacidade + URL de suporte
- [ ] Classificação etária

### 2.3 Primeira submissão — iterar no privacy manifest
- [ ] Após o 1º upload, **ler os e-mails do App Store Connect**: se o binário tocar
      outra required-reason API sem declaração (FileTimestamp `C617.1`,
      SystemBootTime `35F9.1`, DiskSpace `E174.1`), a Apple avisa qual. Adicionar
      o bloco no `PrivacyInfo.xcprivacy` e reenviar. O manifest atual cobre só
      UserDefaults de propósito (mínimo seguro).

---

## Ordem recomendada
1. **[Mac]** Fechar smoke-test 4–7 no simulador → subir **TestFlight interno**
   (Parte 1) → validar no iPhone físico. **← fecha o M2.**
2. Samir decide **Caminho A vs B** da §2.1 (interno privado × loja pública).
3. Executar a Parte 2 conforme a decisão.

> Relacionado: [smoke-test.md](smoke-test.md) · [decisoes.md](decisoes.md) (ADR-001)
> · roadmap em [../.continue/escopo-mobile.md](../.continue/escopo-mobile.md).
