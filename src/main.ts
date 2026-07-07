// Casca de bootstrap do ShvIA Mobile.
//
// A janela Tauri abre esta casca local (instantânea, sem rede) com um splash da
// marca e navega para o ShvIA hospedado. Diferente do desktop, aqui a casca
// VERIFICA o servidor antes de navegar: rede de celular falha o tempo todo, e
// navegar às cegas estampa a tela de erro nativa do WebView. Fluxo:
//   1. ping barato no /api/v1/health (no-cors: resposta opaca serve — só
//      queremos saber se o servidor está alcançável; DNS/offline rejeita);
//   2. alcançável → location.replace() (o splash não fica no histórico);
//   3. falhou → estado "offline" com "Tentar agora" (e "Abrir mesmo assim"
//      como escape, caso o ping falhe por outra razão que não a rede).
//
// Offline com AUTO-RETRY: além do botão, a casca fica tentando sozinha a cada
// 5 s e no evento `online` do navegador — voltou a rede, entra sem clique
// (pedido do Samir, 07/07: "ficou ali para eu apertar o tentar novamente").
//
// Dev: "?hold" congela no estado connecting e "?hold=offline" mostra o estado
// offline — pra estilizar o splash sem ser redirecionado (sem auto-retry).

// URL do servidor ShvIA (fonte da verdade).
const SHVIA_URL = "https://ia.blue3.com.br";
// Intervalo do auto-retry no estado offline.
const AUTO_RETRY_MS = 5_000;

const rootEl = document.getElementById("bootstrap")!;
const statusEl = document.getElementById("status")!;
const retryEl = document.getElementById("retry");
const forceEl = document.getElementById("force-open");

// "?hold" = modo de estilização: nenhum timer/navegação automática.
const holdMode = new URLSearchParams(window.location.search).has("hold");

let autoRetryTimer: number | undefined;
let checking = false;

function scheduleAutoRetry(): void {
  window.clearTimeout(autoRetryTimer);
  if (holdMode) return;
  autoRetryTimer = window.setTimeout(() => {
    void autoCheck();
  }, AUTO_RETRY_MS);
}

function setState(state: "connecting" | "offline"): void {
  rootEl.dataset.state = state;
  statusEl.textContent =
    state === "connecting"
      ? "Conectando ao ShvIA…"
      : "Sem conexão com o servidor — reconectando…";
  window.clearTimeout(autoRetryTimer);
  if (state === "offline") {
    scheduleAutoRetry();
  }
}

async function serverReachable(timeoutMs = 6000): Promise<boolean> {
  const ctl = new AbortController();
  const timer = setTimeout(() => ctl.abort(), timeoutMs);
  try {
    await fetch(`${SHVIA_URL}/api/v1/health`, {
      mode: "no-cors",
      cache: "no-store",
      signal: ctl.signal,
    });
    return true;
  } catch {
    return false;
  } finally {
    clearTimeout(timer);
  }
}

// Tentativa silenciosa (auto-retry): não mexe na UI enquanto verifica — só
// navega quando o servidor voltar. Sem sobreposição: a próxima só é agendada
// quando esta termina (e só se ainda estivermos offline).
async function autoCheck(): Promise<void> {
  if (checking) return;
  checking = true;
  const ok = await serverReachable();
  checking = false;
  if (ok) {
    window.location.replace(SHVIA_URL);
    return;
  }
  if (rootEl.dataset.state === "offline") {
    scheduleAutoRetry();
  }
}

// Tentativa manual/inicial: mostra o estado "connecting" (sonar) enquanto tenta.
async function connect(): Promise<void> {
  setState("connecting");
  if (await serverReachable()) {
    window.location.replace(SHVIA_URL);
    return;
  }
  setState("offline");
}

window.addEventListener("DOMContentLoaded", () => {
  retryEl?.addEventListener("click", () => {
    void connect();
  });
  forceEl?.addEventListener("click", () => {
    window.location.replace(SHVIA_URL);
  });
  // Rede voltou (evento do SO/navegador) → tenta na hora, sem esperar os 5 s.
  window.addEventListener("online", () => {
    if (!holdMode && rootEl.dataset.state === "offline") {
      window.clearTimeout(autoRetryTimer);
      void autoCheck();
    }
  });

  if (holdMode) {
    const hold = new URLSearchParams(window.location.search).get("hold");
    setState(hold === "offline" ? "offline" : "connecting");
    return;
  }
  void connect();
});

export {};
