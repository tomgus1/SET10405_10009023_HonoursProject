#!/usr/bin/env bash
#
# full-bench.sh — Rust (raw GPUI) vs Rust (your framework) vs Java vs Flutter
# vs Electron: desktop app comparison for the Honours Project.
#
# WHAT EACH METRIC MEANS
# -----------------------------------------------------------------------------
# Lines of code       Counted by `cloc`, scoped to each project's src/ dir only
#                      (excludes build output, vendored deps, node_modules).
# Dependency count     Declared+transitive packages per lockfile. Counting
#                      method differs by ecosystem (see notes column) — do
#                      NOT compare raw numbers across languages directly.
# Clean build          Full rebuild from scratch (caches/artifacts removed
#                      first). Reflects worst-case build time, e.g. CI.
# Second build         Same command run again with no source changes. For
#                      Rust/Java/Flutter this is a true incremental build.
#                      electron-builder does not meaningfully incrementalise
#                      packaging, so its figure is a full re-package, not a
#                      cache hit.
# Artifact size        Size of the final deployable output. Java's raw .jar
#                      needs a pre-installed JVM to run; the jlink figure
#                      bundles a minimal custom JVM alongside it, giving a
#                      fairer "fully self-contained" comparison against the
#                      other three stacks.
# Startup time         Wall-clock time from process launch to first rendered
#                      frame, via `hyperfine` (10 timed runs, 1 warmup).
#                      Requires each app to accept a debug flag
#                      (--headless-selftest-exit) that renders one frame and
#                      exits immediately — without it, GUI apps run forever
#                      and there's nothing to time.
# RSS (idle memory)    Resident Set Size: the actual physical RAM the process
#                      is using right now, in MB. NOT virtual/reserved
#                      memory, which would overstate real usage. Sampled
#                      after the app has sat idle (no user interaction) for
#                      IDLE_SAMPLE_AFTER seconds, so this reflects steady-
#                      state baseline cost, not a busy/active-use figure.
# CPU% (idle)          A single instantaneous reading from `ps`, not an
#                      average over time. Treat as indicative, not precise.
# GPU busy%            System-wide GPU utilisation while the app sits idle
#                      (amdgpu sysfs has no per-process breakdown; NVIDIA's
#                      nvidia-smi pmon does, when available). Near-zero for
#                      all apps is expected and legitimate for a static UI —
#                      it is NOT a sign of failed measurement.
# -----------------------------------------------------------------------------
#
# Requirements (Fedora): sudo dnf install cloc hyperfine
# Optional: cargo install cargo-bloat ; jq (for accurate npm dep counts)
#
# Every path below can be overridden via environment variables, e.g.:
#   PROJECT_LABEL=notes RUST_PROJECT_DIR=~/.../rustnotes RUST_BIN_NAME=notes-app \
#   FLUTTER_PROJECT_DIR=~/.../flutternotes \
#   ELECTRON_PROJECT_DIR=~/.../electronnotes \
#   ./full-bench.sh
#
# EDIT THESE PATHS BEFORE RUNNING (or override via env vars):
RUST_PROJECT_DIR="${RUST_PROJECT_DIR:-$HOME/workspace/personal/university/honours/notes/rustnotes}"
RUST_BIN_NAME="${RUST_BIN_NAME:-notes-app}"
RUST_BIN_PATH="${RUST_BIN_PATH:-$RUST_PROJECT_DIR/target/release/$RUST_BIN_NAME}"
RUST_SRC_DIR="${RUST_SRC_DIR:-$RUST_PROJECT_DIR/src}"

# Your own framework, built on top of GPUI (as distinct from the raw-GPUI app above)
RUSTFW_PROJECT_DIR="${RUSTFW_PROJECT_DIR:-$HOME/workspace/personal/university/honours/notes/rustfwnotes}"
RUSTFW_BIN_NAME="${RUSTFW_BIN_NAME:-notes-app}"
RUSTFW_BIN_PATH="${RUSTFW_BIN_PATH:-$RUSTFW_PROJECT_DIR/target/release/$RUSTFW_BIN_NAME}"
RUSTFW_SRC_DIR="${RUSTFW_SRC_DIR:-$RUSTFW_PROJECT_DIR/src}"

JAVA_PROJECT_DIR="${JAVA_PROJECT_DIR:-$HOME/workspace/personal/university/honours/notes/javanotes}"
JAVA_JAR_PATH="${JAVA_JAR_PATH:-$HOME/workspace/personal/university/honours/notes/javanotes/target/notes-app-1.0.0-SNAPSHOT.jar}"
JAVA_MAIN_CLASS="${JAVA_MAIN_CLASS:-}"
JAVA_SRC_DIR="${JAVA_SRC_DIR:-$JAVA_PROJECT_DIR/src}"

FLUTTER_PROJECT_DIR="${FLUTTER_PROJECT_DIR:-$HOME/workspace/personal/university/honours/notes/flutternotes}"
FLUTTER_SRC_DIR="${FLUTTER_SRC_DIR:-$FLUTTER_PROJECT_DIR/lib}"
# Full path to the flutter command. Defaults to relying on PATH, but a script
# invoked directly (./full-bench.sh) won't source your .zshrc/.bashrc, so if
# flutter lives somewhere like ~/development/flutter/bin, set this explicitly:
#   FLUTTER_CMD=~/development/flutter/bin/flutter ./full-bench.sh
FLUTTER_CMD="${FLUTTER_CMD:-$HOME/development/flutter/bin/flutter}"
# Binary name defaults to the pubspec.yaml 'name' field if not given.
FLUTTER_BIN_NAME="${FLUTTER_BIN_NAME:-}"
FLUTTER_BUNDLE_DIR="${FLUTTER_BUNDLE_DIR:-$FLUTTER_PROJECT_DIR/build/linux/x64/release/bundle}"

ELECTRON_PROJECT_DIR="${ELECTRON_PROJECT_DIR:-$HOME/workspace/personal/university/honours/notes/reactnotes}"
ELECTRON_SRC_DIR="${ELECTRON_SRC_DIR:-$ELECTRON_PROJECT_DIR/src}"
ELECTRON_PACKAGE_CMD="${ELECTRON_PACKAGE_CMD:-npm run desktop:package}"
ELECTRON_RELEASE_DIR="${ELECTRON_RELEASE_DIR:-$ELECTRON_PROJECT_DIR/release}"

RUNS=10
IDLE_SAMPLE_AFTER=3
IDLE_SAMPLE_DURATION=5
PROJECT_LABEL="${PROJECT_LABEL:-notes}"

OUT="$(pwd)/results-${PROJECT_LABEL}-$(date +%Y%m%d-%H%M%S).md"

# ---------------------------------------------------------------------------
have() { command -v "$1" >/dev/null 2>&1; }
section() { echo -e "\n## $1\n" | tee -a "$OUT"; }
note() { echo "_${1}_" | tee -a "$OUT"; echo "" | tee -a "$OUT"; }
row() { echo "| $1 | $2 | $3 |" | tee -a "$OUT"; }

SCRIPT_VERSION="2026-08-10-annotated-methodology"

echo "# Java vs Rust vs Flutter vs Electron Benchmark — $(date)" > "$OUT"
echo "Script version: $SCRIPT_VERSION" >> "$OUT"
echo "" >> "$OUT"

# Resolve Flutter binary name from pubspec.yaml if not explicitly set
if [ -z "$FLUTTER_BIN_NAME" ] && [ -f "$FLUTTER_PROJECT_DIR/pubspec.yaml" ]; then
  FLUTTER_BIN_NAME=$(grep '^name:' "$FLUTTER_PROJECT_DIR/pubspec.yaml" | head -1 | awk '{print $2}' | tr -d '\r')
fi
FLUTTER_BIN_PATH="$FLUTTER_BUNDLE_DIR/$FLUTTER_BIN_NAME"

# Locate the built AppImage (electron-builder names it after the app + version)
find_electron_appimage() {
  find "$ELECTRON_RELEASE_DIR" -maxdepth 1 -iname "*.AppImage" 2>/dev/null | head -1
}

# ---------------------------------------------------------------------------
section "1. Lines of Code"
note "Counted by cloc, scoped to each project's src/ directory only (build output and vendored deps excluded). Not a quality or complexity metric — just raw size."
if have cloc; then
  echo "### Rust (raw GPUI, scoped to $RUST_SRC_DIR)" | tee -a "$OUT"
  cloc "$RUST_SRC_DIR" --include-lang=Rust --md | tee -a "$OUT"

  echo "### Rust (your framework, scoped to $RUSTFW_SRC_DIR)" | tee -a "$OUT"
  cloc "$RUSTFW_SRC_DIR" --include-lang=Rust --md | tee -a "$OUT"

  echo "### Java (scoped to $JAVA_SRC_DIR)" | tee -a "$OUT"
  cloc "$JAVA_SRC_DIR" --include-lang=Java --md | tee -a "$OUT"

  echo "### Flutter/Dart (scoped to $FLUTTER_SRC_DIR)" | tee -a "$OUT"
  cloc "$FLUTTER_SRC_DIR" --include-lang=Dart --md | tee -a "$OUT"

  echo "### Electron/TypeScript (scoped to $ELECTRON_SRC_DIR, excluding node_modules/dist/release)" | tee -a "$OUT"
  cloc "$ELECTRON_SRC_DIR" --include-lang="TypeScript,JavaScript,JSX,TSX" \
    --exclude-dir=node_modules,dist,release,build,out --md | tee -a "$OUT"
else
  echo "cloc not installed (sudo dnf install cloc) — skipping LOC section" | tee -a "$OUT"
fi

# ---------------------------------------------------------------------------
section "2. Dependency Count"
note "Declared+transitive packages per lockfile. Counting method differs by ecosystem (see notes column) — treat as within-language trend data, not a cross-language comparison."

if [ -f "$JAVA_PROJECT_DIR/pom.xml" ] && have mvn; then
  jdeps_count=$(cd "$JAVA_PROJECT_DIR" && mvn -q dependency:tree -Dscope=compile 2>/dev/null | grep -c ":.*:.*:.*")
else
  jdeps_count="n/a"
fi

if have cargo; then
  rdeps_edges=$(cd "$RUST_PROJECT_DIR" && cargo tree 2>/dev/null | wc -l)
  newrdeps_edges=$(cd "$RUSTFW_PROJECT_DIR" && cargo tree 2>/dev/null | wc -l)
else
  rdeps_edges="n/a (cargo not found)"
  newrdeps_edges="n/a (cargo not found)"
fi

if [ -f "$FLUTTER_PROJECT_DIR/pubspec.lock" ]; then
  fdeps=$(grep -c '^    dependency: ' "$FLUTTER_PROJECT_DIR/pubspec.lock" 2>/dev/null)
  [ -z "$fdeps" ] && fdeps="n/a (pubspec.lock format not recognised)"
else
  fdeps="n/a (no pubspec.lock found)"
fi

if [ -f "$ELECTRON_PROJECT_DIR/package-lock.json" ]; then
  if have jq; then
    edeps=$(jq '.packages | length' "$ELECTRON_PROJECT_DIR/package-lock.json" 2>/dev/null)
  else
    edeps=$(grep -c '"resolved":' "$ELECTRON_PROJECT_DIR/package-lock.json" 2>/dev/null)
    edeps="${edeps} (approximate — install jq for an exact count)"
  fi
else
  edeps="n/a (no package-lock.json found)"
fi

row "Language" "Dependency count" "Notes"
row "---" "---" "---"
row "Rust (raw GPUI)" "$rdeps_edges" "cargo tree edges (not unique crates — see conversation notes)"
row "Rust (your framework)" "$newrdeps_edges" "cargo tree edges"
row "Java" "$jdeps_count" "direct+transitive jars"
row "Flutter/Dart" "$fdeps" "packages in pubspec.lock"
row "Electron/npm" "$edeps" "packages in package-lock.json"

# ---------------------------------------------------------------------------
section "3. Build Times"
note "Clean build = full rebuild from scratch (caches removed first). Second build = same command re-run with no source changes. True incremental build for Rust/Java/Flutter; electron-builder does not meaningfully incrementalise packaging, so its figure is a full re-package."

echo "### Rust (raw GPUI, cargo)" | tee -a "$OUT"
if have cargo; then
  cd "$RUST_PROJECT_DIR"
  cargo clean
  t0=$(date +%s.%N); cargo build --release > /dev/null 2>&1; t1=$(date +%s.%N)
  echo "Clean build: $(echo "$t1 - $t0" | bc)s" | tee -a "$OUT"
  t0=$(date +%s.%N); cargo build --release > /dev/null 2>&1; t1=$(date +%s.%N)
  echo "Incremental (no changes): $(echo "$t1 - $t0" | bc)s" | tee -a "$OUT"
fi

echo "### Rust (your framework, cargo)" | tee -a "$OUT"
if have cargo; then
  cd "$RUSTFW_PROJECT_DIR"
  cargo clean
  t0=$(date +%s.%N); cargo build --release > /dev/null 2>&1; t1=$(date +%s.%N)
  echo "Clean build: $(echo "$t1 - $t0" | bc)s" | tee -a "$OUT"
  t0=$(date +%s.%N); cargo build --release > /dev/null 2>&1; t1=$(date +%s.%N)
  echo "Incremental (no changes): $(echo "$t1 - $t0" | bc)s" | tee -a "$OUT"
fi

echo "### Java (Maven)" | tee -a "$OUT"
if [ -f "$JAVA_PROJECT_DIR/pom.xml" ]; then
  cd "$JAVA_PROJECT_DIR"
  t0=$(date +%s.%N); mvn -q clean package -DskipTests > /dev/null 2>&1; t1=$(date +%s.%N)
  echo "Clean build: $(echo "$t1 - $t0" | bc)s" | tee -a "$OUT"
  t0=$(date +%s.%N); mvn -q package -DskipTests > /dev/null 2>&1; t1=$(date +%s.%N)
  echo "Incremental (no changes): $(echo "$t1 - $t0" | bc)s" | tee -a "$OUT"
fi

echo "### Flutter" | tee -a "$OUT"
if have "$FLUTTER_CMD"; then
  cd "$FLUTTER_PROJECT_DIR"
  "$FLUTTER_CMD" clean > /dev/null 2>&1
  t0=$(date +%s.%N); "$FLUTTER_CMD" build linux --release > /dev/null 2>&1; t1=$(date +%s.%N)
  echo "Clean build: $(echo "$t1 - $t0" | bc)s" | tee -a "$OUT"
  t0=$(date +%s.%N); "$FLUTTER_CMD" build linux --release > /dev/null 2>&1; t1=$(date +%s.%N)
  echo "Incremental (no changes): $(echo "$t1 - $t0" | bc)s" | tee -a "$OUT"
else
  echo "flutter not found (checked FLUTTER_CMD='$FLUTTER_CMD') — skipping. Set FLUTTER_CMD to the full binary path if it's not on this shell's PATH." | tee -a "$OUT"
fi

echo "### Electron (electron-builder, packaged via '$ELECTRON_PACKAGE_CMD')" | tee -a "$OUT"
if [ -d "$ELECTRON_PROJECT_DIR" ]; then
  cd "$ELECTRON_PROJECT_DIR"
  rm -rf "$ELECTRON_RELEASE_DIR" dist
  ELECTRON_BUILD_LOG=$(mktemp)
  t0=$(date +%s.%N); eval "$ELECTRON_PACKAGE_CMD" > "$ELECTRON_BUILD_LOG" 2>&1; ELECTRON_BUILD_STATUS=$?; t1=$(date +%s.%N)
  echo "Clean build (release/dist removed first): $(echo "$t1 - $t0" | bc)s (exit code $ELECTRON_BUILD_STATUS)" | tee -a "$OUT"
  if [ "$ELECTRON_BUILD_STATUS" -ne 0 ] || [ -z "$(find_electron_appimage)" ]; then
    echo "No .AppImage found in $ELECTRON_RELEASE_DIR after build. Last 40 lines of build output:" | tee -a "$OUT"
    echo '```' | tee -a "$OUT"
    tail -40 "$ELECTRON_BUILD_LOG" | tee -a "$OUT"
    echo '```' | tee -a "$OUT"
    echo "Check electron-builder's 'directories.output' config (package.json 'build' key or electron-builder.yml) — if it's not 'release', set ELECTRON_RELEASE_DIR to match." | tee -a "$OUT"
  fi
  rm -f "$ELECTRON_BUILD_LOG"
  t0=$(date +%s.%N); eval "$ELECTRON_PACKAGE_CMD" > /dev/null 2>&1; t1=$(date +%s.%N)
  echo "Second run (no source changes; electron-builder does not fully incrementalise): $(echo "$t1 - $t0" | bc)s" | tee -a "$OUT"
else
  echo "$ELECTRON_PROJECT_DIR not found — skipping" | tee -a "$OUT"
fi

# ---------------------------------------------------------------------------
section "4. Artifact Size"
note "Size of the final deployable output for each stack. Java's raw .jar needs a pre-installed JVM; the jlink figure bundles a minimal custom JVM alongside it for a fairer self-contained comparison against the other three."

jar_size=$(du -h "$JAVA_JAR_PATH" 2>/dev/null | cut -f1)
rust_size=$(du -h "$RUST_BIN_PATH" 2>/dev/null | cut -f1)
rust_stripped_size="n/a"
if [ -f "$RUST_BIN_PATH" ] && have strip; then
  cp "$RUST_BIN_PATH" /tmp/rust_stripped
  strip /tmp/rust_stripped
  rust_stripped_size=$(du -h /tmp/rust_stripped | cut -f1)
fi

rustfw_size=$(du -h "$RUSTFW_BIN_PATH" 2>/dev/null | cut -f1)
rustfw_stripped_size="n/a"
if [ -f "$RUSTFW_BIN_PATH" ] && have strip; then
  cp "$RUSTFW_BIN_PATH" /tmp/rustfw_stripped
  strip /tmp/rustfw_stripped
  rustfw_stripped_size=$(du -h /tmp/rustfw_stripped | cut -f1)
fi
flutter_bundle_size=$(du -sh "$FLUTTER_BUNDLE_DIR" 2>/dev/null | cut -f1)

ELECTRON_APPIMAGE=$(find_electron_appimage)
if [ -n "$ELECTRON_APPIMAGE" ]; then
  electron_size=$(du -h "$ELECTRON_APPIMAGE" | cut -f1)
else
  electron_size="n/a (no .AppImage found in $ELECTRON_RELEASE_DIR)"
fi

row "Artifact" "Size" "Notes"
row "---" "---" "---"
row "Rust binary (raw GPUI, unstripped)" "$rust_size" "self-contained, statically linked"
row "Rust binary (raw GPUI, stripped)" "$rust_stripped_size" "debug symbols removed"
row "Rust binary (your framework, unstripped)" "$rustfw_size" "self-contained, statically linked"
row "Rust binary (your framework, stripped)" "$rustfw_stripped_size" "debug symbols removed"
row "Java jar" "$jar_size" "requires separately-installed JVM"
row "Flutter bundle (whole dir)" "$flutter_bundle_size" "self-contained, includes Flutter engine .so"
row "Electron AppImage" "$electron_size" "self-contained, includes bundled Chromium + Node.js runtime"

# fair like-for-like: jlink minimal runtime image
if have cargo-bloat && have cargo; then
  echo "### cargo-bloat breakdown — Rust (raw GPUI), top 15 crates by size" | tee -a "$OUT"
  (cd "$RUST_PROJECT_DIR" && cargo bloat --release --crates -n 15) | tee -a "$OUT"
  echo "### cargo-bloat breakdown — Rust (your framework), top 15 crates by size" | tee -a "$OUT"
  (cd "$RUSTFW_PROJECT_DIR" && cargo bloat --release --crates -n 15) | tee -a "$OUT"
fi

if have jlink && have jdeps; then
  echo "### Java: minimal custom runtime via jlink (fairer size comparison)" | tee -a "$OUT"
  JAVA_HOME_GUESS=$(dirname "$(dirname "$(readlink -f "$(which java)")")")
  if [ ! -d "$JAVA_HOME_GUESS/jmods" ]; then
    echo "WARNING: no jmods/ directory found at $JAVA_HOME_GUESS/jmods — this JDK install likely can't jlink. Skipping." | tee -a "$OUT"
  else
    rm -rf /tmp/java-runtime
    JAVA_MAJOR=$(java -version 2>&1 | head -1 | grep -oP '"\K[0-9]+')
    MODS=$(jdeps -q --multi-release "${JAVA_MAJOR:-25}" --print-module-deps "$JAVA_JAR_PATH" 2>/dev/null)
    if [ -z "$MODS" ]; then
      echo "jdeps auto-detection returned nothing — falling back to java.desktop,java.base" | tee -a "$OUT"
      MODS="java.desktop,java.base"
    fi
    JLINK_ERR=$(jlink --add-modules "$MODS" --strip-debug --no-header-files --no-man-pages \
      --compress=2 --output /tmp/java-runtime 2>&1)
    JLINK_STATUS=$?
    if [ "$JLINK_STATUS" -ne 0 ] || [ ! -d /tmp/java-runtime ]; then
      echo "jlink FAILED (exit code $JLINK_STATUS):" | tee -a "$OUT"
      echo '```' | tee -a "$OUT"
      echo "$JLINK_ERR" | tee -a "$OUT"
      echo '```' | tee -a "$OUT"
    else
      total_size=$(du -ch "$JAVA_JAR_PATH" /tmp/java-runtime 2>/dev/null | tail -1 | cut -f1)
      row "jlink custom runtime + jar" "$total_size" "closest apples-to-apples vs the other self-contained artifacts"
    fi
  fi
fi

# ---------------------------------------------------------------------------
section "5. Startup Time ($RUNS runs)"
note "Wall-clock time from process launch to first rendered frame, via hyperfine (10 timed runs, 1 warmup). All apps must support a --headless-selftest-exit flag that opens the window, renders one frame, and exits immediately — without it these will hang/timeout, since GUI apps otherwise run until the window is closed."

if have hyperfine; then
  echo "### Rust (raw GPUI)" | tee -a "$OUT"
  hyperfine --warmup 1 -r "$RUNS" -N "$RUST_BIN_PATH --headless-selftest-exit" 2>&1 | tee -a "$OUT"

  echo "### Rust (your framework)" | tee -a "$OUT"
  hyperfine --warmup 1 -r "$RUNS" -N "$RUSTFW_BIN_PATH --headless-selftest-exit" 2>&1 | tee -a "$OUT"

  echo "### Java" | tee -a "$OUT"
  hyperfine --warmup 1 -r "$RUNS" -N "java -jar $JAVA_JAR_PATH --headless-selftest-exit" 2>&1 | tee -a "$OUT"

  echo "### Flutter" | tee -a "$OUT"
  if [ -x "$FLUTTER_BIN_PATH" ]; then
    hyperfine --warmup 1 -r "$RUNS" -N "$FLUTTER_BIN_PATH --headless-selftest-exit" 2>&1 | tee -a "$OUT"
  else
    echo "Flutter binary not found/executable at $FLUTTER_BIN_PATH — skipping" | tee -a "$OUT"
  fi

  echo "### Electron" | tee -a "$OUT"
  if [ -n "$ELECTRON_APPIMAGE" ]; then
    chmod +x "$ELECTRON_APPIMAGE" 2>/dev/null
    echo "NOTE: AppImages self-mount via FUSE. If this hangs, try: '$ELECTRON_APPIMAGE' --appimage-extract-and-run --headless-selftest-exit" | tee -a "$OUT"
    hyperfine --warmup 1 -r "$RUNS" -i --show-output -N "'$ELECTRON_APPIMAGE' --headless-selftest-exit" 2>&1 | tee -a "$OUT"
  else
    echo "No AppImage found — skipping" | tee -a "$OUT"
  fi
else
  echo "hyperfine not installed (sudo dnf install hyperfine) — skipping startup timing" | tee -a "$OUT"
fi

# ---------------------------------------------------------------------------
section "6. Idle Memory & CPU (app launched, left running ${IDLE_SAMPLE_AFTER}s, then sampled)"
note "RSS (Resident Set Size) = actual physical RAM the process is using right now, in MB — not virtual/reserved memory, which would overstate real usage. CPU% is a single instantaneous 'ps' reading, not an average. Both are sampled after the app has sat idle (no interaction) for ${IDLE_SAMPLE_AFTER}s, so this reflects steady-state baseline cost, not active-use cost. Needs a real display (X11/Wayland) — run from your desktop session, not headless SSH."

measure_idle() {
  local cmd="$1"
  local label="$2"
  eval "$cmd &"
  local pid=$!
  sleep "$IDLE_SAMPLE_AFTER"
  if ! kill -0 "$pid" 2>/dev/null; then
    echo "$label process exited before sampling — skipping" | tee -a "$OUT"
    return
  fi
  local rss_kb
  rss_kb=$(ps -o rss= -p "$pid" 2>/dev/null | tr -d ' ')
  echo "$label RSS after ${IDLE_SAMPLE_AFTER}s idle: $((rss_kb / 1024)) MB" | tee -a "$OUT"
  sleep "$IDLE_SAMPLE_DURATION"
  if kill -0 "$pid" 2>/dev/null; then
    local cpu_pct
    cpu_pct=$(ps -o %cpu= -p "$pid" 2>/dev/null | tr -d ' ')
    echo "$label CPU% (instantaneous, idle): ${cpu_pct}%" | tee -a "$OUT"
  fi
  # kill the whole process tree — Electron/Flutter spawn helper/renderer processes
  pkill -P "$pid" 2>/dev/null
  kill "$pid" 2>/dev/null
}

measure_idle "$RUST_BIN_PATH" "Rust (raw GPUI)"
measure_idle "$RUSTFW_BIN_PATH" "Rust (your framework)"
measure_idle "java -jar $JAVA_JAR_PATH" "Java"
[ -x "$FLUTTER_BIN_PATH" ] && measure_idle "$FLUTTER_BIN_PATH" "Flutter"
[ -n "$ELECTRON_APPIMAGE" ] && measure_idle "'$ELECTRON_APPIMAGE'" "Electron"

# ---------------------------------------------------------------------------
section "7. GPU Usage (idle, sampled over ${IDLE_SAMPLE_DURATION}s)"
note "GPU utilisation while the app sits idle (no user interaction) — not a rendering-performance or frame-rate measurement. amdgpu sysfs reports system-wide usage only (no per-process breakdown); nvidia-smi pmon gives per-process figures when available. Near-zero for all apps is expected and legitimate for a static UI — it is not a sign of failed measurement."

GPU_BACKEND="none"
if have nvidia-smi; then
  GPU_BACKEND="nvidia"
elif compgen -G "/sys/class/drm/card*/device/gpu_busy_percent" > /dev/null 2>&1; then
  GPU_BACKEND="amdgpu-sysfs"
fi
echo "GPU monitoring backend: $GPU_BACKEND" | tee -a "$OUT"

measure_gpu_nvidia() {
  local cmd="$1"
  local label="$2"
  eval "$cmd &"
  local pid=$!
  sleep "$IDLE_SAMPLE_AFTER"
  if ! kill -0 "$pid" 2>/dev/null; then
    echo "$label process exited before GPU sampling — skipping" | tee -a "$OUT"
    return
  fi
  local pmon_out
  pmon_out=$(timeout "$((IDLE_SAMPLE_DURATION + 1))" nvidia-smi pmon -c "$IDLE_SAMPLE_DURATION" -s um 2>/dev/null | awk -v p="$pid" '$2==p')
  pkill -P "$pid" 2>/dev/null
  kill "$pid" 2>/dev/null
  if [ -z "$pmon_out" ]; then
    echo "$label: no GPU activity attributed to PID $pid" | tee -a "$OUT"
  else
    local avg_sm avg_mem
    avg_sm=$(echo "$pmon_out" | awk '{s+=$4; n++} END{if(n>0) printf "%.1f", s/n; else print "n/a"}')
    avg_mem=$(echo "$pmon_out" | awk '{s+=$5; n++} END{if(n>0) printf "%.1f", s/n; else print "n/a"}')
    echo "$label GPU SM util (avg): ${avg_sm}%  |  GPU mem util (avg): ${avg_mem}%" | tee -a "$OUT"
  fi
}

measure_gpu_amdgpu() {
  local cmd="$1"
  local label="$2"
  local card_path
  card_path=$(ls -d /sys/class/drm/card*/device/gpu_busy_percent 2>/dev/null | head -1)
  eval "$cmd &"
  local pid=$!
  sleep "$IDLE_SAMPLE_AFTER"
  if ! kill -0 "$pid" 2>/dev/null; then
    echo "$label process exited before GPU sampling — skipping" | tee -a "$OUT"
    return
  fi
  local total=0 count=0
  for i in $(seq 1 "$IDLE_SAMPLE_DURATION"); do
    local busy
    busy=$(cat "$card_path" 2>/dev/null || echo 0)
    total=$((total + busy))
    count=$((count + 1))
    sleep 1
  done
  pkill -P "$pid" 2>/dev/null
  kill "$pid" 2>/dev/null
  echo "$label: system-wide GPU busy (avg, NOT per-process): $((total / count))%" | tee -a "$OUT"
}

case "$GPU_BACKEND" in
  nvidia)
    measure_gpu_nvidia "$RUST_BIN_PATH" "Rust (raw GPUI)"
    measure_gpu_nvidia "$RUSTFW_BIN_PATH" "Rust (your framework)"
    measure_gpu_nvidia "java -jar $JAVA_JAR_PATH" "Java"
    [ -x "$FLUTTER_BIN_PATH" ] && measure_gpu_nvidia "$FLUTTER_BIN_PATH" "Flutter"
    [ -n "$ELECTRON_APPIMAGE" ] && measure_gpu_nvidia "'$ELECTRON_APPIMAGE'" "Electron"
    ;;
  amdgpu-sysfs)
    echo "(close other GPU-using apps for a cleaner system-wide reading)" | tee -a "$OUT"
    measure_gpu_amdgpu "$RUST_BIN_PATH" "Rust (raw GPUI)"
    measure_gpu_amdgpu "$RUSTFW_BIN_PATH" "Rust (your framework)"
    measure_gpu_amdgpu "java -jar $JAVA_JAR_PATH" "Java"
    [ -x "$FLUTTER_BIN_PATH" ] && measure_gpu_amdgpu "$FLUTTER_BIN_PATH" "Flutter"
    [ -n "$ELECTRON_APPIMAGE" ] && measure_gpu_amdgpu "'$ELECTRON_APPIMAGE'" "Electron"
    ;;
  *)
    echo "No supported GPU monitoring backend found." | tee -a "$OUT"
    ;;
esac

# ---------------------------------------------------------------------------
echo -e "\nDone. Full results written to $OUT\n"
