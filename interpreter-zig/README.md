Useful flags
```zig
zig build-exe interpreter.zig -femit-bin=build/interpreter-fast-strip -O ReleaseFast -fstrip && build/interpreter-fast-strip
```
objdump -dC build/<executable>
