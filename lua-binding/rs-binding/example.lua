---@class kznllm
---@field hello fun(message: string): nil
---Prints hello world

---@type kznllm
local kznllm_c = require("kznllm_c")

kznllm_c.hello("world")
