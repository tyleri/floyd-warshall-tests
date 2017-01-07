n,e = io.read("*n", "*n")
edges = {}
for i = 1,n do
  edges[i] = {}
  for j = 1,n do
    if (i == j) then edges[i][j] = 0 else edges[i][j] = -1 end
  end
end

for i = 1,e do
  n1,n2,w = io.read("*n", "*n", "*n")
  edges[n1][n2] = w
end

for k = 1,n do
  for i = 1,n do
    if (edges[i][k] ~= -1) then
      for j = 1,n do
        if (edges[k][j] ~= -1 and (edges[i][j] == -1 or edges[i][k] + edges[k][j] < edges[i][j])) then
          edges[i][j] = edges[i][k] + edges[k][j]
        end
      end
    end
  end
end

tests = io.read("*n")
for i = 1,tests do
  nt1,nt2 = io.read("*n", "*n")
  print(edges[nt1][nt2])
end

