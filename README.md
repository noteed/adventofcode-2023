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
