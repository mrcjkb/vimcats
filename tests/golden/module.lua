---@mod mod.intro Introduction
---@brief [[
---
---We can have multiple `---@mod` tags so that we can have a block only for text.
---This is for the cases where you want bunch of block only just for text
---and does not contains any code.
---
---@brief ]]

---@mod mod.Human Human module

local U = {}

---The Homosapien
---@class Human
---@field legs number Total number of legs
---@field hands number Total number of hands
---@field brain boolean Does humans have brain?

---Default traits of a human
---@type Human
U.DEFAULT = {
    legs = 2,
    hands = 2,
    brain = false,
}

---Creates a Human
---@return Human
---@usage [[
---local H = require('Human')
---local human = H:create()
---
---print(getmetatable(human))
---@usage ]]
function U:create()
    return setmetatable(self.DEFAULT, { __index = self })
end

return U
