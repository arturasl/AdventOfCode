#!/usr/bin/env bash

set -o nounset
set -o errexit
set -o pipefail
shopt -s failglob

time=/usr/bin/time
above_1s=0
for main_file in */app/Main.hs; do
    day="$(basename "$(dirname "$(dirname "${main_file}")")")"

    cd "$day"
    cabal build --verbose=0
    output="$($time --format '%e' cabal run < "./app/large.in" 2>&1)"
    elapsed_sec=$(echo "$output" | tail -n 1)
    if [[ ! "$elapsed_sec" =~ ^0\. ]]; then
        above_1s=$((above_1s + 1))
    fi
    result=$(echo "$output" | tail -n 2 | head -n 1)

    echo "${day}, elapsed_sec: ${elapsed_sec}s, result: ${result}"
    cd ..
done

echo "Above 1s: ${above_1s}"
