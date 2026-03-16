#!/bin/bash
set -e

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_FILE="${WORKSPACE_ROOT}/report.md"
BASE_TARGET_DIR="${WORKSPACE_ROOT}/benches-base/target/criterion"
CURRENT_TARGET_DIR="${WORKSPACE_ROOT}/target/criterion"

python3 - <<'PY' "$BASE_TARGET_DIR" "$CURRENT_TARGET_DIR" "$REPORT_FILE"
import json
import sys
from datetime import datetime
from pathlib import Path

base_dir = Path(sys.argv[1])
current_dir = Path(sys.argv[2])
report_file = Path(sys.argv[3])


def collect_results(root: Path):
    results = {}
    if not root.exists():
        return results

    for estimates in sorted(root.glob('*/new/estimates.json')):
        name = estimates.parts[-3]
        data = json.loads(estimates.read_text())
        results[name] = {
            'mean': data['mean']['point_estimate'],
            'std_dev': data['std_dev']['point_estimate'],
        }
    return results


def format_time(value: float) -> str:
    if value >= 1_000_000_000:
        return f"{value / 1_000_000_000:.2f}s"
    if value >= 1_000_000:
        return f"{value / 1_000_000:.2f}ms"
    if value >= 1_000:
        return f"{value / 1_000:.2f}μs"
    return f"{value:.2f}ns"


def display_name(name: str) -> str:
    return name.replace('_', ' ')


def write_table(handle, title: str, results):
    if not results:
        return
    handle.write(f"## {title}\n\n")
    handle.write("| Test Case | Mean Time | Std Dev |\n")
    handle.write("|-----------|-----------|---------|\n")
    for name in sorted(results):
        handle.write(
            f"| {display_name(name)} | {format_time(results[name]['mean'])} | ±{format_time(results[name]['std_dev'])} |\n"
        )
    handle.write("\n")


base_results = collect_results(base_dir)
current_results = collect_results(current_dir)
common = sorted(set(base_results) & set(current_results))

with report_file.open('w') as handle:
    handle.write('# Benchmark Report\n\n')
    handle.write(f"Generated on: {datetime.now().astimezone().strftime('%a %b %d %H:%M:%S %Z %Y')}\n\n")

    write_table(handle, 'Baseline (v0.2.3)', base_results)
    write_table(handle, 'Current Version', current_results)

    if common:
        handle.write('## Performance Comparison\n\n')
        handle.write('| Test Case | Baseline | Current | Change | Status |\n')
        handle.write('|-----------|----------|---------|--------|--------|\n')
        for name in common:
            baseline = base_results[name]['mean']
            current = current_results[name]['mean']
            change = ((current - baseline) / baseline) * 100
            if change < -5:
                status = 'Faster'
                change_display = f"{abs(change):.2f}% faster"
            elif change > 5:
                status = 'Slower'
                change_display = f"{change:.2f}% slower"
            else:
                status = 'Similar'
                change_display = f"{change:.2f}%"
            handle.write(
                f"| {display_name(name)} | {format_time(baseline)} | {format_time(current)} | {change_display} | {status} |\n"
            )
        handle.write('\n')
PY

echo "✅ Report generated at: $REPORT_FILE"
