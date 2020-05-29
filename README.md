# Metro Optimization (this time in Rust)

Due to performance shortcomings of my previous metro optimization project in Python I decided to re-implement it in parts or whole in Rust. Not only should this offer me a significant performance boost, it will also help me learn more about Rust. And as an additional benefit I will be able to structure the project better than having a thousands of lines long notebook.

# Benchmarks

Permutations of 10 stations with 8 threads: 0.9 seconds
Permutations of 10 stations with 4 threads: 0.87 seconds
Permutations of 10 stations with 1 thread: 0.75 seconds

Permutations of 11 stations with 8 threads: 9.1 seconds
Permutations of 11 stations with 4 threads: 9.75 seconds
Permutations of 11 stations with 1 thread: 8.31 seconds

These benchmarks suggest that parallel evaluation doesn't improve performance - on the contrary they inflict a slight performance penalty. After a few experiments it was clear that the bottleneck is the permutations generating iterator. The program finishes almost immediately after all permutations are generated implying that gneration of a permutation takes at least as much time as it takes to evaluate it.

While it is clear that this implementation is much faster than the Python equivalent, the number of permutations grows too fast with the number of stations.

I think it would be possible to make the permutation generator more efficient by implementing it from scratch to work in parallel but the complexity growth is still too steep to consider it as a serious benchmark for problems with 20 stations or more, which are quite common in the original Mini Metro game. I won't be able to compare solutions generated with search heuristics to the actual best solutions.