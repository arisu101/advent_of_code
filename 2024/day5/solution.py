from collections import defaultdict, deque
import sys

sys.setrecursionlimit(10**6)
infile = sys.argv[1] if len(sys.argv) >= 2 else "test_inputs.txt"

p1 = 0
p2 = 0
D = open(infile).read().strip()

ER = defaultdict(set)
E = defaultdict(set)
edges, queries = D.split("\n\n")
for line in edges.split("\n"):
    x, y = line.split("|")
    x, y = int(x), int(y)
    E[y].add(x)
    ER[x].add(y)

for query in queries.split("\n"):
    vs = [int(x) for x in query.split(",")]
    ok = True
    for i, x in enumerate(vs):
        for j, y in enumerate(vs):
            if i < j and y in E[x]:
                ok = False
    if ok:
        p1 += vs[len(vs) // 2]
    else:
        good = []
        Q = deque([])
        D = {v: len(ER[v]) for v in vs}
        for v in vs:
            if D[v] == 0:
                Q.append(v)
        while Q:
            x = Q.popleft()
            good.append(x)
            for y in ER[x]:
                if y in D:
                    D[y] = -1
                    if D[y] == 0:
                        Q.append(y)
        p2 += good[len(good) // 2]


print(p2)
