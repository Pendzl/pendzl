#!/bin/bash
SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

set -e

start_time=$(date +%s.%3N)
script -efq $SCRIPT_DIR/build.log -c \
"env CARGO_TERM_COLOR=always FORCE_COLOR=1 npx tsx $SCRIPT_DIR/scripts/compile/compileAllContracts.ts $* \
    2>&1 | sed -n -E '/profiles for the non root.*|package:.*|workspace:.*/!p'; exit \"\${PIPESTATUS[0]}\""

pnpm generateTypes
end_time=$(date +%s.%3N)
elapsed=$(echo "scale=3; $end_time - $start_time" | bc)

printf 'Build took %02dm:%02fs\n' $(echo -e "$elapsed%3600/60\n$elapsed%60"| bc)
pnpm ansiToHtml $SCRIPT_DIR/build.log  $SCRIPT_DIR/build.log.html