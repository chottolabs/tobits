some findings:
- noticed that rust implements it as unchecked, even `.count()` is just counting the non-continuation bytes (didn't realize this initially and the checked init adds 30ms to zig implementation)

```sh
Benchmark 1 (69 runs): unicode/unicode-zig/build/uni big.txt
  measurement          mean ± σ            min … max           outliers         delta
  wall_time          72.4ms ±  905us    68.1ms … 77.8ms          5 ( 7%)        0%
  peak_rss            155MB ± 1.97KB     155MB …  155MB          0 ( 0%)        0%
  cpu_cycles          262M  ± 2.78M      240M  …  263M           3 ( 4%)        0%
  instructions       1.55G  ± 0.52      1.55G  … 1.55G           0 ( 0%)        0%
  cache_references   2.43M  ±  128      2.43M  … 2.43M           7 (10%)        0%
  cache_misses        776K  ± 45.2K      744K  … 1.13M           5 ( 7%)        0%
  branch_misses       602K  ± 3.81K      587K  …  622K           7 (10%)        0%
Benchmark 2 (65 runs): unicode/unicode-rs/target/release/unicode-rs big.txt
  measurement          mean ± σ            min … max           outliers         delta
  wall_time          78.0ms ±  902us    75.4ms … 81.3ms          4 ( 6%)        💩+  7.7% ±  0.4%
  peak_rss            157MB ± 93.8KB     157MB …  157MB          0 ( 0%)        💩+  1.0% ±  0.0%
  cpu_cycles          285M  ± 4.69M      272M  …  303M           4 ( 6%)        💩+  8.7% ±  0.5%
  instructions       1.36G  ±  258      1.36G  … 1.36G           0 ( 0%)        ⚡- 12.0% ±  0.0%
  cache_references   4.87M  ±  537      4.87M  … 4.87M           0 ( 0%)        💩+100.4% ±  0.0%
  cache_misses       2.99M  ± 74.7K     2.79M  … 3.20M           5 ( 8%)        💩+285.0% ±  2.7%
  branch_misses      1.71M  ± 8.12K     1.69M  … 1.73M           0 ( 0%)        💩+184.3% ±  0.4%
```
