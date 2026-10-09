import importlib.util
import io
import os
import sys
import tempfile
import unittest
from unittest import mock

BENCHES_DIR = os.path.dirname(os.path.abspath(__file__))


def load_run():
    saved_path = list(sys.path)
    try:
        spec = importlib.util.spec_from_file_location(
            "bench_run", os.path.join(BENCHES_DIR, "run.py")
        )
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
    finally:
        sys.path[:] = saved_path
    return module


run = load_run()

CPUINFO = """processor\t: 0
vendor_id\t: GenuineIntel
model\t\t: 94
model name\t: Intel(R) Core(TM) i7-6700K CPU @ 4.00GHz
cpu MHz\t\t: 4000.000

processor\t: 1
vendor_id\t: GenuineIntel
model\t\t: 94
model name\t: Some Other CPU @ 1.00GHz
cpu MHz\t\t: 4000.000
"""


class CpuModelTest(unittest.TestCase):
    def test_first_model_name_line(self):
        with tempfile.NamedTemporaryFile("w", suffix=".cpuinfo") as f:
            f.write(CPUINFO)
            f.flush()
            self.assertEqual(
                run.cpu_model(f.name), "Intel(R) Core(TM) i7-6700K CPU @ 4.00GHz"
            )

    def test_processor_fallback(self):
        with mock.patch("platform.processor", return_value="x86_64-test"):
            self.assertEqual(
                run.cpu_model("/nonexistent/cpuinfo"), "x86_64-test"
            )

    def test_no_model_name_exits(self):
        with tempfile.NamedTemporaryFile("w", suffix=".cpuinfo") as f:
            f.write("processor\t: 0\nvendor_id\t: GenuineIntel\n")
            f.flush()
            with mock.patch("sys.stderr", new_callable=io.StringIO) as err:
                with self.assertRaises(SystemExit) as cm:
                    run.cpu_model(f.name)
        self.assertEqual(cm.exception.code, 1)
        self.assertIn("CPU model", err.getvalue())

    def test_empty_processor_exits(self):
        with mock.patch("platform.processor", return_value=""):
            with mock.patch("sys.stderr", new_callable=io.StringIO) as err:
                with self.assertRaises(SystemExit) as cm:
                    run.cpu_model("/nonexistent/cpuinfo")
        self.assertEqual(cm.exception.code, 1)
        self.assertIn("CPU model", err.getvalue())


class MainOutputTest(unittest.TestCase):
    def test_cpu_line_before_table(self):
        with tempfile.TemporaryDirectory() as tmp:
            argv = ["run.py", "fib", "--runs", "1", "--json", os.path.join(tmp, "out.json")]
            env = {k: v for k, v in os.environ.items() if k != "GITHUB_STEP_SUMMARY"}
            with mock.patch("sys.argv", argv), \
                    mock.patch.dict(os.environ, env, clear=True), \
                    mock.patch.object(run.os.path, "isfile", return_value=True), \
                    mock.patch.object(run, "run_once", return_value=(0.01, "42")), \
                    mock.patch.object(run, "cpu_model", return_value="Test CPU @ 1GHz"), \
                    mock.patch("sys.stdout", new_callable=io.StringIO) as out:
                run.main()
        lines = out.getvalue().splitlines()
        table_start = next(i for i, line in enumerate(lines) if line.startswith("| Benchmark"))
        self.assertGreater(table_start, 0, "no line before the table")
        self.assertEqual(lines[table_start - 1], "CPU: Test CPU @ 1GHz")

    def test_cpu_line_before_table_in_step_summary(self):
        with tempfile.TemporaryDirectory() as tmp:
            summary = os.path.join(tmp, "summary.md")
            argv = ["run.py", "fib", "--runs", "1", "--json", os.path.join(tmp, "out.json")]
            env = {**os.environ, "GITHUB_STEP_SUMMARY": summary}
            with mock.patch("sys.argv", argv), \
                    mock.patch.dict(os.environ, env, clear=True), \
                    mock.patch.object(run.os.path, "isfile", return_value=True), \
                    mock.patch.object(run, "run_once", return_value=(0.01, "42")), \
                    mock.patch.object(run, "cpu_model", return_value="Test CPU @ 1GHz"), \
                    mock.patch("sys.stdout", new_callable=io.StringIO):
                run.main()
            with open(summary) as f:
                lines = f.read().splitlines()
        table_start = next(i for i, line in enumerate(lines) if line.startswith("| Benchmark"))
        self.assertGreater(table_start, 0, "no line before the table")
        self.assertEqual(lines[table_start - 1], "CPU: Test CPU @ 1GHz")


if __name__ == "__main__":
    unittest.main()
