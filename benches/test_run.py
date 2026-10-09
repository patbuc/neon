import importlib.util
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


if __name__ == "__main__":
    unittest.main()
