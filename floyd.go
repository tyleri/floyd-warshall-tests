package main
import ("fmt")

func main() {
  var n,e int
  fmt.Scanf("%d %d", &n, &e)

  // create 2D array
  edges := make([][]int, n)
  for i := 0; i < n; i++ {
    edges[i] = make([]int, n)
    for j := 0; j < n; j++ {
      if i != j {
        edges[i][j] = -1
      }
    }
  }

  var n1,n2,w int
  for i := 0; i < e; i++ {
    fmt.Scanf("%d %d %d", &n1, &n2, &w)
    edges[n1-1][n2-1] = w
  }

  for k := 0; k < n; k++ {
    for i := 0; i < n; i++ {
      if edges[i][k] != -1 {
        for j := 0; j < n; j++ {
          if edges[k][j] != -1 && (edges[i][j] == -1 || edges[i][k] + edges[k][j] < edges[i][j]) {
            edges[i][j] = edges[i][k] + edges[k][j]
          }
        }
      }
    }
  }

  var tests int
  fmt.Scanf("%d", &tests)

  var nt1,nt2 int
  for i := 0; i < tests; i++ {
    fmt.Scanf("%d %d", &nt1, &nt2)
    fmt.Println(edges[nt1-1][nt2-1])
  }
}
