#!/usr/bin/env bash
#
# Build one app, start it, run the k6 benchmark against it, then stop it.
#
# Benchmarks a single target at a time (see k6/load_testing.js). The app runs in
# the background and is ALWAYS stopped on exit (trap), even if k6 fails or you
# press Ctrl+C.
#
# Postgres must be running first:  docker compose up -d database
# For a fair comparison, re-seed the DB to the same state before each target.
#
# Usage:
#   ./run-bench.sh springboot
#   ./run-bench.sh rust -u 100 -d 3m
#
#   -u  virtual users   (default 50)
#   -d  measured phase  (default 2m)
#   -w  warm-up phase   (default 30s)

set -uo pipefail

# Anchor to the repo root (parent of this script's k6/ dir) so it works from anywhere.
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

usage() { echo "usage: $0 <springboot-mvc|springboot-mvc-native|springboot-webflux|springboot-webflux-native|rust> [-u vus] [-d duration] [-w warmup]" >&2; }
say()   { printf '\033[36m==> %s\033[0m\n' "$1"; }

# --- arguments -------------------------------------------------------------
TARGET="${1:-}"; shift || true
VUS=50; DURATION=2m; WARMUP=30s

while getopts ":u:d:w:h" opt; do
    case "$opt" in
        u) VUS="$OPTARG" ;;
        d) DURATION="$OPTARG" ;;
        w) WARMUP="$OPTARG" ;;
        h) usage; exit 0 ;;
        :) echo "Option -$OPTARG requires an argument." >&2; exit 1 ;;
        \?) echo "Unknown option -$OPTARG." >&2; usage; exit 1 ;;
    esac
done

if [[ "$TARGET" != "springboot-mvc" &&
      "$TARGET" != "springboot-mvc-native" &&
      "$TARGET" != "springboot-webflux" &&
      "$TARGET" != "springboot-webflux-native" &&
      "$TARGET" != "rust" ]]; then
    usage; exit 1
fi

# --- build + start ---------------------------------------------------------
case "$TARGET" in
    springboot-mvc)            PORT=8081 ;;
    springboot-mvc-native)     PORT=8082 ;;
    springboot-webflux)        PORT=8083 ;;
    springboot-webflux-native) PORT=8084 ;;
    rust)                      PORT=1337 ;;
esac

# k6's handleSummary writes its file relative to the working dir, so collect them
# under k6/summaries/ by running from there.
SUMMARY_DIR="$root/k6/summaries"
mkdir -p "$SUMMARY_DIR"
cd "$SUMMARY_DIR"

DATETIME=$(date +%Y-%m-%dT%H:%M:%S)

say "Running k6 against http://localhost:$PORT ..."

K6_PROMETHEUS_RW_TREND_STATS="avg,min,max,p(90),p(95),p(99)" K6_PROMETHEUS_RW_SERVER_URL=http://localhost:9090/api/v1/write k6 run \
    --tag target="$TARGET" \
    --tag testid="$TARGET-$VUS-$DATETIME" \
    -o experimental-prometheus-rw \
    -e URL="http://localhost:$PORT" \
    -e LABEL="$TARGET" \
    -e VUS="$VUS" \
    -e DURATION="$DURATION" \
    -e WARMUP="$WARMUP" \
    "$root/k6/load_testing.js"

say "Done. Summary written to k6/summaries/summary-$VUS-$TARGET.json"
