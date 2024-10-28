use mlua::prelude::*;

fn say_hello(_: &Lua, name: String) -> LuaResult<()> {
    println!("Hello, {}!", name);
    Ok(())
}

#[mlua::lua_module]
fn kznllm_rs(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("hello", lua.create_function(say_hello)?)?;
    Ok(exports)
}
