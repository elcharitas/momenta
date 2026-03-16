# Benchmark Report

Generated on: Mon Mar 16 08:48:04 WAT 2026

## Baseline (v0.2.3)

| Test Case | Mean Time | Std Dev |
|-----------|-----------|---------|
| component with props | 1.46μs | ±238.34ns |
| component with state | 7.35μs | ±3.94μs |
| computed signals | 2.29μs | ±340.46ns |
| effects | 585.07μs | ±256.31μs |
| list rendering large | 232.43μs | ±85.90μs |
| list rendering small | 16.55μs | ±10.02μs |
| nested components | 5.38μs | ±1.02μs |
| nested elements to string | 17.82μs | ±8.06μs |
| signal creation | 1.95μs | ±78.77ns |
| signal reads | 29.81μs | ±1.56μs |
| signal updates | 18.30μs | ±1.22μs |
| simple component | 1.12μs | ±87.29ns |
| simple element to string | 2.93μs | ±1.29μs |

## Current Version

| Test Case | Mean Time | Std Dev |
|-----------|-----------|---------|
| component with props | 1.45μs | ±166.34ns |
| component with state | 5.77μs | ±1.17μs |
| computed signals | 2.11μs | ±161.98ns |
| effects | 2.70μs | ±591.24ns |
| list rendering large | 190.18μs | ±88.43μs |
| list rendering small | 8.58μs | ±1.34μs |
| nested component syntax | 8.88μs | ±1.50μs |
| nested components | 5.46μs | ±1.54μs |
| nested elements to string | 14.23μs | ±2.75μs |
| signal creation | 2.15μs | ±565.23ns |
| signal reads | 5.89μs | ±394.43ns |
| signal updates | 7.74μs | ±2.54μs |
| simple component | 1.13μs | ±57.53ns |
| simple element to string | 1.31μs | ±240.22ns |

## Performance Comparison

| Test Case | Baseline | Current | Change | Status |
|-----------|----------|---------|--------|--------|
| component with props | 1.46μs | 1.45μs | -0.46% | Similar |
| component with state | 7.35μs | 5.77μs | 21.52% faster | Faster |
| computed signals | 2.29μs | 2.11μs | 7.68% faster | Faster |
| effects | 585.07μs | 2.70μs | 99.54% faster | Faster |
| list rendering large | 232.43μs | 190.18μs | 18.18% faster | Faster |
| list rendering small | 16.55μs | 8.58μs | 48.14% faster | Faster |
| nested components | 5.38μs | 5.46μs | 1.33% | Similar |
| nested elements to string | 17.82μs | 14.23μs | 20.15% faster | Faster |
| signal creation | 1.95μs | 2.15μs | 10.34% slower | Slower |
| signal reads | 29.81μs | 5.89μs | 80.24% faster | Faster |
| signal updates | 18.30μs | 7.74μs | 57.70% faster | Faster |
| simple component | 1.12μs | 1.13μs | 0.63% | Similar |
| simple element to string | 2.93μs | 1.31μs | 55.32% faster | Faster |

