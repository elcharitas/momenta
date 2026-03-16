#!/bin/bash
set -e

# Script to run both baseline and current benchmarks, then generate a report

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SCRIPT_DIR="${WORKSPACE_ROOT}/scripts"

echo "🚀 Running Momenta Benchmark Suite"
echo "===================================="
echo ""

rm -rf "${WORKSPACE_ROOT}/benches-base/target/criterion" "${WORKSPACE_ROOT}/target/criterion"

# Run baseline benchmarks (v0.2.3)
echo "📊 Running baseline benchmarks (v0.2.3)..."
cd "${WORKSPACE_ROOT}/benches-base"
cargo bench --all-features
echo ""

# Run current version benchmarks
echo "📊 Running current version benchmarks..."
cd "${WORKSPACE_ROOT}/benches"
cargo bench --all-features
echo ""

# Generate the report
echo "📝 Generating benchmark report..."
cd "${WORKSPACE_ROOT}"
bash "${SCRIPT_DIR}/generate_report.sh"
echo ""

echo "✅ All benchmarks completed!"
echo "📄 Report available at: ${WORKSPACE_ROOT}/report.md"
