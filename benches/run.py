#!/usr/bin/env python3
"""Benchmark driver: runs paired Neon/Python benchmarks and compares timings."""

import sys

# benches/collections.py would otherwise shadow the stdlib "collections"
# module for every import below, since Python puts this script's directory
# first on sys.path.
if not getattr(sys.flags, "safe_path", False):
    del sys.path[0]

import argparse
import json
import os
import platform
import statistics
import subprocess
import time

BENCHMARKS = {
    "fib": 31,
    "loop_arith": 1500000,
    "closures": 80000,
    "structs": 80000,
    "nbody": 15000,
    "binary_trees": 13,
    "sieve": 1000000,
    "collections": 160000,
    "strings": 140000,
}

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
NEON_BIN = os.path.join(REPO_ROOT, "target", "release", "neon")


def positive_int(value):
    ivalue = int(value)
    if ivalue < 1:
        raise argparse.ArgumentTypeError(f"--runs must be >= 1, got {ivalue}")
    return ivalue


def cpu_model(path):
    try:
        f = open(path)
    except FileNotFoundError:
        model = platform.processor()
    else:
        model = None
        with f:
            for line in f:
                key, _, value = line.partition(":")
                if key.strip() == "model name":
                    model = value.strip()
                    break
    if not model:
        print("could not determine the CPU model", file=sys.stderr)
        sys.exit(1)
    return model


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
    _, warmup_checksum = run_once(cmd)
    checksums = [warmup_checksum]
    durations = []
    for _ in range(runs):
        elapsed, output = run_once(cmd)
        durations.append(elapsed * 1000)
        checksums.append(output)
    return durations, checksums


def stats(durations):
    return {
        "mean": statistics.mean(durations),
        "stddev": statistics.stdev(durations) if len(durations) > 1 else 0.0,
        "min": min(durations),
        "median": statistics.median(durations),
    }


def stats_cells(s):
    return f"{s['mean']:.3f} ms | {s['stddev']:.3f} ms | {s['min']:.3f} ms | {s['median']:.3f} ms"


def json_entry(name, unit, value, stddev=None):
    entry = {"name": name, "unit": unit, "value": value}
    if stddev is not None:
        entry["range"] = f"± {stddev:.3f}"
    return entry


def main():
    parser = argparse.ArgumentParser(description="Run Neon vs Python benchmarks")
    parser.add_argument("names", nargs="*", help="benchmark names to run (default: all)")
    parser.add_argument("--runs", type=positive_int, default=5, help="timed runs per benchmark (default: 5)")
    parser.add_argument("--json", default="bench-results.json", help="path to write results JSON")
    args = parser.parse_args()

    if args.names:
        deduped = list(dict.fromkeys(args.names))
        unknown = [n for n in deduped if n not in BENCHMARKS]
        if unknown:
            print(f"unknown benchmark(s): {', '.join(unknown)}", file=sys.stderr)
            sys.exit(1)
        selected = deduped
    else:
        selected = list(BENCHMARKS.keys())

    if not os.path.isfile(NEON_BIN):
        print(f"missing {NEON_BIN} — run `cargo build --release` first", file=sys.stderr)
        sys.exit(1)

    failures = []
    rows = []
    json_entries = []

    for name in selected:
        size = BENCHMARKS[name]
        neon_script = os.path.join(REPO_ROOT, "benches", f"{name}.n")
        python_script = os.path.join(REPO_ROOT, "benches", f"{name}.py")

        neon_durations, neon_checksums = time_command([NEON_BIN, neon_script, str(size)], args.runs)
        neon_checksum = neon_checksums[-1]
        python_durations, python_checksums = time_command(
            [sys.executable, python_script, str(size)], args.runs
        )
        python_checksum = python_checksums[-1]

        if len(set(neon_checksums)) > 1:
            failures.append(f"inconsistent neon checksums for {name}: {sorted(set(neon_checksums))!r}")
        if len(set(python_checksums)) > 1:
            failures.append(f"inconsistent python checksums for {name}: {sorted(set(python_checksums))!r}")
        if neon_checksum != python_checksum:
            failures.append(f"checksum mismatch for {name}: neon={neon_checksum!r} python={python_checksum!r}")

        neon_stats = stats(neon_durations)
        python_stats = stats(python_durations)
        ratio = neon_stats["mean"] / python_stats["mean"]

        rows.append((name, neon_stats, python_stats, ratio))

        json_entries.append(json_entry(f"{name} neon (ms)", "ms", neon_stats["mean"], neon_stats["stddev"]))
        json_entries.append(json_entry(f"{name} python (ms)", "ms", python_stats["mean"], python_stats["stddev"]))
        json_entries.append(json_entry(f"{name} neon/python", "ratio", ratio))

    table_lines = [
        "| Benchmark | Neon mean | Neon stddev | Neon min | Neon median | "
        "Python mean | Python stddev | Python min | Python median | Neon/Python |",
        "|---|---|---|---|---|---|---|---|---|---|",
    ]
    for name, neon_stats, python_stats, ratio in rows:
        table_lines.append(f"| {name} | {stats_cells(neon_stats)} | {stats_cells(python_stats)} | {ratio:.3f} |")
    table = "\n".join(table_lines)

    print(table)

    summary_path = os.environ.get("GITHUB_STEP_SUMMARY")
    if summary_path:
        with open(summary_path, "a") as f:
            f.write(table + "\n")

    if failures:
        for failure in failures:
            print(failure, file=sys.stderr)
        if summary_path:
            with open(summary_path, "a") as f:
                f.write("\n" + "\n".join(f"- {failure}" for failure in failures) + "\n")
        sys.exit(1)

    with open(args.json, "w") as f:
        json.dump(json_entries, f, indent=2)


if __name__ == "__main__":
    main()
