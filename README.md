# Prime Number Generator & Checker

this is just a small rust code to check if a number is a prime
its also possible to generate primes in a range

## Benchmarks (prime number generator)
made on Apple MacBook Air M2

./prims --gen 1 100             | total: 0.004s

./prims --gen 1 1000            | total: 0.005s

./prims --gen 1 10000           | total: 0.006s

./prims --gen 1 100000          | total: 0.028s

./prims --gen 1 1000000         | total: 0.571s 

## Usage

#### get the help
./prims --help

#### check if 73 is a prim number
./prims --check 73

#### generate all prim numbers between 10 and 1000
./prims --gen 10 1000

## Installation
download the repository and then either build it yourself
via `cargo build --release` and find the release in `target/release/prims`
or you use just the prebuild prims in the release folder

also you can debug run the code via `cargo run`

*Note: for the cargo commands you need to have cargo installed ...*
