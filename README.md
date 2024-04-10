# Advent of code 2023

See [adventofcode.com/2023](https://adventofcode.com/2023). The site uses some
OAuth providers for authentication (to see the puzzle descriptions and submit
solutions). Puzzle description page URLs look like
[adventofcode.com/2023/day/1](https://adventofcode.com/2023/day/1).

# Day 1

```
$ cd day/1
$ nix-shell -p cargo
$ cargo new solve # But this was renamed, see below.
$ cargo ./input.txt
...
Sum: 52834
$ rm -r .git
```

I could have used `cargo new --vcs none solve` to prevent the creation of the
`.git/` directory.

I've created a `day/1/solve/` directory structure, but I should have used
`day/1/` instead (same for the following days). The puzzle input files are
outside of `solve/` (i.e. `day/1/input.txt` for instance).

When implementing the `multicall` binary (see below after Day 10), I've removed
the `solve/` subdirectory, and renamed the package name from "solve" to "day1"
in the `Cargo.toml` file.

Each participant receive their own input, so it's ok to publish the results.
(And keeping them here helps during refactoring.)

In addition of `input.txt`, there is also a shorter `example.txt` file.

# Day 2

It seems I need rustc 1.61 (the above provided 1.60) to build `nom` 7.1.3. So
I've updated my nixos-unstable channel. (It's possible my stable channel is
quite old too.) This brings me to rustc 1.73.

```
$ cd day/2
$ nix-shell -I nixpkgs=channel:nixos-unstable -p cargo
```

```
$ cargo run ./input.txt
...
Sum: 2176
Sum of powers: 63700
```

# Day 3

```
$ cargo run ./input.txt
...
Part one: 525181
Part two: 84289137
```

# Day 4

```
$ cargo run ./input.txt
...
Part two: 14624680
```

# Day 5

Use

```
$ time cargo run --release ./input.txt
...
Part one: Some(322500873)
Part two: 108956227
real    1m46.364s
user    1m46.349s
sys     0m0.012s
```

(Part one is available much earlier.)

I guess that instead of resolving individual numbers (seeds), I should resolve
ranges. So that after each "stage", the output is one (best case) or multiple
ranges. And before proceeding to next stage, ranges that are "compaptible"
could be merged together.

# Day 6

For part two, I simply changed the `input.txt` file manually.

```
$ cargo run ./input-part-1.txt
...
Part one: 24655068
```

```
$ cargo run ./input.txt
...
Part one: 24655068
```

# Day 7

The code corresponds to part two. Part one is commented out/changed.

```
$ cargo run ./input.txt
...
Part one: 251824095
```

# Day 8

```
$ cargo run ./input.txt
...
Part one: 16409
["QXA", "PDA", "TDA", "QQA", "PPA", "AAA"]
QXA: 12643
PDA: 14257
TDA: 15871
QQA: 18023
PPA: 19637
AAA: 16409
Part two: 11795205644011
```

# Day 9

```
$ cargo run ./input.txt
...
Part two: 1112
```

# Day 10

```
$ cargo run --release ./input.txt
...
Part one: 6599
```

I havn't made part 2.


# Multi-calls

A binary to run all puzzles is given too.

```
$ cargo new --vcs none multicall
$ cargo run -- --day 1 ../day/1/input.txt
...
Sum: 52834
```
