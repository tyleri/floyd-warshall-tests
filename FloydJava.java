import java.util.Scanner;

public class FloydJava {

  public static void main(String[] args) {
    Scanner s = new Scanner(System.in);

    int n = s.nextInt();
    int e = s.nextInt();
    int[][] edges = new int[n][n];

    for (int i = 0; i < n; i++) {
      for (int j = 0; j < n; j++) {
        if (i != j) {
          edges[i][j] = -1;
        }
      }
    }

    for (int i = 0; i < e; i++) {
      int n1 = s.nextInt()-1;
      int n2 = s.nextInt()-1;
      int w = s.nextInt();
      edges[n1][n2] = w;
    }

    for (int k = 0; k < n; k++) {
      for (int i = 0; i < n; i++) {
        if (edges[i][k] == -1) {
          continue;
        }
        for (int j = 0; j < n; j++) {
          if (edges[k][j] != -1 && (edges[i][j] == -1 || edges[i][k] + edges[k][j] < edges[i][j])) {
            edges[i][j] = edges[i][k] + edges[k][j];
          }
        }
      }
    }

    int tests = s.nextInt();

    for (int i = 0; i < tests; i++) {
      int nt1 = s.nextInt()-1;
      int nt2 = s.nextInt()-1;
      System.out.println(edges[nt1][nt2]);
    }
  }
}
