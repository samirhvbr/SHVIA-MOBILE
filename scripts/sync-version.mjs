#!/usr/bin/env node
// Propagates the version from version.md (single source of truth) to every file that
// carries one. Idempotent (writes only what changes), dependency-free (plain Node).
// Runs on `prebuild` and by hand via `npm run version:sync`; `--verificar` checks without
// writing. Modelled on the SHVTERM script.
//
// ## The store carriers were outside this script (finding F-24, September 2026 review)
//
// Measured on 02/09 with the repository at 0.6.16:
//
//   src-tauri/gen/apple/project.yml        CFBundleShortVersionString: 0.6.8
//   src-tauri/gen/apple/*/Info.plist       0.6.8
//   src-tauri/gen/android/.../tauri.properties  versionName=0.2.2, versionCode=2002
//
// Eight releases behind on iOS, and Android on a *different numbering scheme* altogether.
// `tauri ios build` regenerates part of this, but a clean clone and an Xcode session opened
// by hand use what is in git — two sources of truth for the number the store sees.
//
// ## versionCode is the part that can actually break a release
//
// Google Play refuses a build whose `versionCode` is not greater than the last one
// published. Deriving it from semver is easy; deriving it *safely* is the whole point, so
// this script REFUSES to lower it rather than writing a number that gets rejected at
// submission — the one moment when discovering the problem is most expensive.
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = resolve(dirname(fileURLToPath(import.meta.url)), ".."); // raiz do repo
const read = (p) => readFileSync(resolve(ROOT, p), "utf8");

const CHECK_ONLY = process.argv.includes("--verificar");
const version = read("version.md").trim();
if (!/^\d+\.\d+\.\d+$/.test(version)) {
  console.error(
    `[sync-version] versão inválida em version.md: ${JSON.stringify(version)}`,
  );
  process.exit(1);
}

// [arquivo, regex que captura o trecho a substituir, substituição]
const targets = [
  ["package.json", /("version"\s*:\s*")\d+\.\d+\.\d+(")/, `$1${version}$2`],
  [
    "src-tauri/tauri.conf.json",
    /("version"\s*:\s*")\d+\.\d+\.\d+(")/,
    `$1${version}$2`,
  ],
  [
    "src-tauri/Cargo.toml",
    /(^\s*version\s*=\s*")\d+\.\d+\.\d+(")/m,
    `$1${version}$2`,
  ],
  // Lock files (existem após install/build): ancorar no nome do nosso pacote
  // para não tocar nas versões das dependências.
  [
    "src-tauri/Cargo.lock",
    /(name = "shvia-mobile"\r?\nversion = ")\d+\.\d+\.\d+(")/,
    `$1${version}$2`,
  ],
  [
    "package-lock.json",
    /("name":\s*"shvia-mobile",\s*"version":\s*")\d+\.\d+\.\d+(")/g,
    `$1${version}$2`,
  ],

  // ── What the stores read (F-24) ────────────────────────────────────────
  [
    "src-tauri/gen/apple/project.yml",
    /(CFBundleShortVersionString:\s*)\d+\.\d+\.\d+/,
    `$1${version}`,
  ],
  [
    "src-tauri/gen/apple/project.yml",
    /(CFBundleVersion:\s*")\d+\.\d+\.\d+(")/,
    `$1${version}$2`,
  ],
  [
    "src-tauri/gen/apple/shvia-mobile_iOS/Info.plist",
    /(<key>CFBundleShortVersionString<\/key>\s*\n\s*<string>)\d+\.\d+\.\d+(<\/string>)/,
    `$1${version}$2`,
  ],
  [
    "src-tauri/gen/apple/shvia-mobile_iOS/Info.plist",
    /(<key>CFBundleVersion<\/key>\s*\n\s*<string>)\d+\.\d+\.\d+(<\/string>)/,
    `$1${version}$2`,
  ],
  [
    "src-tauri/gen/android/app/tauri.properties",
    /(tauri\.android\.versionName=)\d+\.\d+\.\d+/,
    `$1${version}`,
  ],
];

/**
 * Android `versionCode`, derived from semver and never allowed to go backwards.
 *
 * `major*1_000_000 + minor*10_000 + patch*100` leaves room for 99 minors, 99 patches and a
 * spare hundred per patch for re-submissions of the same version — which is the case that
 * makes people hand-edit the file and lose sync in the first place.
 */
const androidVersionCode = (v) => {
  const [maj, min, pat] = v.split(".").map(Number);
  return maj * 1_000_000 + min * 10_000 + pat * 100;
};

let changed = 0;
let drift = 0;
for (const [file, re, repl] of targets) {
  let text;
  try {
    text = read(file);
  } catch {
    console.warn(`[sync-version] pulei (ausente): ${file}`);
    continue;
  }
  if (!re.test(text)) {
    console.warn(`[sync-version] padrão de versão não encontrado em ${file}`);
    continue;
  }
  const next = text.replace(re, repl);
  if (next !== text) {
    if (CHECK_ONLY) {
      console.error(`[sync-version] ✗ ${file} não está em ${version}`);
      drift++;
      continue;
    }
    writeFileSync(resolve(ROOT, file), next);
    console.log(`[sync-version] ${file} → ${version}`);
    changed++;
  }
}

// ── versionCode: derive, and refuse to go backwards ──────────────────────
const propsPath = "src-tauri/gen/android/app/tauri.properties";
try {
  const props = read(propsPath);
  const atual = Number(/tauri\.android\.versionCode=(\d+)/.exec(props)?.[1] ?? 0);
  const alvo = androidVersionCode(version);
  if (alvo < atual) {
    // Writing this would produce a build the Play Console rejects — and it would be
    // discovered at submission, which is the most expensive moment to find out.
    console.error(
      `[sync-version] 🔴 versionCode iria DIMINUIR: ${atual} → ${alvo}.\n`
        + `               O Google Play recusa build com código menor que o último publicado.\n`
        + `               Suba a versão em version.md, ou ajuste o esquema em androidVersionCode().`,
    );
    process.exit(1);
  }
  if (alvo !== atual) {
    if (CHECK_ONLY) {
      console.error(`[sync-version] ✗ versionCode está ${atual}, deveria ser ${alvo}`);
      drift++;
    } else {
      writeFileSync(
        resolve(ROOT, propsPath),
        props.replace(/(tauri\.android\.versionCode=)\d+/, `$1${alvo}`),
      );
      console.log(`[sync-version] ${propsPath} versionCode → ${alvo}`);
      changed++;
    }
  }
} catch {
  console.warn(`[sync-version] pulei (ausente): ${propsPath}`);
}

if (CHECK_ONLY) {
  if (drift) {
    console.error(`\n[sync-version] 🔴 ${drift} portador(es) fora de ${version}. Rode \`npm run version:sync\`.\n`);
    process.exit(1);
  }
  console.log(`[sync-version] ok: todos os portadores em ${version}.`);
} else {
  console.log(`[sync-version] ${version} (${changed} arquivo(s) atualizado(s))`);
}
