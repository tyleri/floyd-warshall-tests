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
languages.

## Summarized Results

input03.txt
  - luajit:   0.1602s
  - rust:     0.3030s
  - ocaml:    0.3990s
  - pypy2:    0.7616s
  - java:     1.1634s
  - pypy3:    1.3644s
  - scala:    1.9828s
  - go:       2.1626s
  - python2:  4.8068s
  - python3:  7.9362s

input04.txt
  - rust:     0.3982s
  - ocaml:    0.5454s
  - java:     1.2722s
  - luajit:   1.5498s
  - scala:    2.1416s
  - pypy2:    2.2284s
  - go:       2.5196s
  - pypy3:    2.6224s
  - python2:  28.0788s
  - python3:  45.0692s

input05.txt
  - rust:     0.4114s
  - ocaml:    0.5416s
  - luajit:   1.2412s
  - java:     1.271s
  - scala:    2.0392s
  - pypy2:    2.148s
  - go:       2.5192s
  - pypy3:    2.5372s
  - python2:  28.2238s
  - python3:  44.0434s

## Future

I may continue adding to this repository when I have time, testing the runtime
of this algorithm with other languages.
