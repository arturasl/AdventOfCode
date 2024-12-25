#!/bin/bash

time=/usr/bin/time
while IFS= read -r prog || [[ -n "$prog" ]]; do
    day="$(basename "$(dirname "${prog}")")"
    output="$($time --format '%e' lua "$prog" < "./${day}/large.in" 2>&1)"
    elapsed_sec=$(echo "$output" | tail -n 1)
    result=$(echo "$output" | tail -n 2 | head -n 1)
    echo "${day}, elapsed_sec: ${elapsed_sec}s, result: ${result}"
done < <(find . -name 'main.lua' | sort)
