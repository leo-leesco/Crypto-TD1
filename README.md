# SHAKE128 implementation

This project can be cloned from [GitHub](git@github.com:leo-leesco/Crypto-TD1.git).

## How to test

To test all tests :
```shell
cargo test
```

Note that running the tests has the side effect of building `target/debug/shake128`.

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
