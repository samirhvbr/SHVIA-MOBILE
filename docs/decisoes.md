# ShvIA Mobile — Decisões (ADRs)

Formato ADR. Não relitigar direção já decidida dentro de um how-to — linkar o ADR.

---

## ADR-001 — Repo separado, stack Tauri, reusando a infra de loja da Blue3

- **Data:** 07/07/2026 · **Status:** Aceito
- **Contexto:** Levar o ShvIA para **iOS + Android**. O desktop (`SHVIA-DESKTOP`)
  já é Tauri 2 (shell fino que carrega `ai.shvia.org`). A Blue3 **já publica**
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

---

## ADR-002 — M3 (valor nativo p/ 4.2): sequência por tratabilidade, não por impacto

- **Data:** 16/07/2026 · **Status:** Aceito
- **Contexto:** Samir **reafirmou o Caminho B** (App Store **pública**) em 16/07,
  largando de vez a distribuição privada por **Business Manager** (as "2 semanas com
  aparelho na mesa"). Público ⇒ a regra **4.2 (web-wrapper)** se aplica ao ShvIA
  (casca fina) ⇒ **M3 é pré-requisito da submissão iOS** — no Android/Play não (o
  Play tolera wrapper). O problema: os itens do M3 têm dependências externas **muito
  diferentes**, e tratá-los na ordem errada trava tudo atrás do recurso mais caro.
- **Decisão:** sequenciar o M3 pelo que **destrava sozinho** primeiro:
  1. **Biometria (Face ID / Touch ID / BiometricPrompt)** — **100% na casca**, sem
     servidor, sem portal Apple além da assinatura normal. **FEITO PRIMEIRO** (mobile
     0.4.0). `tauri-plugin-biometric`; o gate roda na página **local** (`src/`) —
     única com bridge nativo (a página remota **não** recebe comando, ADR-001) — e
     trava o cold-start antes de navegar pro ShvIA hospedado. Espelha o modelo Blue3
     ([BIOMETRIA.md](../../BLUE3-INTRANET-MOBILE/docs/MOBILE/BIOMETRIA.md)): a
     biometria é **acesso LOCAL**; o cookie de sessão same-origin continua sendo o
     auth remoto. Toggle de ativar/desativar vive na casca (não há como ser no ShvIA
     web sem furar o posture).
  2. **Push (APNs)** — **bloqueado em recursos externos** (do Samir/servidor):
     `.p8` (portal Apple ▸ Keys), capability **Push** + **App Group** + entitlement
     `aps-environment` no App ID, **e** endpoint no **SHVIA-WEB** (guardar token +
     enviar). Reuso concreto: a `.p8` é **por Team `S65UBCTPN5`**, não por app → a
     mesma chave da Blue3 serve, muda só o bundle p/ `cloud.blue3.shvia`. Envio:
     espelhar o **ES256/`firebase/php-jwt`** do serviço D9 da Blue3
     ([BLUE3-MOBILE-SERVICOS-AO-VIVO.md]) **ou** FCM como o BLUE3-INTRANET-MOBILE
     ([NOTIFICACOES.md]). É "o gancho mais forte" da 4.2, mas **não fecha só na casca**.
  3. **Deep-link / Universal Links** — **bloqueado**: exige
     `apple-app-site-association` no **SHVIA-WEB** + entitlement `associated-domains`
     (e `assetlinks.json` no Android). Custom scheme `shvia://` é parcial e fraco
     sozinho — Universal Links é o que vale, e depende do servidor.
  4. Câmera+mic (feito), tela offline nativa (feita — reforçar), share sheet (depois).
- **Consequências:** dá pra **avançar o M3 hoje** sem depender de nada externo
  (biometria entregue na 0.4.0). Push e Universal Links entram quando o Samir
  liberar `.p8` + portal + os endpoints no SHVIA-WEB. Na review, citar esses
  recursos nas notas (checklist §2.1).
- **Verificação pendente:** o gate biométrico só se valida **no aparelho/simulador
  iOS** (host só faz `cargo check` + `tsc`); Face ID real precisa do Mac + device.

---

## ADR-003 — Domínio próprio `ai.shvia.org`: dual-host e fim do curinga em `is_internal`

- **Contexto/Problema:** o ShvIA saiu de `ia.blue3.com.br` para `ai.shvia.org` (a
  Blue3 ficou como financiadora, não como marca do produto). A casca mobile é
  mantida em **paridade** com a do SHVIA-DESKTOP e tinha os mesmos três pontos
  blocantes: `SHVIA_URL` (o destino), a allowlist de navegação interna e o
  `connect-src` da CSP. Nenhum deles falha de forma legível — host fora da
  allowlist faz `on_navigation` tratar a **navegação inicial** como link externo
  (o ShvIA abre no Safari e o app fica preso no splash); host fora do
  `connect-src` faz o ping de alcance ser barrado pelo WebView, e a casca fica em
  "Sem conexão" para sempre com o servidor no ar.
- **Decisão:** dual-host por allowlist **exata** de FQDN, espelhando
  `SERVER_HOSTS` do desktop: `ai.shvia.org` (canônico, o que o app abre),
  `ia.shvia.org` (CNAME do canônico) e `ia.blue3.com.br` (legado, mesmo IP, só
  durante a transição). Verificadas por DNS. Desligar o legado é remover uma
  linha.
- **Fim do curinga — mudança de COMPORTAMENTO, não só de host:** o
  `is_internal` daqui aceitava `host.ends_with(".blue3.com.br")`, isto é,
  **qualquer** subdomínio do domínio corporativo carregava dentro do app. O
  desktop fechou isso no 0.9.0 dele; o mobile herda a mesma postura agora, e
  passou a exigir **esquema** também (`https` para o servidor, `http`/`tauri`
  para a casca local). O ápex `shvia.org` fica FORA de propósito: resolve para
  outro IP e serve a landing, não o app. Coberto por 3 testes de unidade
  (`cargo test`), incluindo o sufixo-armadilha `ai.shvia.org.evil.com`.
- **Marca no splash:** o `<span>Blue3</span>` saiu e o `brand-mark.png` (a seta
  da Blue3) virou `brand-mark.svg`, a mesma marca do favicon e do badge do web. O
  splash é **pré-login**: a Blue3 só pode aparecer depois do login e só para
  e-mail dela (SHVIA-WEB/`config/brand.php`). O rodapé passou a mostrar só
  `v<versão>` — que é o que interessa saber num teste via TestFlight.
- **O `identifier` `cloud.blue3.shvia` NÃO muda.** Além de zerar dados por
  sandbox, desde a 2.51.0 do SHVIA-WEB o `APNS_BUNDLE_ID` tem de ser igual ao
  bundle id — renomear quebraria push no iOS em silêncio. E trocar bundle id de
  app publicado é app novo na loja, não atualização.
- **Consequências/limites — a ordem importa mais aqui que no desktop:** a
  atualização do mobile passa por **review da Apple**, com latência de dias. O
  build com o host novo tem de ser **submetido e distribuído ANTES** de qualquer
  redirect ou desligamento do host antigo; na ordem inversa, todo aparelho com a
  versão publicada fica sem app até o review sair. Trocar de origem também
  desloga uma vez (cookie é por origem) e zera `localStorage`/`IndexedDB` da
  WebView. Verificação: `cargo test` + `npm run build` no host; o comportamento
  de link externo (item 7 do `docs/smoke-test.md`) só se valida no aparelho.

[BLUE3-MOBILE-SERVICOS-AO-VIVO.md]: ../../BLUE3-INTRANET-MOBILE/docs/BLUE3-MOBILE-SERVICOS-AO-VIVO.md
[NOTIFICACOES.md]: ../../BLUE3-INTRANET-MOBILE/docs/NOTIFICACOES.md
