object FloydScala {
    def main(args: Array[String]) {
        val stdin = scala.io.StdIn

        val Array(n, e) = stdin.readLine.split(" ").map(x => x.toInt)
        val edges = Array.ofDim[Int](n, n)

        for (i <- 0 until n) {
            for (j <- 0 until n) {
                edges(i)(j) = if (i == j) 0 else -1
            }
        }

        for (_ <- 0 until e) {
            val Array(n1, n2, w) = stdin.readLine.split(" ").map(x => x.toInt)
            edges(n1-1)(n2-1) = w
        }

        for (k <- 0 until n) {
            for (i <- 0 until n) {
                if (edges(i)(k) != -1) {
                    for (j <- 0 until n) {
                        if (edges(k)(j) != -1 && (edges(i)(j) == -1 || edges(i)(k) + edges(k)(j) < edges(i)(j))) {
                            edges(i)(j) = edges(i)(k) + edges(k)(j)
                        }
                    }
                }
            }
        }

        val trials = stdin.readLine.toInt

        for (_ <- 0 until trials) {
            val Array(start, end) = stdin.readLine.split(" ").map(x => x.toInt)
            println(edges(start-1)(end-1))
        }
    }
}