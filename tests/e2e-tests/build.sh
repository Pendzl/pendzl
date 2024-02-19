#!/bin/bash
SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

set -e

script -efq $SCRIPT_DIR/build.log -c \
"env CARGO_TERM_COLOR=always FORCE_COLOR=1 npx tsx $SCRIPT_DIR/scripts/compile/compileAllContracts.ts $* \
    2>&1 | sed -n -E '/profiles for the non root.*|package:.*|workspace:.*/!p'; exit \"\${PIPESTATUS[0]}\""

pnpm generateTypes

pnpm ansiToHtml $SCRIPT_DIR/build.log  $SCRIPT_DIR/build.log.html