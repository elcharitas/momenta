#!/bin/bash
set -e

# Script to parse criterion benchmark results and generate report.md

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_FILE="${WORKSPACE_ROOT}/report.md"

# Criterion output directories
BASE_TARGET_DIR="${WORKSPACE_ROOT}/benches-base/target/criterion"
CURRENT_TARGET_DIR="${WORKSPACE_ROOT}/target/criterion"

echo "# Benchmark Report" > "$REPORT_FILE"
echo "" >> "$REPORT_FILE"
echo "Generated on: $(date)" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# Function to extract benchmark results from criterion JSON
extract_results() {
    local target_dir=$1
    local label=$2

    echo "## $label" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"

    if [ ! -d "$target_dir" ]; then
        echo "⚠️  No results found in $target_dir" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
        return
    fi

    # Find all benchmark groups
    for bench_dir in "$target_dir"/*; do
        if [ -d "$bench_dir" ] && [ "$(basename "$bench_dir")" != "report" ]; then
            bench_name=$(basename "$bench_dir")

            # Skip if it's a base directory
            if [ "$bench_name" = "base" ]; then
                continue
            fi

            echo "### $bench_name" >> "$REPORT_FILE"
            echo "" >> "$REPORT_FILE"

            # Look for estimates.json in all subdirectories
            find "$bench_dir" -name "new" -type d | while read -r group_dir; do
                estimates_file="$group_dir/estimates.json"
                if [ -f "$estimates_file" ]; then
                    # Get the relative path for the group name
                    group_name=$(dirname "$group_dir" | sed "s|$bench_dir/||" | sed 's|/base$||')

                    # Extract mean time from estimates.json
                    if command -v jq &> /dev/null; then
                        mean=$(jq -r '.mean.point_estimate' "$estimates_file")
                        std_dev=$(jq -r '.std_dev.point_estimate' "$estimates_file")

                        # Convert nanoseconds to appropriate unit
                        if (( $(echo "$mean >= 1000000000" | bc -l) )); then
                            mean_display=$(echo "scale=2; $mean / 1000000000" | bc)
                            unit="s"
                        elif (( $(echo "$mean >= 1000000" | bc -l) )); then
                            mean_display=$(echo "scale=2; $mean / 1000000" | bc)
                            unit="ms"
                        elif (( $(echo "$mean >= 1000" | bc -l) )); then
                            mean_display=$(echo "scale=2; $mean / 1000" | bc)
                            unit="μs"
                        else
                            mean_display=$(echo "scale=2; $mean" | bc)
                            unit="ns"
                        fi

                        echo "- **$group_name**: $mean_display $unit (±$(echo "scale=2; $std_dev / 1000000" | bc) ms)" >> "$REPORT_FILE"
                    else
                        echo "- **$group_name**: (jq not available for parsing)" >> "$REPORT_FILE"
                    fi
                fi
            done

            echo "" >> "$REPORT_FILE"
        fi
    done
}

# Extract results from both benchmark runs
extract_results "$BASE_TARGET_DIR" "Baseline (v0.2.3)"
extract_results "$CURRENT_TARGET_DIR" "Current Version"

# Add comparison section if both results exist
if [ -d "$BASE_TARGET_DIR" ] && [ -d "$CURRENT_TARGET_DIR" ]; then
    echo "## Performance Comparison" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    echo "Comparing current version against baseline (v0.2.3)." >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"

    # This would require more sophisticated parsing to calculate actual differences
    # For now, we'll add a placeholder
    echo "*Detailed comparison requires manual analysis of the above results.*" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
fi

echo "✅ Report generated at: $REPORT_FILE"
