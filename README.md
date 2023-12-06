# Day 1

```
$ cd day/1
$ nix-shell -p cargo
$ cargo new solve
$ cd solve/
$ cargo run
$ rm -r .git
```

## Part ~one and~ two

```
$ cargo run ../input.txt
```

# Day 2

It seems I need rustc 1.61 (the above provided 1.60) to build `nom` 7.1.3. So
I've updated my nixos-unstable channel. (It's possible my stable channel is
quite old too.) This brings me to rustc 1.73.

```
$ cd day/2
$ nix-shell -I nixpkgs=channel:nixos-unstable -p cargo
$ cargo new solve
$ cd solve/
```

# Day 3 and 4

Similarly as before.

# Day 5

Use

```
$ time cargo run --release
...
real    1m46.364s
user    1m46.349s
sys     0m0.012s
```

I guess that instead of resolving individual numbers (seeds), I should resolve
ranges. So that after each "stage", the output is one (best case) or multiple
ranges. And before proceeding to next stage, ranges that are "compaptible"
could be merged together.

# Day 6

For part two, I simply changed the `input.txt` file manually.
