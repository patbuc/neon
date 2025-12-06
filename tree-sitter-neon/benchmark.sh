#!/bin/bash
# Performance benchmarking for Tree-sitter parser

cd "$(dirname "$0")"

echo "==========================="
echo "Tree-sitter Parser Benchmark"
echo "==========================="
echo ""

# Create large test file
echo "Creating large test file..."
cat > benchmark_large.n <<'EOF'
// Large file for performance testing
EOF

for i in {1..1000}; do
  cat >> benchmark_large.n <<EOF
val x${i} = ${i}
fn func${i}(a, b) {
  return a + b + ${i}
}
print "Iteration ${i}: \${x${i}}"
EOF
done

echo "Generated file with $(wc -l < benchmark_large.n) lines"
echo ""

# Time the parsing
echo "Parsing benchmark..."
time npx tree-sitter parse benchmark_large.n --quiet

# Clean up
rm benchmark_large.n

echo ""
echo "Benchmark complete!"
