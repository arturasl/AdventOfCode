#!/usr/bin/env bash

set -o nounset
set -o errexit
set -o pipefail
shopt -s failglob

main() {
    local argFrom=""
    local argTo=""
    while [[ "$#" -ne '0' ]]; do
        case "$1" in
            "--from")
                argFrom="$2"
                [[ -z "$argFrom" ]] && echo 'Unspecified --from argument' 1>&2 && exit 1
                shift 2
                ;;
            "--to")
                argTo="$2"
                [[ -z "$argTo" ]] && echo 'Unspecified --to argument' 1>&2 && exit 1
                shift 2
                ;;
            *)
                echo "Unknown argument ${1}" 1>&2
                exit 1
                ;;
        esac
    done

    local cur_dir
    cur_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"

    local newest
    newest="$(find "$cur_dir" -maxdepth 1 -name "d*p*" | sort --reverse | head --lines=1)"

    local from_base

    from_base="$(basename "$argFrom")" # d05p1
    if [[ -z "$from_base" ]]; then
        from_base="$(basename "$newest")"
    fi

    local next_base
    next_base="$(basename "$argTo")"
    if [[ -z "$next_base" ]]; then
        local newest_base
        newest_base="$(basename "$newest")"
        local newest_day
        newest_day="$(echo "$newest_base" | cut --delimiter=p --fields=1 | sed "--expression=s#^d0*##")"
        local newest_task
        newest_task="$(echo "$newest_base" | cut --delimiter=p --fields=2)"

        local next_day="$newest_day"
        local next_task="$newest_task"
        if [[ "$next_task" == "2" ]]; then
            next_day="$(( next_day + 1))"
            next_task=1
        else
            next_task=2
        fi
        next_base="$(printf "d%02dp%d" "$next_day" "$next_task")"
    fi

    echo "Will move from '${from_base}' to '${next_base}'"

    cp --recursive "${cur_dir}/${from_base}" "${cur_dir}/${next_base}"

    # Remove built artifacts.
    rm -rf "${cur_dir}/${next_base}/target"
    rm -rf "${cur_dir}/${next_base}/dist-newstyle"

    # Move files/directories having `from_base` in their name.
    dirs="$(find "${cur_dir}/${next_base}" -maxdepth 3 -name "*${from_base}*")"
    for dir in $dirs; do
        local new_name="$dir"
        new_name="$(basename "$new_name")"
        new_name="${new_name/"$from_base"/"$next_base"}"
        mv "$dir" "$(dirname "$dir")/${new_name}"
    done

    # Update usages of `from_base` in files.
    find "${cur_dir}/${next_base}" -type f -exec \
        sed --in-place "--expression=s#${from_base}#${next_base}#g" '{}' \
        ';'
}

main "$@"
