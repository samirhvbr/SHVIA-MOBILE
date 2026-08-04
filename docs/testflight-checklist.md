# TestFlight & App Store — checklist de submissão iOS

> Fecha o M2 (TestFlight) e prepara o M4 (loja). O app é **shell fino Tauri 2**
> (`cloud.blue3.shvia`) carregando `ai.shvia.org`. Conta Apple da Blue3 já
> existe — **Team ID `S65UBCTPN5`**. Build **só no Mac** (Xcode).
>
> Legenda: `[x]` feito · `[ ]` falta · `[Mac]` exige o MacBook · `[ASC]` no App
> Store Connect (web) · `[Linux]` dá pra fazer nesta máquina.

---

## §0 — Runbook do dia da publicação (revisão de 30/07)

> Ordem de execução, do que **trava o upload** pro que só melhora a chance de
> passar. Cada item aponta pra seção que o detalha. O que a revisão de 30/07
> achou de novo está no [roadmap](../.continue/escopo-mobile.md) §5.

**O que já está pronto e verificado hoje (30/07, no host Linux):** `cargo check`
✓ · `cargo test` 3/3 ✓ · `tsc` + `vite build` ✓ · `npm audit` 0 vulnerabilidades ✓
· servidor `ai.shvia.org/up` 200 em 32 ms ✓ · ícones sem canal alpha (18/18,
incluindo o 1024) ✓ · Team ID, bundle, `minimumSystemVersion` e categoria ✓.

### Bloqueio duro — sem isto o App Store Connect não deixa submeter
- [~] **Política de privacidade e página de suporte — ESCRITAS em 30/07**, no
      `SHVIA-SITE` 0.4.0 (`public/privacidade.html` e `public/suporte.html`).
      URLs que vão para a ficha da loja:
      - Privacidade: `https://shvia.org/privacidade.html`
      - Suporte: `https://shvia.org/suporte.html`

      **Falta, e é do Samir:**
      - [x] 🟢 **Razão social + CNPJ — PREENCHIDOS em 31/07 (site 0.4.1, pushed).**
            `BLUE3 TECNOLOGIA LTDA`, CNPJ `19.648.136/0001-30` — razão social
            confirmada pelo certificado Apple da conta S65UBCTPN5; CNPJ do
            registro público (sócio-administrador: Samir Hanna Verza). Harness
            do site verde (124 checks). **Conferir na leitura final antes do
            deploy** — identificação legal é do Samir por definição.
      - [ ] 🔴 **Criar as caixas `privacidade@shvia.org` e `suporte@shvia.org`.**
            As duas são citadas nas páginas e vão para a ficha da loja; o revisor
            da Apple às vezes escreve para o suporte durante a review.
      - [ ] Publicar: `cd /srv/www/shvia.org && bash deploy.sh` e **abrir as duas
            URLs anônimo** (o revisor abre sem login).

      Conteúdo fundamentado no código real do SHVIA-WEB, não em modelo genérico:
      colunas de `users`, tabelas de conteúdo e de uso, BYOK por usuário,
      `data_locality` (on-prem / EUA / China), zeragem do RAG fora do on-prem,
      `maskOutboundPii`, toggle `lgpd_strict` e a retenção de 180 dias agendada
      em `routes/console.php`. A tensão com o ADR-001 do site ("nada de Blue3 na
      landing") foi resolvida pelo **ADR-014** de lá: a exceção vale só para o
      parágrafo de identificação legal.
      - [ ] ⚠️ **Confirmar que o scheduler roda em produção.** A política promete
            expurgo em 180 dias e os comandos estão agendados, mas o
            `docs/PUSH/PUSH-APNS-20260716.md` registra o cron/queue como
            "pendente de ativação em prod". Retenção documentada que não executa
            é exposição de LGPD — verificar antes de a página ir ao ar.

### Correção de raiz — FEITA em 31/07 no Mac (0.5.4)
- [x] `[Mac]` **`npm run tauri ios init` rodado em 31/07** (instalou xcodegen
      2.46 e regenerou o `.xcodeproj`). Resultado:
      **(b)** ✅ `PrivacyInfo.xcprivacy` entrou no alvo (PBXBuildFile "in
      Resources" no `project.pbxproj` — conferido por grep, 4 refs) e **está no
      payload do IPA** (auditado por unzip);
      **(c)** ✅ versão vem das fontes (0.5.x). **Refinado em 01/08:** o
      `tauri ios build` INJETA a versão do `tauri.conf.json` no Info.plist do
      archive (um build com gen/apple em 0.5.4 e conf em 0.5.6 produziu IPA
      0.5.6) — o literal do `gen/apple` importa menos do que se acreditava;
      manter os dois em sincronia mesmo assim (XcodeGen usa o project.yml).
      **(a)** ⚠️ **DESCOBERTA que muda a regra: o `ios init` NÃO mescla o
      `Info.ios.plist` no `gen/apple/.../Info.plist`** (a crença da 0.3.0 não
      vale mais). O init de 31/07 REMOVEU as 4 chaves (mic, câmera, Face ID,
      encryption) do arquivo gerado — diff só-deleções; restauradas via
      `git checkout` do 0.5.3. **Regra nova: após QUALQUER `ios init`, conferir
      as 4 chaves no `gen/apple/shvia-mobile_iOS/Info.plist` e restaurar se
      preciso.** As chaves ficam nos DOIS arquivos (fonte + gerado), de propósito.
- [x] `tauri icon` **NÃO foi rodado, de propósito**: o init não tocou o
      `Assets.xcassets` (ícones 18/18 sem alpha preservados) nem o Android —
      rodar de novo só reintroduziria o gotcha do alpha e o reset do navy.
- [x] `PrivacyInfo.xcprivacy` em Copy Bundle Resources — conferido no pbxproj e
      no IPA final (não foi preciso abrir o Xcode GUI).
- [x] **Gotcha novo (31/07): o cache do `target/` com caminhos velhos morde CADA
      target/perfil separadamente** — `failed to read plugin permissions
      .../x/SHVIA-MOBILE/...` (caminho antigo). Apareceu no release de device
      (`aarch64-apple-ios/release`) E depois de novo no dev de simulador
      (`aarch64-apple-ios-sim/debug`). Cura de uma vez, cobrindo host + device +
      sim, debug + release:
      ```bash
      cd src-tauri/target
      rm -rf {debug,release}/{build,.fingerprint} \
             aarch64-apple-ios/{debug,release}/{build,.fingerprint} \
             aarch64-apple-ios-sim/{debug,release}/{build,.fingerprint}
      ```
- [x] `[Mac]` **Build de distribuição OK (31/07; refeito 04/08 como 0.6.1):**
      `npm run tauri ios build -- --export-method app-store-connect` →
      `src-tauri/gen/apple/build/arm64/ShvIA.ipa`.
- [x] **Gotcha 04/08 (Apple recusou o upload do 0.6.0 com "Invalid bundle
      structure"): `tauri ios init` rodado com o `Externals/libapp.a` JÁ
      construído faz o XcodeGen copiar a lib de 56 MB para o Copy Bundle
      Resources → `ShvIA.app/libapp.a` no IPA → 409 no Transporter.** Cura de
      raiz no project.yml: `excludes: ["**/*.a"]` no source `Externals` (o
      LINK é o `dependencies: framework libapp.a`, não é afetado). O init de
      14/07 escapou por sorte: a lib ainda não existia na árvore.
- [x] **Gotcha 04/08 (nº 2): o `.entitlements` é GERADO pelo XcodeGen — edição
      manual no arquivo morre no próximo `xcodegen generate`** (foi o que
      apagou o `aps-environment` do primeiro build 0.6.1). Fonte da verdade:
      bloco `entitlements.properties` do project.yml.
- [ ] **AUDITORIA PRÉ-UPLOAD (obrigatória antes de todo Deliver):**
      ```bash
      IPA=src-tauri/gen/apple/build/arm64/ShvIA.ipa
      unzip -l $IPA | grep -cE 'libapp|\.a$'        # TEM que ser 0
      unzip -l $IPA | grep -c PrivacyInfo            # TEM que ser 1
      mkdir -p /tmp/ipa && unzip -qo $IPA -d /tmp/ipa
      codesign -d --entitlements :- /tmp/ipa/Payload/ShvIA.app | grep aps-environment  # production
      plutil -p /tmp/ipa/Payload/ShvIA.app/Info.plist | grep -E 'ShortVersion|FaceID|NonExempt'
      rm -rf /tmp/ipa
      ```

### Validação no aparelho — o que nunca rodou em iOS
- [ ] `[Mac]` **Smoke-test item 10 (biometria) PRIMEIRO** — Face ID é o caminho
      que nenhum build em aparelho exercitou (o 0.3.13 que foi pro TestFlight é
      **anterior** à 0.4.0) e é a primeira tela que o revisor vê.
- [ ] `[Mac]` Itens 4–7 (TTS, mic, anexos, links externos) no iPhone físico —
      mic e câmera reais, que o simulador fingia. Ver [smoke-test.md](smoke-test.md).

### Publicação
- [ ] `[Mac]` Build + upload (§1.4) → `[ASC]` TestFlight interno (sem beta review).
- [ ] `[ASC]` Ficha da loja: metadados, screenshots, nutrition label, classificação
      etária (§2.2).
- [x] **DECISÃO (Samir, 31/07): SEGURAR ~1 semana e submeter COM push.** Alvo:
      **07/08/2026**. Tracking: [issue #1](https://github.com/samirhvbr/SHVIA-MOBILE/issues/1)
      (checklist da semana + status diário comentado por rotina agendada às 9h)
      + lembrete no Calendar em 07/08. Caixas `privacidade@`/`suporte@shvia.org`
      criadas em 31/07. TestFlight interno segue valendo AGORA (IPA 0.5.4 no
      Transporter) para o smoke on-device enquanto o push é construído.

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
- [x] `NSMicrophoneUsageDescription` + `NSCameraUsageDescription` — no
      `src-tauri/Info.ios.plist` (fonte) **e** no `gen/apple/.../Info.plist`
      (o que vai pro bundle)
- [x] `NSFaceIDUsageDescription` — **reconciliado na 0.5.3.** Estava só na fonte:
      o `gen/apple/.../Info.plist` é de 14/07 e a chave entrou na 0.4.0 (16/07).
      ⚠️ Sem ela o iOS **encerra o processo** na 1ª chamada de `evaluatePolicy` —
      neste app, o botão "Ativar Face ID" da 1ª execução. Nenhum build em aparelho
      passou por esse caminho ainda (o 0.3.13 do TestFlight é anterior à biometria).
- [x] `ITSAppUsesNonExemptEncryption = false` — mesma história: entrou na 0.3.11,
      nunca chegou ao `gen/apple`; reconciliado na 0.5.3. Evita a pergunta de
      conformidade de exportação a **cada** upload.
- [x] `PrivacyInfo.xcprivacy` criado (`gen/apple/shvia-mobile_iOS/`) — sem tracking,
      sem tipos de dado coletados pelo binário, UserDefaults (CA92.1)
- [ ] 🔴 `[Mac]` **O `PrivacyInfo.xcprivacy` NÃO está no target — confirmado em
      30/07.** `grep PrivacyInfo gen/apple/shvia-mobile.xcodeproj/project.pbxproj`
      não retorna nada: o `.xcodeproj` foi gerado em 14/07, **antes** de o arquivo
      existir (0.3.11). Ou seja, o privacy manifest **não entra no bundle** hoje.
      Cura limpa: `tauri ios init` (o `project.yml` já lista `shvia-mobile_iOS`
      como source — falta o XcodeGen rodar). Alternativa manual: arrastar pro
      navegador do Xcode marcando o target `shvia-mobile_iOS`. Depois conferir em
      TARGETS ▸ Build Phases ▸ Copy Bundle Resources.

### 1.3 Versão / build number
- [x] **Causa-raiz do "congelado em 0.2.6" entendida (30/07).** Não é o build que
      esquece de reescrever: é que `gen/apple/shvia-mobile_iOS/Info.plist` —
      o `INFOPLIST_FILE` do alvo, conferido no `project.pbxproj` — foi gerado uma
      única vez, no `tauri ios init` de 14/07, quando o app estava em 0.2.6, e
      **nunca mais foi refeito**. O `project.yml` carrega o mesmo literal. Ambos
      atualizados à mão na 0.5.3.
- [ ] `[Mac]` **Confirmar no Organizer do Xcode** que o `.ipa` subiu com a versão
      atual (0.5.x), não 0.2.6. `CFBundleVersion` precisa ser **único e crescente**
      entre uploads — `0.5.3` > `0.3.13` na comparação por componentes da Apple
      (5 > 3), então a sequência do TestFlight anterior está preservada.
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
`~/x/BLUE3/BLUE3-INTRANET-MOBILE/docs/Runner .../ExportOptions.plist`). **Sem custo novo:**
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
  - [~] Push (APNs) — o gancho mais forte de "não é só um site". **Reuso concreto
        da Blue3:** o BLUE3 usa **APNs token-based** (`.p8` + KeyID + TeamID via
        `firebase/php-jwt` ES256, sem certificado). A **`.p8` é por CONTA Apple
        (Team ID), não por app** → como o ShvIA usa o mesmo Team `S65UBCTPN5`, **a
        MESMA chave serve**; só muda o `APNS_BUNDLE_ID` p/ `cloud.blue3.shvia`.
        Cruza 3 pontos — **estado real em 28/07:**
        - [x] **(b) SHVIA-WEB — FEITO na 2.51.0 (16/07).** `POST`/`DELETE
              /push/token` pela sessão same-origin (`{token, platform}`),
              `ApnsClient` (JWT ES256, HTTP/2), `PushService::sendToUser()`, job
              `SendPushNotification`, `php artisan push:test {user}`,
              `config/apns.php`, 11 testes verdes. Contrato completo em
              `~/x/SHVIA/SHVIA-WEB/docs/PUSH/PUSH-APNS-20260716.md`.
        - [ ] **(a) shell Tauri (ESTE repo)** — entitlement `aps-environment`,
              pedir permissão, `registerForRemoteNotifications`, injetar o device
              token na página remota (mesmo `webview.eval` do
              `window.__shviaShellVersion`) e navegar pro `data.route` no tap.
              **App Group NÃO é necessário** para alerta simples — só entraria com
              Notification Service Extension.
        - [ ] **(a2) front do SHVIA-WEB** — ler `window.__shviaPushToken` → `POST
              /push/token` com cookie + CSRF; `DELETE` no logout. **Reconferido
              em 30/07 (SHVIA-WEB 2.88.8): `grep __shviaPushToken public/js/app.js`
              não acha nada** — o `app.js` só lê o `__shviaShellVersion` (linha
              ~9185). Continua sem existir.
        - [ ] **(c) portal/infra (Samir)** — capability **Push Notifications** no
              App ID `cloud.blue3.shvia` (é o que muda por app; a `.p8` não) e as
              `APNS_*` no `.env` de produção. ⚠️ `APNS_PRODUCTION`: **false** p/
              build debug (token de sandbox), **true** p/ TestFlight/loja — causa
              nº 1 de "push não chega".
        Refs: `~/x/BLUE3/BLUE3-INTRANET-MOBILE/docs/BLUE3-MOBILE-SERVICOS-AO-VIVO.md`
        (D9, .env APNS_*) e `docs/MOBILE/SERVICOS_ESPORTES.md`.
  - [ ] Deep-link `shvia://` + Universal Links (abre conversa/projeto direto)
  - [x] **Biometria (Face ID/Touch ID) — mobile 0.4.0, build-verified.** Gate local
        (opt-in 1ª execução + lock no cold-start) via `tauri-plugin-biometric`. Falta
        só o smoke-test on-device (Face ID real). Ver ADR-002 e escopo M3.
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
      capturar no simulador com os fluxos reais (chat, projeto, resposta streamando).
      ⚠️ A revisão de 28/07 lembrou: **Modo Code, memória e os overlays de saída
      nunca passaram por um olho mobile** — conferir antes de virar screenshot.
- [~] **URL de política de privacidade** + **URL de suporte** — escritas em
      30/07 no `SHVIA-SITE` 0.4.0: `https://shvia.org/privacidade.html` e
      `https://shvia.org/suporte.html`. Faltam CNPJ, caixas de e-mail e deploy —
      detalhes e pendências no **§0** deste arquivo.
- [ ] Classificação etária

### 2.3 Primeira submissão — iterar no privacy manifest
- [ ] Após o 1º upload, **ler os e-mails do App Store Connect**: se o binário tocar
      outra required-reason API sem declaração (FileTimestamp `C617.1`,
      SystemBootTime `35F9.1`, DiskSpace `E174.1`), a Apple avisa qual. Adicionar
      o bloco no `PrivacyInfo.xcprivacy` e reenviar. O manifest atual cobre só
      UserDefaults de propósito (mínimo seguro).

---

## Ordem recomendada

Substituída pelo **§0 — Runbook do dia da publicação** no topo deste arquivo
(30/07). A ordem antiga terminava em "Samir decide Caminho A vs B", decisão já
tomada em 15/07 (Caminho B, App Store pública — §2.1).

> Relacionado: [smoke-test.md](smoke-test.md) · [decisoes.md](decisoes.md) (ADR-001)
> · roadmap em [../.continue/escopo-mobile.md](../.continue/escopo-mobile.md).
