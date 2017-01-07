[n, e] = [int(i) for i in input().split(" ")]
edges = [[-1] * n for _ in range(n)]
for i in range(n):
    edges[i][i] = 0

for _ in range(e):
    [n1, n2, w] = [int(i) for i in input().split(" ")]
    edges[n1-1][n2-1] = w

for k in range(n):
    for i in range(n):
        if edges[i][k] == -1:
            continue
        for j in range(n):
            if edges[k][j] != -1 and (edges[i][j] == -1 or edges[i][k] + edges[k][j] < edges[i][j]):
                edges[i][j] = edges[i][k] + edges[k][j]

trials = int(input())

for _ in range(trials):
    [start, end] = [int(i) for i in input().split(" ")]
    print(edges[start-1][end-1])
