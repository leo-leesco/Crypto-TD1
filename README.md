# SHAKE128 implementation

## How to test

To test all unit tests written in `tests` :
```shell
cargo test
```

## How to build

Simply run :
```shell
cargo build --release
```

Usage :
```shell
./target/release/shake128 <hash_size_in_bytes>
```

The hash will be calculated on `stdin`.
