# Benchmark Report

Generated on: Sat Oct 11 23:24:25 WAT 2025

## Baseline (v0.2.3)

| Test Case | Mean Time | Std Dev |
|-----------|-----------|---------|
| component with props | 1.27μs | ±.04μs |
| component with state | 5.03μs | ±.08μs |
| computed signals | 1.92μs | ±.11μs |
| effects | 544.09μs | ±191.78μs |
| list rendering large | 150.45μs | ±8.26μs |
| list rendering small | 7.62μs | ±.34μs |
| nested components | 4.50μs | ±.13μs |
| nested elements to string | 12.31μs | ±.35μs |
| signal creation | 1.72μs | ±.12μs |
| signal reads | 23.66μs | ±1.91μs |
| signal updates | 12.85μs | ±.39μs |
| simple component | 1.00μs | ±.05μs |
| simple element to string | 1.22μs | ±.02μs |

## Current Version

| Test Case | Mean Time | Std Dev |
|-----------|-----------|---------|
| component with props | 1.68μs | ±.02μs |
| component with state | 5.06μs | ±.07μs |
| computed signals current | 1.19μs | ±.02μs |
| effects current | 1.32ms | ±.17ms |
| list (100 items) to html | 51.27μs | ±.78μs |
| list (1000 items) to html | 558.52μs | ±7.01μs |
| list rendering large | 148.33μs | ±3.20μs |
| list rendering small | 7.71μs | ±.33μs |
| nested components | 5.78μs | ±.08μs |
| nested elements to string | 12.40μs | ±.20μs |
| signal creation current | 1.11μs | ±.02μs |
| signal reads current | 14.82μs | ±.44μs |
| signal updates current | 374.65μs | ±206.04μs |
| simple component to html | 352.63717327160884ns | ±3.711646119104236ns |
| simple component to string | 397.4124106015492ns | ±19.87557939986799ns |
| simple component | 957.8688539040097ns | ±36.53586137639381ns |
| simple element to string | 1.21μs | ±.02μs |

## Performance Comparison

| Test Case | Baseline | Current | Change | Status |
|-----------|----------|---------|--------|--------|
