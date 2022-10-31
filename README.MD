# Demo project for graph library

## TGF
All tgf files are in tgf directory

## How to run
You can pass path to `.tgf` file as first arg to binary
```bash
cargo run -- ./tgf/nums.tgf
```

## Output looks like
```bash
Id: 1, neighbors: [ 2 3 4 5 ], Value: 54
Id: 2, neighbors: [ ], Value: 2
Id: 3, neighbors: [ 1 2 4 ], Value: 3
Id: 4, neighbors: [ ], Value: 4
Id: 5, neighbors: [ 2 ], Value: 5
```
