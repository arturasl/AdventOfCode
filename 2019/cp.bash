#!/usr/bin/env bash

set -o nounset
set -o errexit
set -o pipefail
shopt -s failglob

main() {
    local cur_dir
    cur_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
    local newest
    newest="$(find  "$cur_dir" -maxdepth 1 -name "d*_*" | sort --reverse | head --lines=1)"

    local from="${1-}"
    if [[ -z "$from" ]]; then
        from="$newest"
    fi
    local from_base
    from_base="$(basename "$from")" # d05_1

    local newest_base
    newest_base="$(basename "$newest")"
    local newest_day
    newest_day="$(echo "$newest_base" | cut --delimiter=_ --fields=1 | sed "--expression=s#^d0*##")"
    local newest_task
    newest_task="$(echo "$newest_base" | cut --delimiter=_ --fields=2)"

    local next_day="$newest_day"
    local next_task="$newest_task"
    if [[ "$next_task" == "2" ]]; then
        next_day="$(( next_day + 1))"
        next_task=1
    else
        next_task=2
    fi
    local next_base
    next_base="$(printf "d%02d_%d" "$next_day" "$next_task")"

    cp --recursive "$from" "${cur_dir}/${next_base}"
    dirs="$(find "${cur_dir}/${next_base}" -name "${from_base}")"
    for dir in $dirs; do
        mv "$dir" "$(dirname "$dir")/${next_base}"
    done
    find "${cur_dir}/${next_base}" -type f -exec \
        sed --in-place "--expression=s#${from_base}#${next_base}#g" '{}' \
        ';'

    local from_base_dash
    from_base_dash=$(echo "$from_base" | tr '_' '-')
    local next_base_dash
    next_base_dash=$(echo "$next_base" | tr '_' '-')
    find "${cur_dir}/${next_base}" -type f -exec \
        sed --in-place "--expression=s#${from_base_dash}#${next_base_dash}#g" '{}' \
        ';'
}

main "$@"
