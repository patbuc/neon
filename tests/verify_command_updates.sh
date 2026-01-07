#!/bin/bash
# Tests that design.md and task.md contain the 'bd sync' instruction

FAIL=0

echo "Checking .opencode/command/design.md..."
if ! grep -q "bd sync" .opencode/command/design.md; then
  echo "FAIL: 'bd sync' not found in .opencode/command/design.md"
  FAIL=1
else
  echo "PASS: 'bd sync' found in .opencode/command/design.md"
fi

echo "Checking .opencode/command/task.md..."
if ! grep -q "bd sync" .opencode/command/task.md; then
  echo "FAIL: 'bd sync' not found in .opencode/command/task.md"
  FAIL=1
else
  echo "PASS: 'bd sync' found in .opencode/command/task.md"
fi

if [ $FAIL -eq 1 ]; then
  exit 1
fi

echo "All checks passed!"
exit 0
