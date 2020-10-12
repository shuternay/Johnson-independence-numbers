# Independence numbers of Johnson-type graphs

This repository contains calculations code for [Independence numbers of Johnson-type graphs](https://arxiv.org/pdf/1907.06752.pdf) [1].
The code calculates independence number of a given Johnsons-type graph using Östergård algorithm [2].


## Usage

To compile the code you will need to install [Rust compiler](https://www.rust-lang.org/learn/get-started).
To calculate the independence number for J_+/-(n, k, t) run the following command:
```
cargo run --release <n> <k> <t>
```
For example,
```
cargo run --release 5 2 -1
```

Output will look as follows:
```
# List of J_+/-(n, k, t) vertices with their internal indices
Vertices:
0 [1, 1, 0, 0, 0]
1 [-1, 1, 0, 0, 0]
2 [1, -1, 0, 0, 0]
3 [-1, -1, 0, 0, 0]
4 [1, 0, 1, 0, 0]
5 [-1, 0, 1, 0, 0]
6 [1, 0, -1, 0, 0]
7 [-1, 0, -1, 0, 0]
...
# One step of the algorithm. Coeffs correspond to c_i in [2], that is, the maximal independent set size on vertices with indices >= i. They are calculated from right to left
progress 14/40, time: 0 s., coeffs: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3, 2, 1]
...
Found a clique of size 7
Indices: {25, 26, 27, 28, 29, 30, 31}
Vertices:
[0, -1, 0, 0, 1]
[0, 1, 0, 0, -1]
[0, -1, 0, 0, -1]
[0, 0, 1, 1, 0]
[0, 0, -1, 1, 0]
[0, 0, 1, -1, 0]
[0, 0, -1, -1, 0]
...
Independence number of J_+/-(5, 2, -1) is 10
```

Optionally you can fix some vertices and calculate the maximal size of an independent set which contains these vertices:
```
cargo run --release <n> <k> <t> <indices of fixed vertices ...>
```
For example,
```
# will fix vertices [1, 1, 0, 0, 0] and [-1, 0, 1, 0, 0]
cargo run --release 5 2 -1  0 5
```

Since J_+/-(n, k, t) is a vertex transitive graph, you can always fix one vertex to make calculations faster.



## References

[1] Cherkashin, Danila, and Sergei Kiselev. "Independence numbers of Johnson-type graphs." arXiv preprint arXiv:1907.06752 (2019).

[2] Östergård, Patric RJ. "A fast algorithm for the maximum clique problem." Discrete Applied Mathematics 120.1-3 (2002): 197-207.