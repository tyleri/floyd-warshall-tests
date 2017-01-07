[n, e] = [int(i) for i in raw_input().split(" ")]
edges = [[-1] * n for _ in xrange(n)]
for i in xrange(n):
    edges[i][i] = 0

for _ in xrange(e):
    [n1, n2, w] = [int(i) for i in raw_input().split(" ")]
    edges[n1-1][n2-1] = w

for k in xrange(n):
    for i in xrange(n):
        if edges[i][k] == -1:
            continue
        for j in xrange(n):
            if edges[k][j] != -1 and (edges[i][j] == -1 or edges[i][k] + edges[k][j] < edges[i][j]):
                edges[i][j] = edges[i][k] + edges[k][j]

trials = int(raw_input())

for _ in xrange(trials):
    [start, end] = [int(i) for i in raw_input().split(" ")]
    print(edges[start-1][end-1])
