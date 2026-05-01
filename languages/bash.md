```bash
# Split string by a single character delimiter.
str='a; b;;c ' # Note that last delimiter is ignored, e.g. same as 'a; b;;c ;'.
delim=';'
readarray -d "$delim" -t parts < <(printf "%s" "$str")
for part in "${parts[@]}"; do
    echo "part: '${part}'"
done

# Same as above, but works with multicharacter delimiters.
str='a;; b;;;;c '
delim=';;'
str="${str}${delim}"
while [[ $str == *"$delim"* ]]; do
    part="${str%%"$delim"*}"
    str="${str#*"$delim"}"
    echo "part: '${part}'"
done
```
