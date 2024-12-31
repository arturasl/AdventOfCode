#!/bin/bash

time=/usr/bin/time
above_1s=0
while IFS= read -r main_file || [[ -n "$main_file" ]]; do
    day="$(basename "$(dirname "${main_file}")")"

    program='cargo'
    $program build --release --bin "$day" 2>/dev/null
    output="$($time --format '%e' $program run --release --bin "$day" < "./src/${day}/large.in" 2>&1)"
    elapsed_sec=$(echo "$output" | tail -n 1)
    if [[ ! "$elapsed_sec" =~ ^0\. ]]; then
        above_1s=$((above_1s + 1))
    fi
    result=$(echo "$output" | tail -n 2 | head -n 1)

    echo "${program} ${day}, elapsed_sec: ${elapsed_sec}s, result: ${result}"
done < <(find . -name 'main.rs' | sort)

echo "Above 1s: ${above_1s}"
