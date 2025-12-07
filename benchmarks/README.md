# Neon vs Python Benchmarks

This directory contains benchmarks for comparing Neon's execution speed against Python3.

## Benchmarks

1. **fibonacci (fib.n / fib.py)** - Recursive Fibonacci calculation
   - Tests function call overhead and recursion performance
   - Calculates fib(35) = 9,227,465

2. **loops (loops.n / loops.py)** - Nested loops
   - Tests loop performance and variable assignment
   - Executes 1,000,000 iterations (1000 x 1000)

3. **primes (primes.n / primes.py)** - Prime number counting
   - Tests arithmetic operations, conditionals, and loops
   - Counts primes up to 10,000 (result: 1,229 primes)

## Running Benchmarks

### Prerequisites

For best results, install [hyperfine](https://github.com/sharkdp/hyperfine):
```bash
brew install hyperfine
```

If hyperfine is not installed, the script will fall back to manual timing.

### Run All Benchmarks

```bash
./benchmarks/run.sh
```

### Run Individual Benchmark

```bash
./benchmarks/run.sh fib      # Fibonacci only
./benchmarks/run.sh loops    # Loops only
./benchmarks/run.sh primes   # Primes only
```

## Manual Benchmarking

You can also run benchmarks manually:

```bash
# Build release binary first
cargo build --release

# Run with hyperfine
hyperfine --warmup 3 \
  './target/release/neon benchmarks/fib.n' \
  'python3 benchmarks/fib.py'

# Or run directly
./target/release/neon benchmarks/fib.n
python3 benchmarks/fib.py
```

## Expected Performance

Neon is expected to be faster than Python for these CPU-intensive benchmarks since:
- Neon compiles to bytecode executed by a VM written in Rust
- Python uses an interpreted bytecode VM
- Both use similar execution models, making this a fair comparison

Actual results will vary based on your hardware and system load.
