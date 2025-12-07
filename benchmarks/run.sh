#!/bin/bash
set -e

# Ensure standard paths are available
export PATH="/opt/homebrew/bin:/usr/local/bin:$PATH"

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if hyperfine is installed
if ! command -v hyperfine &> /dev/null; then
    echo -e "${YELLOW}Warning: hyperfine is not installed.${NC}"
    echo "Install it with: brew install hyperfine"
    echo ""
    echo "Falling back to manual timing..."
    USE_HYPERFINE=false
else
    USE_HYPERFINE=true
fi

# Build release binary
echo -e "${BLUE}Building Neon in release mode...${NC}"
cd "$(dirname "$0")/.."
cargo build --release --quiet
echo ""

NEON_BIN="./target/release/neon"
BENCH_DIR="./benchmarks"

# Function to run a single benchmark with hyperfine
run_benchmark_hyperfine() {
    local name=$1
    local neon_file="${BENCH_DIR}/${name}.n"
    local python_file="${BENCH_DIR}/${name}.py"

    echo -e "${GREEN}=== ${name} benchmark ===${NC}"
    hyperfine --warmup 3 --min-runs 10 \
        "${NEON_BIN} ${neon_file}" \
        "python3 ${python_file}"
    echo ""
}

# Function to run a single benchmark manually
run_benchmark_manual() {
    local name=$1
    local neon_file="${BENCH_DIR}/${name}.n"
    local python_file="${BENCH_DIR}/${name}.py"

    echo -e "${GREEN}=== ${name} benchmark ===${NC}"

    echo "Neon (5 runs):"
    for i in {1..5}; do
        /usr/bin/time -p "${NEON_BIN}" "${neon_file}" 2>&1 | grep real
    done

    echo ""
    echo "Python (5 runs):"
    for i in {1..5}; do
        /usr/bin/time -p python3 "${python_file}" 2>&1 | grep real
    done
    echo ""
}

# Main benchmark runner
run_benchmark() {
    if [ "$USE_HYPERFINE" = true ]; then
        run_benchmark_hyperfine "$1"
    else
        run_benchmark_manual "$1"
    fi
}

# Parse command line arguments
if [ $# -eq 0 ]; then
    # Run all benchmarks
    echo -e "${BLUE}Running all benchmarks...${NC}"
    echo ""
    run_benchmark "fib"
    run_benchmark "loops"
    run_benchmark "primes"
else
    # Run specific benchmark
    case "$1" in
        fib|fibonacci)
            run_benchmark "fib"
            ;;
        loops)
            run_benchmark "loops"
            ;;
        primes)
            run_benchmark "primes"
            ;;
        *)
            echo "Unknown benchmark: $1"
            echo "Available benchmarks: fib, loops, primes"
            echo "Usage: $0 [fib|loops|primes]"
            echo "       $0           # run all benchmarks"
            exit 1
            ;;
    esac
fi

echo -e "${GREEN}Done!${NC}"
