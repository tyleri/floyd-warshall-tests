# Floyd-Warshall Tests

## Backstory

I learned about the Floyd-Warshall algorithm from
[a HackerRank problem](https://www.hackerrank.com/challenges/floyd-city-of-blinding-lights).
Choosing my favorite language, Python, I quickly wrote up a solution in ~22
lines of code. However, I was shocked to see that it timed out on 3 of the test
cases. Some of the discussions suggested trying compiled languages, so I
worked on implementing one in Go, a language I had some interest in.

To my surprise, it not only passed all of the test cases easily, but in terms
of speed, it also outperformed Python on my laptop by 11x. Although I've known
that compiled languages are faster than interpreted languages, this is the first
time I've appreciated how much of a difference it makes, since I wrote
essentially the same code in two different languages and ended up with vastly
different results.

After seeing the difference between Go and Python, I decided to test the runtime
of this algorithm on other languages I knew or had interest in. That's how I
ended up creating this repository with the programs written in different
languages. Although this is a very unscientific test, below are the results I
observed when running these programs on my laptop.

## Results

input03.txt
  - luajit:   0.15s
  - go:       0.49s
  - java:     1.38s
  - ocaml:    1.55s
  - python2:  4.99s
  - python3:  7.40s 

input04.txt
  - go:       0.71s
  - java:     1.52s
  - luajit:   1.70s
  - ocaml:    5.69s
  - python2:  35.46s
  - python3:  47.50s

input05.txt
  - go:       0.66s
  - luajit:   1.31s
  - java:     1.40s
  - ocaml:    5.54s
  - python2:  35.73s
  - python3:  49.01s
