# Makefile
CC=cc
CFLAGS=-Wall -fPIC
LDFLAGS=

# The .so file name should match your module name
# On Linux/Unix it will be mymodule.so
# On macOS it will be mymodule.dylib
# On Windows it will be mymodule.dll
SHARED_LIB=kznllm_c.so

$(SHARED_LIB): mymodule.c
	$(CC) $(CFLAGS) -shared -o $@ $^ $(LDFLAGS)
	@echo === Running example.lua
	lua example.lua

clean:
	rm -f $(SHARED_LIB)
