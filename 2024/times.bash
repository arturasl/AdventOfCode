#!/bin/bash

time=/usr/bin/time
above_1s=0
while IFS= read -r prog || [[ -n "$prog" ]]; do
    day="$(basename "$(dirname "${prog}")")"
    output="$($time --format '%e' lua "$prog" < "./${day}/large.in" 2>&1)"
    elapsed_sec=$(echo "$output" | tail -n 1)
    if [[ ! "$elapsed_sec" =~ ^0\. ]]; then
        above_1s=$((above_1s + 1))
    fi
    result=$(echo "$output" | tail -n 2 | head -n 1)
    echo "${day}, elapsed_sec: ${elapsed_sec}s, result: ${result}"
done < <(find . -name 'main.lua' | sort)

echo "Above 1s: ${above_1s}"
