// Comandos existem para o registro de permissões do Tauri; quem os chama é o
// RUST da casca (run_mobile_plugin), nunca a página remota — a postura de
// "nenhum comando nativo exposto à página" (ADR-001) segue valendo.
const COMMANDS: &[&str] = &["watch_token", "request_permission"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .ios_path("ios")
        .build();
}
