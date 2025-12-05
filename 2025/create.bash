#!/usr/bin/env bash

set -o nounset
set -x
set -o errexit
set -o pipefail
shopt -s failglob

main() {
    local dir="$1"
    mkdir -p "$dir" || true
    cd "$dir"

    npm init --yes
    npm pkg set type="module"
    npm pkg set scripts.start="npx ts-node index.ts"

    npm install --save-dev typescript ts-node
    npm install --save-dev @types/node

    npx tsc --init \
        --module NodeNext \
        --moduleResolution NodeNext \
        --target es2020 \
        --types node

    sed -i'' -e '/\/\//d' tsconfig.json

    cat > index.ts <<EOF
import readline from "node:readline";
import assert from "node:assert";

async function main() {
  for await (let line of readline.createInterface({
    input: process.stdin,
  })) {
    line = line.trim();
    if (!line) {
      continue;
    }

    console.log(line);
  }
}

main();
EOF
}

main "$@"
