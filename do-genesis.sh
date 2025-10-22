#!/usr/bin/env bash
set -Eeuo pipefail

# ==== styling ====
if command -v tput >/dev/null 2>&1 && [[ -t 1 ]]; then
  BOLD="$(tput bold)"; DIM="$(tput dim)"; RESET="$(tput sgr0)"
  GREEN="$(tput setaf 2)"; YELLOW="$(tput setaf 3)"; RED="$(tput setaf 1)"; BLUE="$(tput setaf 4)"
else
  BOLD=""; DIM=""; RESET=""; GREEN=""; YELLOW=""; RED=""; BLUE=""
fi

log()   { printf "%s[do-genesis]%s %s\n" "$DIM" "$RESET" "$*"; }
ok()    { printf "%s[ok]%s %s\n"   "$GREEN" "$RESET" "$*"; }
warn()  { printf "%s[warn]%s %s\n" "$YELLOW" "$RESET" "$*"; }
err()   { printf "%s[err]%s %s\n"  "$RED" "$RESET" "$*"; }

has()   { command -v "$1" >/dev/null 2>&1; }

# ==== defaults ====
if [[ -n "${SUDO_USER:-}" && -d "/home/${SUDO_USER}" ]]; then
  ROOT_DEFAULT="/home/${SUDO_USER}/Apollo"
else
  ROOT_DEFAULT="${HOME}/Apollo"
fi
DEST_DEFAULT="${ROOT_DEFAULT}/ab2bc"
COLOR_MODE="auto"

declare -a FLAVORS=()

usage() {
  cat <<'EOF'
Usage: do-genesis.sh [--root DIR] [--dest DIR] [--flavor F]... [--flavors "F1 F2 ..."] [--color auto|always|never]

Examples:
  ./do-genesis.sh --flavors "AQY ASY AUY AIY"
  ./do-genesis.sh --flavor AQY --flavor ASY --flavor AUY --flavor AIY

Notes:
- Valid flavors: AQY, ASY, AUY, AIY
- The script will look for an existing genesis.blob and stage it under DEST/<FLAVOR>/genesis/.
  If none is found, it will print instructions. (It will not force-generate a blob unless you add it later.)
EOF
}

ROOT="$ROOT_DEFAULT"
DEST="$DEST_DEFAULT"

# ==== parse args ====
while [[ $# -gt 0 ]]; do
  case "$1" in
    --root)   ROOT="$2"; shift 2;;
    --dest)   DEST="$2"; shift 2;;
    --flavor) FLAVORS+=("$2"); shift 2;;
    --flavors)
      read -r -a _split <<< "$2"
      FLAVORS+=("${_split[@]}")
      shift 2
      ;;
    --color)  COLOR_MODE="$2"; shift 2;;
    -h|--help) usage; exit 0;;
    *)
      err "Unknown argument: $1"
      usage
      exit 2
      ;;
  esac
done

case "${COLOR_MODE}" in
  always) : ;;
  never)  BOLD=""; DIM=""; RESET=""; GREEN=""; YELLOW=""; RED=""; BLUE="";;
  auto)   : ;;
  *) warn "Unknown --color '${COLOR_MODE}', using auto.";; 
esac

if [[ ${#FLAVORS[@]} -eq 0 ]]; then
  FLAVORS=(AQY ASY AUY AIY)
fi

log "Root: ${ROOT}"
log "Dest: ${DEST}"
log "Flavors: ${FLAVORS[*]}"
log "Color: ${COLOR_MODE}"

mkdir -p "${DEST}"

declare -A ALLOWED=(
  [AQY]=1
  [ASY]=1
  [AUY]=1
  [AIY]=1
)

ensure_validator_yaml() {
  local FLV="$1"
  local base_rpc base_metrics base_p2p
  case "$FLV" in
    AQY) base_rpc=21000; base_metrics=23000; base_p2p=25000;;
    ASY) base_rpc=21150; base_metrics=23150; base_p2p=25150;;
    AUY) base_rpc=21300; base_metrics=23300; base_p2p=25300;;
    AIY) base_rpc=21450; base_metrics=23450; base_p2p=25450;;
    *)   err "Internal: unknown flavor '$FLV' in ensure_validator_yaml"; return 1;;
  esac

  local cfg_dir="${DEST}/${FLV}/config"
  local cfg_file="${cfg_dir}/validator.yaml"
  mkdir -p "${cfg_dir}"

  if [[ -f "${cfg_file}" ]]; then
    ok "(${FLV}) Found validator.yaml"
    return 0
  fi

  cat > "${cfg_file}" <<EOF
# Auto-generated minimal Sui validator config for ${FLV}
genesis:
  genesis-file-location: "${DEST}/${FLV}/genesis/genesis.blob"

json-rpc-address: "0.0.0.0:${base_rpc}"
metrics-address: "0.0.0.0:${base_metrics}"

db-path: "${DEST}/${FLV}/data/db"

p2p-config:
  # Socket syntax is IP:PORT
  listen-address: "0.0.0.0:${base_p2p}"
  max-known-peers: 50
  upnp: false
EOF
  ok "(${FLV}) Wrote ${cfg_file}"
}

ensure_rustup() {
  if has rustup; then
    ok "Setting Rust toolchain override to 1.89.0"
    rustup override set 1.89.0 >/dev/null 2>&1 || rustup override set 1.89.0
  else
    warn "rustup not found; skipping toolchain override"
  fi
}

# ---- Windows/WSL path helper ----
wsl_to_posix() {
  local p="$1"
  if [[ "$p" == ~* ]]; then
    p="${p/#\~/$HOME}"
  fi
  if [[ "$p" == /* ]]; then
    printf "%s" "$p"; return 0
  fi
  if [[ "$p" == \\\\wsl.localhost\\* || "$p" =~ ^[A-Za-z]:\\ ]]; then
    if command -v wslpath >/dev/null 2>&1; then
      local out
      if out="$(wslpath -u "$p" 2>/dev/null)"; then
        printf "%s" "$out"; return 0
      fi
    fi
    local q="${p#\\\\wsl.localhost\\}"
    q="${q#*\\}"
    q="/${q//\\//}"
    printf "%s" "$q"
    return 0
  fi
  printf "%s" "$p"
}

# ---- AIY-specific fix for stale bytecode snapshots ----
clean_aiy_stale_bytecode() {
  local proj_dir="$1"
  local snapshot_dir="${proj_dir}/crates/aiy-framework-snapshot/bytecode_snapshot"
  local cleaned_anything=false
  
  # Remove main bytecode snapshots (don't backup, just remove)
  if [[ -d "$snapshot_dir" ]]; then
    log "(AIY) Removing stale bytecode snapshots before build"
    rm -rf "$snapshot_dir" || {
      warn "(AIY) Failed to remove snapshot directory; continuing anyway"
    }
    cleaned_anything=true
  fi
  
  # Clear any Move build directories that might contain stale sui references
  local build_dirs
  mapfile -t build_dirs < <(find "${proj_dir}/crates/aiy-framework/packages" -name "build" -type d 2>/dev/null)
  for build_dir in "${build_dirs[@]}"; do
    if [[ -d "$build_dir" ]]; then
      log "(AIY) Removing stale Move build directory: $build_dir"
      rm -rf "$build_dir" || warn "(AIY) Failed to remove build directory: $build_dir"
      cleaned_anything=true
    fi
  done
  
  # Clear target directory fingerprints that might be stale
  local target_snapshots
  mapfile -t target_snapshots < <(find "${proj_dir}/target" -name "*framework-snapshot*" -type d 2>/dev/null)
  for target_dir in "${target_snapshots[@]}"; do
    if [[ -d "$target_dir" ]]; then
      log "(AIY) Removing stale target snapshot: $target_dir"
      rm -rf "$target_dir" || warn "(AIY) Failed to remove target snapshot: $target_dir"
      cleaned_anything=true
    fi
  done
  
  # Also remove any backup snapshots from previous runs
  local backup_dir="${proj_dir}/crates/aiy-framework-snapshot/bytecode_snapshot.backup"
  if [[ -d "$backup_dir" ]]; then
    log "(AIY) Removing backup snapshots from previous runs"
    rm -rf "$backup_dir" || warn "(AIY) Failed to remove backup snapshot directory"
    cleaned_anything=true
  fi
  
  if [[ "$cleaned_anything" == "true" ]]; then
    ok "(AIY) Cleaned stale bytecode to ensure fresh build"
    return 0
  else
    log "(AIY) No stale bytecode found"
    return 1
  fi
}

# ---- Genesis creation from external config ----
genesis_from_config() {
  local FLV="$1"
  local bin="sui"
  [[ "$FLV" == "AIY" ]] && bin="aiy"

  local cfg_unc=""
  local work_unc=""
  case "$FLV" in
    AQY)
      cfg_unc="~/Apollo/ab2bc/genesisAQY.YAML"
      work_unc="~/Apollo/ab2bc/AQY/genesis"
      ;;
    ASY)
      cfg_unc="~/Apollo/ab2bc/genesisASY.YAML"
      work_unc="~/Apollo/ab2bc/ASY/genesis"
      ;;
    AUY)
      cfg_unc="~/Apollo/ab2bc/genesisAUY.YAML"
      work_unc="~/Apollo/ab2bc/AUY/genesis"
      ;;
    AIY)
      cfg_unc="~/Apollo/ab2bc/genesisAIY.YAML"
      work_unc="~/Apollo/ab2bc/AIY/genesis"
      ;;
    *)
      return 0;;
  esac

  if [[ "$cfg_unc" == ~* ]]; then cfg_unc="${cfg_unc/#\~/$HOME}"; fi
  if [[ "$work_unc" == ~* ]]; then work_unc="${work_unc/#\~/$HOME}"; fi
  if [[ -n "${SUDO_USER:-}" ]]; then
    sudo_home="/home/${SUDO_USER}"
    if [[ ! -f "$cfg_unc" && -f "${sudo_home}${cfg_unc#${HOME}}" ]]; then
      cfg_unc="${sudo_home}${cfg_unc#${HOME}}"
    fi
    if [[ ! -d "$work_unc" ]]; then
      work_unc="${sudo_home}${work_unc#${HOME}}"
    fi
  fi

  local cfg_posix work_posix
  if [[ "$cfg_unc" == \\\\wsl.localhost\\* || "$cfg_unc" =~ ^[A-Za-z]:\\ ]]; then
    cfg_posix="$(wsl_to_posix "$cfg_unc")"
  else
    cfg_posix="$cfg_unc"
  fi
  if [[ "$work_unc" == \\\\wsl.localhost\\* || "$work_unc" =~ ^[A-Za-z]:\\ ]]; then
    work_posix="$(wsl_to_posix "$work_unc")"
  else
    work_posix="$work_unc"
  fi

  if [[ -f "$cfg_posix" ]]; then
    mkdir -p "$work_posix"
    local exe="${ROOT}/${FLV}/target/release/${bin}"
    if [[ ! -x "$exe" ]]; then
      warn "(${FLV}) Expected binary not found at ${exe}; attempting to build before genesis."
      ( pushd "${ROOT}/${FLV}" >/dev/null && cargo build --release --bin "${bin}" ) || {
        err "(${FLV}) Build failed; cannot run genesis."
        return 1
      }
    fi
    
    ok "(${FLV}) Creating genesis from config (cfg: $cfg_unc → $cfg_posix, work: $work_unc → $work_posix)"
    local genesis_result=0
    "$exe" genesis -f --from-config "$cfg_posix" --working-dir "$work_posix" || genesis_result=$?
    
    if [[ $genesis_result -ne 0 ]]; then
      err "(${FLV}) Genesis command failed."
      return 1
    fi
    
    ok "(${FLV}) Genesis created in $work_posix"
  else
    warn "(${FLV}) Skipping genesis: config YAML not found at $cfg_unc (POSIX $cfg_posix)"
  fi
}

# ---- Legacy derive_more normalizer (left intact) ----
pin_derive_more_workspace() {
  local ws="$1/Cargo.toml"
  [[ -f "$ws" ]] || return 0
  if ! grep -Eq '^\[workspace\.dependencies\]' "$ws"; then
    printf '\n[workspace.dependencies]\n' >> "$ws"
    ok "($(basename "$1")) Added [workspace.dependencies]"
  fi
  if ! grep -Eq '^\s*derive_more\s*=' "$ws"; then
    printf 'derive_more = { version = "0.99.17", default-features = false, features = ["as_ref","as_mut"] }\n' >> "$ws"
    ok "($(basename "$1")) Pinned derive_more in workspace.dependencies"
  fi
}

ensure_sui_types_dep_derive_more() {
  local ct="$1/crates/sui-types/Cargo.toml"
  [[ -f "$ct" ]] || return 0
  if grep -Eq '^\[dependencies\]' "$ct" && ! grep -Eq '^\s*derive_more\s*=' "$ct"; then
    awk '
      BEGIN{done=0}
      /^\[dependencies\]$/ && !done { print; print "derive_more = { workspace = true }"; done=1; next }
      { print }
    ' "$ct" > "${ct}.tmp" && mv "${ct}.tmp" "$ct"
    ok "($(basename "$1")) sui-types uses derive_more = { workspace = true }"
  fi
}

qualify_derives_in_crypto_rs() {
  local cr="$1"
  [[ -f "$cr" ]] || return 0

  perl -0777 -pe 's/use\s+derive_more::\{\s*derive_more::AsRef\s*,\s*derive_more::AsMut\s*,/use derive_more::{AsRef, AsMut, /g' "$cr" > "${cr}.tmp" && mv "${cr}.tmp" "$cr"
  perl -0777 -pe 's/use\s+derive_more::\{\s*derive_more::AsRef\s*,/use derive_more::{AsRef, /g' "$cr" > "${cr}.tmp" && mv "${cr}.tmp" "$cr"
  perl -0777 -pe 's/use\s+derive_more::\{\s*derive_more::AsMut\s*,/use derive_more::{AsMut, /g' "$cr" > "${cr}.tmp" && mv "${cr}.tmp" "$cr"

  if ! grep -Eq 'use\s+derive_more::\{[^}]*\bAsRef\b' "$cr" && ! grep -Eq '^use\s+derive_more::AsRef;' "$cr"; then
    sed -i '1i use derive_more::AsRef;' "$cr"
  fi
  if ! grep -Eq 'use\s+derive_more::\{[^}]*\bAsMut\b' "$cr" && ! grep -Eq '^use\s+derive_more::AsMut;' "$cr"; then
    sed -i '1i use derive_more::AsMut;' "$cr"
  fi

  perl -0777 -pe 's/\bimpl\s+derive_more::AsRef</impl AsRef</g; s/\bimpl\s+derive_more::AsMut</impl AsMut</g' "$cr" > "${cr}.tmp" && mv "${cr}.tmp" "$cr"

  perl -0777 -pe '
    s/(#\s*\[\s*derive\s*\((?:(?!\)).)*?)\bderive_more::AsRef\b/\1AsRef/sg;
    s/(#\s*\[\s*derive\s*\((?:(?!\)).)*?)\bderive_more::AsMut\b/\1AsMut/sg;
  ' "$cr" > "${cr}.tmp" && mv "${cr}.tmp" "$cr"

  ok "(sui-types) Normalized derive_more AsRef/AsMut usage in crypto.rs"
}

surgical_fixes_for_flavor() {
  local FLV="$1"
  local ws="${ROOT}/${FLV}"
  case "$FLV" in
    AIY)
      local f="${ws}/crates/aiy-types/src/rpc_proto_conversions.rs"
      if [[ -f "$f" ]]; then
        perl -0777 -pe 's/with_bridge_object_version\(\s*bridge_object_version\s*\)/with_bridge_object_version(bridge_object_version.into())/g' "$f" > "${f}.tmp" && mv "${f}.tmp" "$f"
        ok "(AIY) Applied .into() for bridge_object_version"
      fi
      ;;
    *)
      ;;
  esac
}

# --- NEW: Rewrite MystenLabs -> ab2bc for AIY TOML git refs only ---
rewrite_mysten_to_ab2bc_for_aiy() {
  local proj="$1"
  # Only touch TOML files that mention the specific MystenLabs Sui repo
  local needle_https="https://github.com/MystenLabs/sui.git"
  local needle_ssh="git@github.com:MystenLabs/sui.git"
  local repl_https="https://github.com/ab2bc/sui.git"
  local repl_ssh="git@github.com:ab2bc/sui.git"

  mapfile -t files < <(grep -RIl --include='*.toml' --include='*.TOML' -e "$needle_https" -e "$needle_ssh" "$proj" 2>/dev/null || true)
  if [[ ${#files[@]} -gt 0 ]]; then
    for f in "${files[@]}"; do
      sed -i "s#${needle_https}#${repl_https}#g" "$f"
      sed -i "s#${needle_ssh}#${repl_ssh}#g" "$f"
      ok "(AIY) Rewrote MystenLabs→ab2bc in $f"
    done
  fi
}

build_flavor() {
  local FLV="$1"
  local proj=""
  if [[ -f "${ROOT}/${FLV}/Cargo.toml" ]]; then
    proj="${ROOT}/${FLV}"
  elif [[ -f "${ROOT}/AIY/Cargo.toml" ]]; then
    proj="${ROOT}/AIY"
  else
    proj="$(pwd)"
  fi

  pushd "${proj}" >/dev/null || { warn "Could not enter project dir '${proj}'"; return 1; }

  if [[ "$FLV" == "AIY" ]]; then
    # Clean stale bytecode BEFORE build to prevent interference
    clean_aiy_stale_bytecode "${proj}"
    # Ensure fresh dependency graph for AIY only
    rm -f "${proj}/Move.lock" 2>/dev/null || true
    ensure_rustup
    surgical_fixes_for_flavor "${FLV}"
    # Rewrite MystenLabs repo references to ab2bc for AIY only
    rewrite_mysten_to_ab2bc_for_aiy "${proj}"
    ok "(${FLV}) Building binary 'aiy' in ${proj}"
    if ! cargo build --release --bin "aiy"; then
      err "(${FLV}) cargo build failed for --bin aiy"
      popd >/dev/null || true
      return 1
    fi
  else
    ok "(${FLV}) Building binary 'sui' in ${proj}"
    if ! cargo build --release --bin "sui"; then
      err "(${FLV}) cargo build failed for --bin sui"
      popd >/dev/null || true
      return 1
    fi
  fi

  ok "(${FLV}) Build complete"
  popd >/dev/null || true
  return 0
}

process_flavor() {
  local FLV="$1"
  if [[ -z "${ALLOWED[$FLV]:-}" ]]; then
    warn "Skipping unknown flavor: '${FLV}'"
    return 0
  fi

  log "---- Processing ${BOLD}${FLV}${RESET} ----"
  ensure_validator_yaml "${FLV}"
  build_flavor "${FLV}"
  genesis_from_config "${FLV}" || true
}

for flv in "${FLAVORS[@]}"; do
  process_flavor "${flv}"
done

log "Done."
