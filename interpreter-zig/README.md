crafting interpreters - zig

Adapted from the book based on the code from zig tokenizer.

Leverages some nice things from Zig like labeled switch, sentinel-terminated
slices, comptime initialized static string map for keywords.

Useful flags
```zig
zig build-exe interpreter.zig -femit-bin=build/interpreter-fast-strip -O ReleaseFast -fstrip && build/interpreter-fast-strip
```

```sh
objdump -dC build/<executable>
```
