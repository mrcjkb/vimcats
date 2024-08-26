---@class lz.n.KeysBase: vim.keymap.set.Opts
---@field desc? string
---@field noremap? boolean
---@field remap? boolean
---@field expr? boolean
---@field nowait? boolean
---@field ft? string|string[]

---@class lz.n.KeysSpec: lz.n.KeysBase
---@field [1] string lhs
---@field [2]? string|fun()|false rhs
---@field mode? string|string[]

local U = {}
return U
