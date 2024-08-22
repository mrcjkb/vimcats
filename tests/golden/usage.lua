local U = {}

---Prints a message
---@param msg string Message
---@usage lua [[
---require('module.U').sum(10, 5)
---@usage ]]
function U.echo(msg)
    print(msg)
end

---Add a number to 10
---@param this number A number
---@usage `require('module.U').sum(5)`
function U.sum(this)
    print(10 + this)
end

return U
