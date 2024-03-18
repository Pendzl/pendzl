#!/bin/bash
SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
ALREADY_EXISTING_NODE_PID=$(lsof -t -i :9944 -s TCP:LISTEN)
if [ ! -z "$ALREADY_EXISTING_NODE_PID" ]; then
    echo "Killing process $ALREADY_EXISTING_NODE_PID occupying test port"
    kill $ALREADY_EXISTING_NODE_PID
fi
TMP_DIR_NAME="test-chain-state-tmp"
TEST_BP_DIR="test-chain-state-bp"
rm -rf test-chain-state*
rm $SCRIPT_DIR/*.testrun*.log
mkdir $TMP_DIR_NAME
($SCRIPT_DIR/substrate-contracts-node --dev --base-path $SCRIPT_DIR/$TMP_DIR_NAME --rpc-port 9944 &> substrate-contracts-node.testrun.log)&
NODE_PID=$!
sleep 3 #precautiously wait for node to finish start up
export NODE_OPTIONS=$NODE_OPTIONS" --max-old-space-size=16384"
start_time=$(date +%s.%3N)
#for debugging memory leaks, unfreed handles: npx mocha => npx wtfnode node_modules/.bin/_mocha
script -efq $SCRIPT_DIR/mocha.testrun.log -c \
"env CARGO_TERM_COLOR=always FORCE_COLOR=1  npx tsx $SCRIPT_DIR/runWithoutWarnings.ts npx mocha --node-option max-old-space-size=16384 --config ./.mocharc.js -C --exit --full-trace false --require tsx/cjs --require 'tests/setup/globalHooks.ts' 'tests/**/*.ts' --colors"

end_time=$(date +%s.%3N)
elapsed=$(echo "scale=3; $end_time - $start_time" | bc)
echo "Test execution took $elapsed seconds"
npx tsx $SCRIPT_DIR/scripts/fixupNodeLog.ts $SCRIPT_DIR/substrate-contracts-node.testrun.log
kill $NODE_PID

rm -rf test-chain-state-tmp