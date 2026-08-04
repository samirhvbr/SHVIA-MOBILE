# Ficha da App Store — ShvIA (cloud.blue3.shvia)

> Rascunho pronto-para-colar dos campos do App Store Connect (04/08/2026).
> Fundamentado no produto real: shell Tauri do `ai.shvia.org` com Face ID,
> push (APNs), câmera/mic e tela offline nativa. Ajuste o tom à vontade —
> os LIMITES de caracteres são da Apple e estão anotados.

## 1. Informações do app (App Information)

| Campo | Valor | Limite |
|---|---|---|
| **Nome** | `ShvIA` | 30 |
| **Subtítulo** | `IA da sua operação, no bolso` | 30 |
| **Categoria primária** | Produtividade (já setada no binário) | — |
| **Categoria secundária** | Negócios | — |
| **URL de privacidade** | `https://shvia.org/privacidade.html` | — |
| **URL de suporte** | `https://shvia.org/suporte.html` | — |
| **Copyright** | `© 2026 BLUE3 TECNOLOGIA LTDA` | — |

## 2. Descrição (4000 chars — esta tem ~900)

```
O ShvIA é a plataforma de IA da sua operação — chat com modelos que rodam
na infraestrutura da empresa, base de conhecimento unificada, projetos e
histórico — agora no iPhone.

PRIVACIDADE EM PRIMEIRO LUGAR
• Modelos on-premise: por padrão, suas conversas são processadas na
  infraestrutura própria e não saem de lá.
• Você escolhe o modelo a cada conversa — inclusive provedores de nuvem,
  com transparência de onde o dado é processado.
• Face ID para proteger o acesso ao app.

FEITO PARA O TRABALHO
• Consulta à base de conhecimento da operação, com citações.
• Projetos com instruções e fontes próprias.
• Histórico de consumo por modelo, com custo e tokens.
• Notificações do que importa, direto no aparelho.
• Ditado por voz, anexos por câmera e leitura em voz alta.

CONTA NECESSÁRIA
O ShvIA é uma plataforma corporativa: o acesso é por conta criada pela
organização. Fale com o administrador da sua empresa ou visite shvia.org.
```

## 3. Keywords (100 chars, separadas por vírgula, sem espaço)

```
ia,chat,assistente,llm,empresa,privacidade,conhecimento,produtividade,gpt,copiloto
```
(97 chars. "gpt" é permitido como keyword genérica; se preferir zero risco, troque por "docs".)

## 4. Promotional text (170 chars — editável sem novo build)

```
IA privada da sua operação: modelos on-premise, base de conhecimento e
notificações — com Face ID. Suas conversas ficam na SUA infraestrutura.
```

## 5. App Privacy (nutrition label) — respostas do questionário

Base: `shvia.org/privacidade.html` (site 0.4.1). **"Do you collect data?" → YES.**

| Categoria ASC | Coleta? | Tipos | Vinculado à identidade? | Tracking? | Finalidade |
|---|---|---|---|---|---|
| **Contact Info** | Sim | Name, Email Address, Phone Number | **Sim** | Não | App Functionality (conta) |
| **User Content** | Sim | Other User Content (conversas, arquivos, projetos) | **Sim** | Não | App Functionality |
| **Identifiers** | Sim | User ID | **Sim** | Não | App Functionality |
| **Usage Data** | Sim | Product Interaction (registros de inferência: tokens, custo, latência) | **Sim** | Não | App Functionality, Analytics |
| Diagnostics | Sim | Crash Data? → **Não coletamos crash/telemetria própria** — responder NÃO | — | — | — |
| Location / Health / Financial / Contacts / Browsing | **Não** | — | — | — | — |

**Regras de ouro ao preencher:** Tracking = **NO em tudo** (nenhum dado cruza
apps/sites de terceiros para publicidade — o PrivacyInfo.xcprivacy já declara
`NSPrivacyTracking=false`). Data Broker = não. Os dados são coletados via
serviço próprio (o app é o cliente do ai.shvia.org).

## 6. Classificação etária (questionário) — sugestão de respostas

Tudo "None" (violência, sexual, drogas, horror, jogos de azar, etc.), com
duas atenções:

- **Unrestricted Web Access: NO** — a WebView é travada nos hosts do ShvIA
  (SERVER_HOSTS exatos, sem curinga; link externo abre no Safari FORA do app).
- **Gambling/Contests: NO.**

Resultado esperado: **4+**. Nota: se o revisor questionar conteúdo gerado por
IA, o argumento é que o acesso é por conta corporativa provisionada pela
organização (não é um chatbot aberto ao público) e há filtros no servidor.

## 7. App Review Information (o campo que mais derruba submissão)

- **Sign-in required: YES** → fornecer conta demo:
  - Username: `apple-review@shvia.org` *(criar no ShvIA antes do Submit —
    perfil comum, sem admin, com acesso a chat + um projeto de exemplo com
    2-3 documentos na base de conhecimento, para a review ver o produto real)*
  - Password: *(gerar forte e colar no ASC)*
- **Notes (sugestão, em inglês):**

```
ShvIA is a corporate AI workspace (B2B). Accounts are provisioned by each
organization's administrator — there is no public sign-up in the app; the
demo account above is pre-provisioned for review.

Native features beyond the web experience: Face ID app lock (opt-in card on
first launch), push notifications (APNs; a test notification can be triggered
for the demo account on request), camera/microphone for attachments and
voice dictation, native offline handling, and external links opening in
Safari. The in-app content is served from our own domain (ai.shvia.org)
only — general web browsing is not possible.
```

- **Contact:** nome + telefone + e-mail do Samir.

## 8. Preço e disponibilidade

- **Preço: Grátis** (monetização é fora da loja, por contrato — nada de IAP).
- **Países:** decisão do Samir — só Brasil é coerente com o produto hoje;
  todos os países não custa nada e evita clique futuro.

## 9. Screenshots (obrigatórios)

- **iPhone 6.9"** (1320×2868, do simulador iPhone 17 Pro Max): 3–5 imagens.
  Sugestão de sequência: (1) card "Ativar Face ID" (privacidade primeiro),
  (2) chat com resposta e medidor de tokens/custo, (3) seletor de modelo
  com data locality, (4) base de conhecimento/projeto, (5) tela de login.
- **iPad 13"** (2064×2752): OBRIGATÓRIO se o app oferecer iPad. O binário
  hoje suporta iPad (orientações ~ipad declaradas). Alternativa para
  simplificar a 1ª submissão: restringir a iPhone (TARGETED_DEVICE_FAMILY=1)
  — decisão do Samir; a M5 fez o layout de tablet de propósito, então manter
  iPad + gerar os screenshots é o caminho coerente.

## 10. Checklist final pré-Submit

- [ ] Build 0.6.1 selecionado na versão
- [ ] Conta `apple-review@` criada e testada (login + chat + KB funcionando)
- [ ] SHVIA-WEB ≥ 2.91.7 em produção (front do push)
- [ ] Smoke on-device verde (Face ID card, permissão, push:test, tap)
- [ ] Screenshots subidos (6.9" + iPad se mantido)
- [ ] Nutrition label preenchida (§5) · Classificação (§6) · Review info (§7)
- [ ] Submit → review costuma responder em 1–3 dias; rejeição não queima nada
