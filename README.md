# SHAKE128 implementation

Given a permutation $f$ of $b$ bits, we choose $b=r+c$ where $c$ is the capacity and $r$ is the size of the actual data you wish to hash.

## How to test

To test all unit tests written in `tests` :
```shell
cargo test
```

Some tests are built with printing in mind, use :
```shell
cargo test -- --nocapture
```
