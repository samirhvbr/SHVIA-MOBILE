// Casca de bootstrap do ShvIA Mobile.
//
// A janela Tauri abre esta casca local (instantânea, sem rede) com um splash da
// marca e navega para o ShvIA hospedado. Diferente do desktop, aqui a casca
// VERIFICA o servidor antes de navegar: rede de celular falha o tempo todo, e
// navegar às cegas estampa a tela de erro nativa do WebView. Fluxo:
//   1. ping barato no /api/v1/health (no-cors: resposta opaca serve — só
//      queremos saber se o servidor está alcançável; DNS/offline rejeita);
//   2. alcançável → location.replace() (o splash não fica no histórico);
//   3. falhou → estado "offline" com "Tentar novamente" (e "Abrir mesmo assim"
//      como escape, caso o ping falhe por outra razão que não a rede).
//
// Dev: "?hold" congela no estado connecting e "?hold=offline" mostra o estado
// offline — pra estilizar o splash sem ser redirecionado.

// URL do servidor ShvIA (fonte da verdade).
const SHVIA_URL = "https://ia.blue3.com.br";

const rootEl = document.getElementById("bootstrap")!;
const statusEl = document.getElementById("status")!;
const retryEl = document.getElementById("retry");
const forceEl = document.getElementById("force-open");

function setState(state: "connecting" | "offline"): void {
  rootEl.dataset.state = state;
  statusEl.textContent =
    state === "connecting" ? "Conectando ao ShvIA…" : "Sem conexão com o servidor.";
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

  const hold = new URLSearchParams(window.location.search).get("hold");
  if (hold !== null) {
    setState(hold === "offline" ? "offline" : "connecting");
    return;
  }
  void connect();
});

export {};
