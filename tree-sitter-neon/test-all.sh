#!/bin/bash
# Comprehensive testing script for Neon Tree-sitter grammar

set -e

cd "$(dirname "$0")"

echo "================================"
echo "Neon Tree-sitter Grammar Tests"
echo "================================"
echo ""

# Test 1: Grammar generation
echo "[1/5] Testing grammar generation..."
npx tree-sitter generate > /dev/null 2>&1
echo "✓ Grammar generates successfully"
echo ""

# Test 2: Corpus tests
echo "[2/5] Running test corpus..."
npx tree-sitter test
echo ""

# Test 3: Parse all example files
echo "[3/5] Parsing example files..."
success=0
total=0
shopt -s nullglob
for file in ../examples/*.n ../examples/*.neon; do
  if [ -f "$file" ]; then
    total=$((total + 1))
    if npx tree-sitter parse "$file" --quiet 2>/dev/null; then
      success=$((success + 1))
      echo "✓ $(basename "$file")"
    else
      echo "✗ $(basename "$file")"
    fi
  fi
done
echo "Examples: $success/$total passed"
echo ""

# Test 4: Parse all test files
echo "[4/5] Parsing test scripts..."
success=0
total=0
for file in ../tests/scripts/*.n; do
  if [ -f "$file" ]; then
    total=$((total + 1))
    if npx tree-sitter parse "$file" --quiet 2>/dev/null; then
      success=$((success + 1))
    fi
  fi
done
echo "Test scripts: $success/$total passed"
echo ""

# Test 5: Highlight test
echo "[5/5] Testing syntax highlighting..."
npx tree-sitter highlight test_comprehensive.n > /dev/null 2>&1
echo "✓ Highlighting works"
echo ""

echo "================================"
echo "All tests completed!"
echo "================================"
