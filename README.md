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
  - luajit:   0.196s
  - ocaml:    0.549s
  - java:     1.504s
  - go:       2.351s
  - python2:  7.433s
  - python3:  12.536s

input04.txt
  - ocaml:    0.958s
  - java:     2.097s
  - luajit:   2.777s
  - go:       3.214s
  - python2:  58.300s
  - python3:  90.017s

input05.txt
  - ocaml:    0.734s
  - java:     1.637s
  - luajit:   1.690s
  - go:       3.104s
  - python2:  42.590s
  - python3:  91.992s

## Future

I may continue adding to this repository when I have time, testing the runtime
of this algorithm with other languages.
