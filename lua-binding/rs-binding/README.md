```
cargo rustc --release -- -C link-arg=-undefined -C link-arg=dynamic_lookup
```

(on linux)
```
ln -s target/release/lib<module>.so <module>.so
```

(on mac)
```
ln -s target/release/lib<module>.dylib <module>.so
```

NOTE: this is the identifier you will use in lua (i.e. `require 'kznllm_c` in this case)
```
#[mlua::lua_module]
fn kznllm_c(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("hello", lua.create_function(hello)?)?;
    Ok(exports)
}
```
