---@mod module.config Configuration

local Config = {}

---Get the config
---@return number
function Config:get()
    return 3.14
end

---@export Config
return setmetatable(Config, {
    __index = function(this, k)
        return this.state[k]
    end,
    __newindex = function(this, k, v)
        this.state[k] = v
    end,
})
