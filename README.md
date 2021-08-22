# VarInt Encoding Benchmark

This repo was inspired by https://steinborn-me.pages.dev/posts/performance/how-fast-can-you-write-a-varint/ but using Rust rather than Java.

# Results

All benchmarks were performed with a Ryzen 1700 CPU.

```
test tests::blended  ... bench:   3,059,360 ns/iter (+/- 21,405)
test tests::bungee   ... bench:   7,298,080 ns/iter (+/- 309,710)
test tests::lucky5   ... bench:   2,479,300 ns/iter (+/- 15,811)
test tests::velocity ... bench:   6,171,420 ns/iter (+/- 158,624)
```

## What about an array?

I was curious what the differences are with using an `[u8; 5]` array as the write buffer rather than a `Vec<u8>`. This was the outcome:

```
test tests::blended  ... bench:   2,017,167 ns/iter (+/- 305,102)
test tests::bungee   ... bench:   2,504,040 ns/iter (+/- 60,773)
test tests::lucky5   ... bench:      45,386 ns/iter (+/- 9,554)
test tests::velocity ... bench:   7,023,560 ns/iter (+/- 58,624)
```

Lucky5 is orders of magnitude faster? And the bungee writer now surpasses velocity's old writer?

The array version is found in the `arrays` branch.