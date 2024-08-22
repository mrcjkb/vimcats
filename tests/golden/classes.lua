local U = {}

---The Homosapien
---@class Human
---@field legs number Total number of legs
---@field hands number Total number of hands
---@field brain boolean Does humans have brain?

---@class SuperSecret
---@field first number First ingredient
---@field public second? number Second ingredient
---@field third number Third ingredient
---@field todo? number
---@field protected __secret_1 number Secret ingredient first
---@field private __secret_2? number

---Plugin's configuration
---@class CommentConfig
---@field padding boolean Add a space b/w comment and the line
---Whether the cursor should stay at its position
---NOTE: This only affects NORMAL mode mappings and doesn't work with dot-repeat
---
---@field sticky boolean
---Lines to be ignored while comment/uncomment.
---Could be a regex string or a function that returns a regex string.
---Example: Use '^$' to ignore empty lines
---
---@field ignore string|fun():string
---@field pre_hook fun(ctx:CommentCtx):string Function to be called before comment/uncomment
---@field post_hook fun(ctx:CommentCtx) Function to be called after comment/uncomment

---@class XMen : Homosapien
---@field power number Power quantifier
---@field [string] unknown Generic fields

---@class (exact) XactMen
---@field power number Power quantifier

return U
