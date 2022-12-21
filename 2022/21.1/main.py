import sys
import re
import operator

def main():
    cache = {}

    def get(name):
        if type(cache[name]) == type(1):
            return cache[name]
        cache[name] = cache[name]()
        return cache[name]

    for line in (line.strip().replace(' ', '') for line in sys.stdin):
        parts = line.split(':')

        assert len(parts) == 2
        name = parts[0]

        m = re.match(r'^(\w+)([/*+-])(\w+)$', parts[1])
        if m:
            m = m.groups()
            cache[name] = lambda x=m: (
                {'*': operator.mul,
                 '/': operator.floordiv,
                 '+': operator.add,
                 '-': operator.sub}[x[1]](get(x[0]), get(x[2])))
        else:
            cache[name] = int(parts[1])

    print(get('root'))

if __name__ == "__main__":
    main()
