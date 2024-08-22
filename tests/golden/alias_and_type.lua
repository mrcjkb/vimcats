local U = {}

---@alias NoDesc string

---All the lines in the buffer
---@alias Lines string[]

---Vim operator-mode motions.
---
---Read `:h map-operator`
---@alias VMode
---| '"line"' # Vertical motion
---| '"char"' # Horizontal motion
---| 'v'
---| `some.ident` # Some identifier

---Returns all the content of the buffer
---@return Lines
function U.get_all()
    return vim.api.nvim_buf_get_lines(0, 0, -1, false)
end

---List of all the lines in the buffer
---It can be more than one
---@type Lines lines in a buffer
---@see Lines
U.LINES = {}

---Global vim mode
---@type VMode
---@usage `print(require('plugin').VMODE)`
U.VMODE = 'line'

return U
