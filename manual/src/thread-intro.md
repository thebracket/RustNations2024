# Let's Do Some Threading!

To ensure you are all comfortable with the concepts here, let's build a simple threaded application. You won't hear that in many other language venues - *simple* and *threaded* don't often live together.

In this short workshop we will:

1. Define a function that detects if a number is prime. We'll do it quite inefficiently!
2. We'll generate a list of candidate numbers.
3. We'll divide the list into "chunks" of roughly equal size.
4. We'll spawn `n` threads, each of which will tackle a chunk of the data.
5. If a thread determines a number to be prime, it'll be appended to a shared vector.
6. We output execution time and list of primes to `stdout`.

Let's build a single-threaded version and establish our workload.
