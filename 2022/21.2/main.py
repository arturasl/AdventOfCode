import sys
import re
import operator
from fractions import Fraction

def op(o):
    return {
        '*': operator.mul,
        '/': operator.truediv,
        '+': operator.add,
        '-': operator.sub
    }[o]

def main():
    cache = {}

    def get(name):
        if type(cache[name]) == type(Fraction()) or cache[name] is None:
            return cache[name]

        l, o, r = cache[name]
        l = get(l)
        o = op(o)
        r = get(r)

        if l is None or r is None:
            return None

        cache[name] = o(l, r)
        return cache[name]


    for line in (line.strip().replace(' ', '') for line in sys.stdin):
        parts = line.split(':')

        assert len(parts) == 2
        name = parts[0]

        m = re.match(r'^(\w+)([/*+-])(\w+)$', parts[1])
        if m:
            m = m.groups()
            cache[name] = (m[0], m[1], m[2])
        else:
            cache[name] = Fraction(int(parts[1]))

    cache['humn'] = None
    l, r = get(cache['root'][0]), get(cache['root'][2])
    if r is None:
        l, r = r, l
        cache['root'] = (cache['root'][2], cache['root'][1], cache['root'][0])

    l = cache['root'][0]
    while True:
        if cache[l] is None:
            break

        ll, oo, rr = cache[l]

        if get(ll) is None:
            r = {
                '*': operator.truediv,
                '/': operator.mul,
                '+': operator.sub,
                '-': operator.add
            }[oo](r, get(rr))
            l = ll
        else:
            r = {
                '*': operator.truediv,
                '/': operator.truediv,
                '+': operator.sub,
                '-': lambda a, b: -(a - b)
            }[oo](r, get(ll))
            l = rr

    print(r)

if __name__ == "__main__":
    main()
