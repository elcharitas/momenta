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

# Temporary files for storing results
BASE_RESULTS=$(mktemp)
CURRENT_RESULTS=$(mktemp)

# Function to extract benchmark results from criterion JSON
extract_results() {
    local target_dir=$1
    local output_file=$2

    if [ ! -d "$target_dir" ]; then
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

            # Look for estimates.json in all subdirectories
            find "$bench_dir" -name "new" -type d | while read -r group_dir; do
                estimates_file="$group_dir/estimates.json"
                if [ -f "$estimates_file" ]; then
                    # Get the relative path for the group name
                    group_name=$(dirname "$group_dir" | sed "s|$bench_dir/||" | sed 's|/base$||')

                    # Clean up the benchmark name
                    clean_name=$(echo "$bench_name" | sed 's/_/ /g')

                    # Extract mean time from estimates.json
                    if command -v jq &> /dev/null; then
                        mean=$(jq -r '.mean.point_estimate' "$estimates_file")
                        std_dev=$(jq -r '.std_dev.point_estimate' "$estimates_file")

                        # Store raw nanoseconds value for comparison
                        echo "$clean_name|$mean|$std_dev" >> "$output_file"
                    fi
                fi
            done
        fi
    done
}

# Function to format time with appropriate unit
format_time() {
    local mean=$1
    local std_dev=$2

    if (( $(echo "$mean >= 1000000000" | bc -l) )); then
        mean_display=$(echo "scale=2; $mean / 1000000000" | bc)
        std_display=$(echo "scale=2; $std_dev / 1000000000" | bc)
        echo "${mean_display}s ± ${std_display}s"
    elif (( $(echo "$mean >= 1000000" | bc -l) )); then
        mean_display=$(echo "scale=2; $mean / 1000000" | bc)
        std_display=$(echo "scale=2; $std_dev / 1000000" | bc)
        echo "${mean_display}ms ± ${std_display}ms"
    elif (( $(echo "$mean >= 1000" | bc -l) )); then
        mean_display=$(echo "scale=2; $mean / 1000" | bc)
        std_display=$(echo "scale=2; $std_dev / 1000" | bc)
        echo "${mean_display}μs ± ${std_display}μs"
    else
        mean_display=$(echo "scale=2; $mean" | bc)
        std_display=$(echo "scale=2; $std_dev" | bc)
        echo "${mean_display}ns ± ${std_display}ns"
    fi
}

# Extract results from both benchmark runs
extract_results "$BASE_TARGET_DIR" "$BASE_RESULTS"
extract_results "$CURRENT_TARGET_DIR" "$CURRENT_RESULTS"

# Generate baseline table if data exists
if [ -s "$BASE_RESULTS" ]; then
    echo "## Baseline (v0.2.3)" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    echo "| Test Case | Mean Time | Std Dev |" >> "$REPORT_FILE"
    echo "|-----------|-----------|---------|" >> "$REPORT_FILE"

    while IFS='|' read -r name mean std_dev; do
        formatted=$(format_time "$mean" "$std_dev")
        # Split formatted result to extract mean and std dev
        mean_part=$(echo "$formatted" | cut -d'±' -f1 | xargs)
        std_part=$(echo "$formatted" | cut -d'±' -f2 | xargs)
        echo "| $name | $mean_part | ±$std_part |" >> "$REPORT_FILE"
    done < "$BASE_RESULTS"

    echo "" >> "$REPORT_FILE"
fi

# Generate current version table if data exists
if [ -s "$CURRENT_RESULTS" ]; then
    echo "## Current Version" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    echo "| Test Case | Mean Time | Std Dev |" >> "$REPORT_FILE"
    echo "|-----------|-----------|---------|" >> "$REPORT_FILE"

    while IFS='|' read -r name mean std_dev; do
        formatted=$(format_time "$mean" "$std_dev")
        mean_part=$(echo "$formatted" | cut -d'±' -f1 | xargs)
        std_part=$(echo "$formatted" | cut -d'±' -f2 | xargs)
        echo "| $name | $mean_part | ±$std_part |" >> "$REPORT_FILE"
    done < "$CURRENT_RESULTS"

    echo "" >> "$REPORT_FILE"
fi

# Add comparison section if both results exist
if [ -s "$BASE_RESULTS" ] && [ -s "$CURRENT_RESULTS" ]; then
    echo "## Performance Comparison" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    echo "| Test Case | Baseline | Current | Change | Status |" >> "$REPORT_FILE"
    echo "|-----------|----------|---------|--------|--------|" >> "$REPORT_FILE"

    # Create associative arrays for lookup
    declare -A base_times
    declare -A current_times

    while IFS='|' read -r name mean std_dev; do
        base_times["$name"]="$mean"
    done < "$BASE_RESULTS"

    while IFS='|' read -r name mean std_dev; do
        current_times["$name"]="$mean"
    done < "$CURRENT_RESULTS"

    # Compare matching benchmarks
    for name in "${!base_times[@]}"; do
        if [ -n "${current_times[$name]}" ]; then
            base_val="${base_times[$name]}"
            current_val="${current_times[$name]}"

            # Calculate percentage change
            change=$(echo "scale=2; (($current_val - $base_val) / $base_val) * 100" | bc)

            # Format times
            base_formatted=$(format_time "$base_val" "0" | cut -d'±' -f1 | xargs)
            current_formatted=$(format_time "$current_val" "0" | cut -d'±' -f1 | xargs)

            # Determine status emoji
            if (( $(echo "$change < -5" | bc -l) )); then
                status="🚀 Faster"
                change_display="${change#-}% faster"
            elif (( $(echo "$change > 5" | bc -l) )); then
                status="⚠️ Slower"
                change_display="${change}% slower"
            else
                status="➖ Similar"
                change_display="${change}%"
            fi

            echo "| $name | $base_formatted | $current_formatted | $change_display | $status |" >> "$REPORT_FILE"
        fi
    done

    echo "" >> "$REPORT_FILE"
fi

# Cleanup temporary files
rm -f "$BASE_RESULTS" "$CURRENT_RESULTS"

echo "✅ Report generated at: $REPORT_FILE"
