#!/usr/bin/env python3
"""Benchmark driver: runs paired Neon/Python benchmarks and compares timings."""

import argparse
import json
import os
import statistics
import subprocess
import sys
import time

BENCHMARKS = {
    "fib": 31,
    "loop_arith": 1500000,
    "closures": 80000,
    "structs": 80000,
    "nbody": 15000,
}

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
NEON_BIN = os.path.join(REPO_ROOT, "target", "release", "neon")


def run_once(cmd):
    start = time.perf_counter()
    result = subprocess.run(cmd, capture_output=True, text=True)
    elapsed = time.perf_counter() - start
    if result.returncode != 0:
        print(f"command failed: {' '.join(cmd)}", file=sys.stderr)
        print(result.stderr, file=sys.stderr)
        sys.exit(1)
    return elapsed, result.stdout.strip()


def time_command(cmd, runs):
    run_once(cmd)  # warm-up
    durations = []
    checksum = None
    for _ in range(runs):
        elapsed, output = run_once(cmd)
        durations.append(elapsed * 1000)
        checksum = output
    return durations, checksum


def stats(durations):
    return {
        "mean": statistics.mean(durations),
        "stddev": statistics.stdev(durations) if len(durations) > 1 else 0.0,
        "min": min(durations),
        "median": statistics.median(durations),
    }


def main():
    parser = argparse.ArgumentParser(description="Run Neon vs Python benchmarks")
    parser.add_argument("names", nargs="*", help="benchmark names to run (default: all)")
    parser.add_argument("--runs", type=int, default=5, help="timed runs per benchmark (default: 5)")
    parser.add_argument("--json", default="bench-results.json", help="path to write results JSON")
    args = parser.parse_args()

    if args.names:
        unknown = [n for n in args.names if n not in BENCHMARKS]
        if unknown:
            print(f"unknown benchmark(s): {', '.join(unknown)}", file=sys.stderr)
            sys.exit(1)
        selected = args.names
    else:
        selected = list(BENCHMARKS.keys())

    if not os.path.isfile(NEON_BIN):
        print(f"missing {NEON_BIN} — run `cargo build --release` first", file=sys.stderr)
        sys.exit(1)

    mismatches = []
    rows = []
    json_entries = []

    for name in selected:
        size = BENCHMARKS[name]
        neon_script = os.path.join(REPO_ROOT, "benches", f"{name}.n")
        python_script = os.path.join(REPO_ROOT, "benches", f"{name}.py")

        neon_durations, neon_checksum = time_command([NEON_BIN, neon_script, str(size)], args.runs)
        python_durations, python_checksum = time_command(
            [sys.executable, python_script, str(size)], args.runs
        )

        if neon_checksum != python_checksum:
            mismatches.append((name, neon_checksum, python_checksum))

        neon_stats = stats(neon_durations)
        python_stats = stats(python_durations)
        ratio = neon_stats["mean"] / python_stats["mean"]

        rows.append((name, neon_stats, python_stats, ratio))

        json_entries.append(
            {
                "name": f"{name} neon (ms)",
                "unit": "ms",
                "value": neon_stats["mean"],
                "range": f"± {neon_stats['stddev']:.3f}",
            }
        )
        json_entries.append(
            {
                "name": f"{name} python (ms)",
                "unit": "ms",
                "value": python_stats["mean"],
                "range": f"± {python_stats['stddev']:.3f}",
            }
        )
        json_entries.append(
            {
                "name": f"{name} neon/python",
                "unit": "ratio",
                "value": ratio,
            }
        )

    table_lines = [
        "| Benchmark | Neon mean | Neon stddev | Neon min | Neon median | "
        "Python mean | Python stddev | Python min | Python median | Neon/Python |",
        "|---|---|---|---|---|---|---|---|---|---|",
    ]
    for name, neon_stats, python_stats, ratio in rows:
        table_lines.append(
            "| {name} | {nmean:.3f} ms | {nstd:.3f} ms | {nmin:.3f} ms | {nmed:.3f} ms | "
            "{pmean:.3f} ms | {pstd:.3f} ms | {pmin:.3f} ms | {pmed:.3f} ms | {ratio:.3f} |".format(
                name=name,
                nmean=neon_stats["mean"],
                nstd=neon_stats["stddev"],
                nmin=neon_stats["min"],
                nmed=neon_stats["median"],
                pmean=python_stats["mean"],
                pstd=python_stats["stddev"],
                pmin=python_stats["min"],
                pmed=python_stats["median"],
                ratio=ratio,
            )
        )
    table = "\n".join(table_lines)

    print(table)

    summary_path = os.environ.get("GITHUB_STEP_SUMMARY")
    if summary_path:
        with open(summary_path, "a") as f:
            f.write(table + "\n")

    with open(args.json, "w") as f:
        json.dump(json_entries, f, indent=2)

    if mismatches:
        for name, neon_checksum, python_checksum in mismatches:
            print(
                f"checksum mismatch for {name}: neon={neon_checksum!r} python={python_checksum!r}",
                file=sys.stderr,
            )
        sys.exit(1)


if __name__ == "__main__":
    main()
