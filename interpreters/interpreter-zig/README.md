crafting interpreters - zig

Adapted from the book based on the code from zig tokenizer.

Leverages some nice things from Zig like labeled switch, sentinel-terminated
slices, comptime initialized static string map for keywords.

build commands
```zig
zig build-exe interpreter.zig -femit-bin=build/interpreter-fast-strip -O ReleaseFast -fstrip && build/interpreter-fast-strip
```

```sh
objdump -dC build/<executable>
```

---

benchmarking

```sh
Benchmark 1 (557 runs): interpreter-zig/build/source_tokenizer-fast-strip interpreter-zig/example.lox
  measurement          mean ± σ            min … max           outliers         delta
  wall_time          8.93ms ±  427us    7.62ms … 15.3ms          9 ( 2%)        0%
  peak_rss           1.43MB ±    0      1.43MB … 1.43MB          0 ( 0%)        0%
  cpu_cycles         3.66M  ±  331K     2.82M  … 4.45M           0 ( 0%)        0%
  instructions       2.08M  ± 18.3      2.08M  … 2.08M           0 ( 0%)        0%
  cache_references   20.4K  ± 4.13K     6.50K  … 29.7K           8 ( 1%)        0%
  cache_misses       1.03   ± 9.31         0   …  207          120 (22%)        0%
  branch_misses      11.6K  ±  289      11.1K  … 12.5K          72 (13%)        0%
Benchmark 2 (630 runs): interpreter-rs/target/release/interpreter-rs interpreter-zig/example.lox
  measurement          mean ± σ            min … max           outliers         delta
  wall_time          7.87ms ± 1.94ms    1.94ms … 11.2ms         41 ( 7%)        ⚡- 11.8% ±  1.8%
  peak_rss           1.94MB ± 67.7KB    1.78MB … 2.10MB          0 ( 0%)        💩+ 35.7% ±  0.4%
  cpu_cycles         4.33M  ± 48.4K     4.20M  … 4.69M          10 ( 2%)        💩+ 18.4% ±  0.7%
  instructions       12.2M  ±  319      12.2M  … 12.2M           2 ( 0%)        💩+485.9% ±  0.0%
  cache_references   7.64K  ±  751      3.09K  … 10.7K         143 (23%)        ⚡- 62.5% ±  1.6%
  cache_misses       35.1   ±  306         0   … 5.73K          75 (12%)        💩+3305.1% ± 2470.0%
  branch_misses      9.27K  ±  840      8.01K  … 20.1K          35 ( 6%)        ⚡- 20.1% ±  0.6%
```

---

`strace` output for zig implementation

```
execve("interpreter-zig/build/source_tokenizer-fast-strip", ["interpreter-zig/build/source_tok"..., "interpreter-zig/example-short.lo"...], 0x7ffc2f3b8028 /* 28 vars */) = 0
arch_prctl(ARCH_SET_FS, 0x100b008)      = 0
prlimit64(0, RLIMIT_STACK, NULL, {rlim_cur=8192*1024, rlim_max=RLIM64_INFINITY}) = 0
prlimit64(0, RLIMIT_STACK, {rlim_cur=16384*1024, rlim_max=RLIM64_INFINITY}, NULL) = 0
rt_sigaction(SIGPIPE, {sa_handler=0x1004e40, sa_mask=[], sa_flags=SA_RESTORER, sa_restorer=0x10060a0}, NULL, 8) = 0
openat(AT_FDCWD, "interpreter-zig/example-short.lox", O_RDONLY|O_NOCTTY|O_CLOEXEC) = 3
mmap(..., 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = ...
mmap(..., 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = ...
mmap(..., 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = ...
mmap(..., 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = ...
mmap(..., 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = ...
munmap(..., 4096)            = 0
munmap(..., 4096)            = 0
read(3, "(){}+-,.;*\n\n", 5250)         = 12
read(3, "", 5238)                       = 0
munmap(..., 4096)            = 0
gettid()                                = 894547
write(2, "left_paren", 10left_paren)              = 10
write(2, "\n", 1
)                       = 1
write(2, "right_paren", 11right_paren)             = 11
write(2, "\n", 1
)                       = 1
...
exit_group(0)                           = ?
+++ exited with 0 +++
```

`strace` output for rust implementation

```
execve("interpreter-rs/target/release/interpreter-rs", ["interpreter-rs/target/release/in"..., "interpreter-zig/example-short.lo"...], ... /* 28 vars */) = 0
brk(NULL)                               = 0x55b38641a000
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f01a7a4f000
access("/etc/ld.so.preload", R_OK)      = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/etc/ld.so.cache", O_RDONLY|O_CLOEXEC) = 3
newfstatat(3, "", {st_mode=S_IFREG|0644, st_size=41074, ...}, AT_EMPTY_PATH) = 0
mmap(..., 41074, PROT_READ, MAP_PRIVATE, 3, 0) = ...
close(3)                                = 0
openat(AT_FDCWD, "/lib/x86_64-linux-gnu/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\0\0\0\0\0\0\0\0\0\3\0>\0\1\0\0\0\0\0\0\0\0\0\0\0"..., 832) = 832
newfstatat(3, "", {st_mode=S_IFREG|0644, st_size=125312, ...}, AT_EMPTY_PATH) = 0
mmap(NULL, 127688, PROT_READ, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7f01a7a24000
mmap(..., 94208, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x3000) = ...
mmap(..., 16384, PROT_READ, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x1a000) = ...
mmap(..., 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x1d000) = ...
close(3)                                = 0
openat(AT_FDCWD, "/lib/x86_64-linux-gnu/libc.so.6", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\3\0\0\0\0\0\0\0\0\3\0>\0\1\0\0\0\20t\2\0\0\0\0\0"..., 832) = 832
pread64(3, "\6\0\0\0\4\0\0\0@\0\0\0\0\0\0\0@\0\0\0\0\0\0\0@\0\0\0\0\0\0\0"..., 784, 64) = 784
newfstatat(3, "", {st_mode=S_IFREG|0755, st_size=1922136, ...}, AT_EMPTY_PATH) = 0
pread64(3, "\6\0\0\0\4\0\0\0@\0\0\0\0\0\0\0@\0\0\0\0\0\0\0@\0\0\0\0\0\0\0"..., 784, 64) = 784
mmap(NULL, 1970000, PROT_READ, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7f01a7843000
mmap(..., 1396736, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x26000) = ...
mmap(..., 339968, PROT_READ, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x17b000) = ...
mmap(..., 24576, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x1ce000) = ...
mmap(..., 53072, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = ...
close(3)                                = 0
mmap(NULL, 12288, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f01a7840000
arch_prctl(ARCH_SET_FS, 0x7f01a7840780) = 0
set_tid_address(...)         = 894409
set_robust_list(..., 24)     = 0
rseq(..., ..., 0, ...) = 0
mprotect(..., 16384, PROT_READ) = 0
mprotect(..., 4096, PROT_READ) = 0
mprotect(..., 12288, PROT_READ) = 0
mprotect(..., 8192, PROT_READ) = 0
prlimit64(0, RLIMIT_STACK, NULL, {rlim_cur=8192*1024, rlim_max=RLIM64_INFINITY}) = 0
munmap(..., 41074)           = 0
poll([{fd=0, events=0}, {fd=1, events=0}, {fd=2, events=0}], 3, 0) = 0 (Timeout)
rt_sigaction(SIGPIPE, {sa_handler=SIG_IGN, sa_mask=[PIPE], sa_flags=SA_RESTORER|SA_RESTART, sa_restorer=0x7f01a787f050}, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
getrandom("\x80\x68\xb0\x89\x32\xb7\xe3\x39", 8, GRND_NONBLOCK) = 8
brk(...)                               = ...
brk(...)                     = ...
openat(AT_FDCWD, "/proc/self/maps", O_RDONLY|O_CLOEXEC) = 3
prlimit64(0, RLIMIT_STACK, NULL, {rlim_cur=8192*1024, rlim_max=RLIM64_INFINITY}) = 0
newfstatat(3, "", {st_mode=S_IFREG|0444, st_size=0, ...}, AT_EMPTY_PATH) = 0
read(3, ..., 1024) = 1024
read(3, "   /usr/lib/x86_64-linux-gnu/lib"..., 1024) = 1024
read(3, "9884421                  /usr/li"..., 1024) = 955
close(3)                                = 0
sched_getaffinity(894409, 32, [0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23]) = 8
rt_sigaction(SIGSEGV, NULL, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
sigaltstack(NULL, {ss_sp=NULL, ss_flags=SS_DISABLE, ss_size=0}) = 0
mmap(NULL, 12288, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS|MAP_STACK, -1, 0) = 0x7f01a7a4c000
mprotect(..., 4096, PROT_NONE) ...
sigaltstack({ss_sp=..., ss_flags=0, ss_size=8192}, NULL) = 0
rt_sigaction(SIGSEGV, {sa_handler=..., sa_mask=[], sa_flags=SA_RESTORER|SA_ONSTACK|SA_SIGINFO, sa_restorer=...}, NULL, 8) = 0
rt_sigaction(SIGBUS, NULL, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
rt_sigaction(SIGBUS, {sa_handler=..., sa_mask=[], sa_flags=SA_RESTORER|SA_ONSTACK|SA_SIGINFO, sa_restorer=...}, NULL, 8) = 0
write(1, "zlox interpreter v0.0.1\n", 24zlox interpreter v0.0.1
) = 24
write(1, "Type your code below. Press Ctrl"..., 76Type your code below. Press Ctrl+D (Unix) or Ctrl+Z (Windows) to end input.
) = 76
openat(AT_FDCWD, "interpreter-zig/example-short.lox", O_RDONLY|O_CLOEXEC) = 3
statx(3, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_ALL, {stx_mask=STATX_ALL|STATX_MNT_ID, stx_attributes=0, stx_mode=S_IFREG|0644, stx_size=12, ...}) = 0
read(3, "(){}+-,.;*\n\n", 12)           = 12
read(3, "", 32)                         = 0
close(3)                                = 0
write(1, "LeftParen '('\n", 14LeftParen '('
)         = 14
write(1, "RightParen ')'\n", 15RightParen ')'
)        = 15
...
sigaltstack({ss_sp=NULL, ss_flags=SS_DISABLE, ss_size=8192}, NULL) = 0
munmap(..., 12288)           = 0
exit_group(0)                           = ?
+++ exited with 0 +++
```

```sh
Benchmark 1 (814 runs): interpreter-zig/build/source_tokenizer-fast-strip interpreter-zig/example.lox
  measurement          mean ± σ            min … max           outliers         delta
  wall_time          6.04ms ±  760us    1.48ms … 7.05ms         48 ( 6%)        0%
  peak_rss           1.43MB ±    0      1.43MB … 1.43MB          0 ( 0%)        0%
  cpu_cycles         1.27M  ± 30.0K     1.21M  … 1.35M           0 ( 0%)        0%
  instructions       2.64M  ± 17.8      2.64M  … 2.64M          33 ( 4%)        0%
  cache_references    357   ±  115       274   …  716          129 (16%)        0%
  cache_misses       0.96   ± 11.2         0   …  256          108 (13%)        0%
  branch_misses       240   ± 27.7       203   …  328          138 (17%)        0%
Benchmark 2 (634 runs): interpreter-rs/target/release/interpreter-rs interpreter-zig/example.lox
  measurement          mean ± σ            min … max           outliers         delta
  wall_time          7.83ms ± 1.53ms    2.10ms … 10.8ms         12 ( 2%)        💩+ 29.6% ±  2.0%
  peak_rss           1.94MB ± 71.6KB    1.79MB … 2.10MB          0 ( 0%)        💩+ 35.5% ±  0.3%
  cpu_cycles         4.33M  ± 54.1K     4.20M  … 4.85M          20 ( 3%)        💩+242.0% ±  0.3%
  instructions       12.2M  ±  253      12.2M  … 12.2M           2 ( 0%)        💩+361.6% ±  0.0%
  cache_references   7.49K  ±  651      3.23K  … 10.2K         122 (19%)        💩+1996.8% ± 12.8%
  cache_misses       17.2   ±  223         0   … 5.54K          58 ( 9%)        💩+1693.4% ± 1599.0%
  branch_misses      9.31K  ±  974      8.00K  … 20.7K          32 ( 5%)        💩+3783.2% ± 27.9%
```

no printing
```sh
Benchmark 1 (5718 runs): interpreter-zig/build/source_tokenizer-fast-strip interpreter-zig/example.lox
  measurement          mean ± σ            min … max           outliers         delta
  wall_time           820us ±  149us     515us … 1.39ms          1 ( 0%)        0%
  peak_rss           1.43MB ±    0      1.43MB … 1.43MB          0 ( 0%)        0%
  cpu_cycles         83.7K  ± 3.80K        0   … 94.2K          77 ( 1%)        0%
  instructions        212K  ± 3.97K        0   …  212K         290 ( 5%)        0%
  cache_references    168   ±  114         0   …  799            4 ( 0%)        0%
  cache_misses       0.25   ± 2.97         0   …  200          434 ( 8%)        0%
  branch_misses       110   ± 65.4         0   …  187            0 ( 0%)        0%
Benchmark 2 (2562 runs): interpreter-rs/target/release/interpreter-rs interpreter-zig/example.lox
  measurement          mean ± σ            min … max           outliers         delta
  wall_time          1.88ms ±  211us     361us … 2.45ms         32 ( 1%)        💩+128.7% ±  1.0%
  peak_rss           1.93MB ± 70.0KB    1.76MB … 2.10MB          0 ( 0%)        💩+ 35.1% ±  0.1%
  cpu_cycles          424K  ± 6.38K      405K  …  664K          48 ( 2%)        💩+406.1% ±  0.3%
  instructions        493K  ±  252       493K  …  494K          13 ( 1%)        💩+132.6% ±  0.1%
  cache_references   7.21K  ±  582      1.35K  … 10.4K         372 (15%)        💩+4196.2% ±  9.4%
  cache_misses       5.56   ±  118         0   … 5.94K         248 (10%)        💩+2165.7% ± 1242.5%
  branch_misses      3.70K  ± 43.1      3.59K  … 4.08K          47 ( 2%)        💩+3263.6% ±  2.5%
```

with vs. without size hint
```sh
Benchmark 1 (62 runs): interpreter-zig/build/source_tokenizer-fast-strip interpreter-zig/example-long.lox
  measurement          mean ± σ            min … max           outliers         delta
  wall_time          80.4ms ±  655us    79.5ms … 85.1ms          3 ( 5%)        0%
  peak_rss           72.5MB ± 1.63KB    72.5MB … 72.5MB         12 (19%)        0%
  cpu_cycles          370M  ± 1.19M      364M  …  371M           9 (15%)        0%
  instructions       1.18G  ± 0.71      1.18G  … 1.18G           2 ( 3%)        0%
  cache_references   1.13M  ±  143      1.13M  … 1.13M           4 ( 6%)        0%
  cache_misses        242K  ± 7.65K      229K  …  275K           3 ( 5%)        0%
  branch_misses      4.94K  ± 15.4      4.90K  … 4.98K           9 (15%)        0%
Benchmark 2 (49 runs): interpreter-zig/build/source_tokenizer-fast-strip-hint interpreter-zig/example-long.lox
  measurement          mean ± σ            min … max           outliers         delta
  wall_time           103ms ±  480us     101ms …  105ms          3 ( 6%)        💩+ 28.5% ±  0.3%
  peak_rss            119MB ±  622KB     118MB …  120MB          0 ( 0%)        💩+ 64.5% ±  0.2%
  cpu_cycles          418M  ± 1.36M      414M  …  422M           3 ( 6%)        💩+ 13.1% ±  0.1%
  instructions       1.19G  ±  397      1.19G  … 1.19G           2 ( 4%)        💩+  1.3% ±  0.0%
  cache_references   5.72M  ± 17.1K     5.69M  … 5.79M           1 ( 2%)        💩+405.1% ±  0.4%
  cache_misses       2.14M  ± 47.2K     2.09M  … 2.30M           6 (12%)        💩+785.1% ±  5.0%
  branch_misses      5.20K  ± 31.8      5.10K  … 5.26K           2 ( 4%)        💩+  5.3% ±  0.2%
```
