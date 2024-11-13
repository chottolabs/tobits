// mymodule.c
#include <lua.h>
#include <lauxlib.h>
#include <lualib.h>

static int say_hello(lua_State *L) {
    const char *name = luaL_checkstring(L, 1);
    printf("Hello, %s!\n", name);
    return 0;
}

static const struct luaL_Reg mylib[] = {
    {"say_hello", say_hello},
    {NULL, NULL}
};

// This name is important - it should be luaopen_[modulename]
int luaopen_kznllm_c(lua_State *L) {
    luaL_newlib(L, mylib);
    return 1;
}
