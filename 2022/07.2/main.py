import sys
from collections import defaultdict
from collections import namedtuple

DirEntry = namedtuple('DirEntry', ['sz', 'name'])
CmdLs = namedtuple('CmdLs', ['entries'])
CmdCd = namedtuple('CmdCd', ['to'])

class DirNode:
    def __init__(self, name, up):
        self.name = name
        self.dirs = {}
        self.files = None
        self.up = up
        self.sz = None

FileNode = namedtuple('FileNode', ['name', 'sz'])

def parse_cmds(os):
    def nxt():
        try:
            return next(os).strip().split()
        except StopIteration:
            return None

    line = nxt()
    while line:
        assert len(line) >= 2, line
        assert line[0] == '$', line

        if line[1] == 'cd':
            assert len(line) == 3, line
            to = line[2]
            assert '/' not in to or to == '/'
            yield CmdCd(to=to)
            line = nxt()
        elif line[1] == 'ls':
            assert len(line) == 2, line
            entries = []

            line = nxt()
            while line:
                assert len(line) >= 1, line
                if line[0] == '$':
                    break

                assert len(line) == 2, line
                name = line[1]
                assert '/' not in name
                if line[0] == 'dir':
                    entries.append(DirEntry(sz=None, name=name))
                else:
                    sz = int(line[0])
                    assert sz > 0
                    entries.append(DirEntry(sz=sz, name=name))

                line = nxt()

            yield CmdLs(entries=entries)

def traverse(cmds):
    root = DirNode(name='/', up=None)
    root.up = root
    cur = root
    for p in cmds:
        if isinstance(p, CmdLs):
            assert cur.files == None, f'f{cur.name}" is already initialized, but {p} was executed again'
            cur.files = []
            for f in p.entries:
                if f.sz is None:
                    cur.dirs[f.name] = DirNode(name=f.name, up=cur)
                else:
                    cur.files.append(FileNode(name=f.name, sz=f.sz))
        elif isinstance(p, CmdCd):
            if p.to == '/':
                cur = root
            elif p.to == '..':
                cur = cur.up
            else:
                if p.to not in cur.dirs:
                    cur.dirs[p.to] = DirNode(name=p.to, up=cur)
                cur = cur.dirs[p.to]
        else:
            assert False

    return root

def calc_dir_size(dir):
    if dir.sz is None:
        assert dir.files is not None
        dir.sz = sum(f.sz for f in dir.files) + sum(calc_dir_size(d) for d in dir.dirs.values())
    return dir.sz

def print_dir(dir, ind=''):
    print(f'{ind} {dir.name} (dir, size={dir.sz:,}))')
    ind += '  '
    for f in dir.files:
        print(f'{ind} {f.name} (file, size={f.sz:,}))')
    for d in dir.dirs.values():
        print_dir(d, ind)

def find_dirs_above(dir, max_sz):
    if dir.sz >= max_sz:
        yield dir
    for d in dir.dirs.values():
        yield from find_dirs_above(d, max_sz)

def main():
    cmds = parse_cmds(iter(sys.stdin))
    root = traverse(cmds)
    calc_dir_size(root)
    print_dir(root)
    available = 70_000_000
    need = 30_000_000
    find_above = need - (available - root.sz)
    if find_above > 0:
        best = min(find_dirs_above(root, find_above), key=lambda d: d.sz)
        print(best.sz)

if __name__ == "__main__":
    main()
