local U = {}

---Trigger a rebuild of one or more projects.
---@param opts table|nil optional configuration options:
---  * {select_mode} (JdtProjectSelectMode) Show prompt
---     to select projects or select all. Defaults
---     to 'prompt'
---
---  * {full_build} (boolean) full rebuild or
---     incremental build. Defaults to true (full build)
---@param reserverd table|nil reserved for the future use
---@return boolean
function U.multi_line(opts, reserverd)
    print(vim.inspect(opts), vim.inspect(reserverd))

    return true
end

---Multiline but missing description
---@param n number
---This is a special
---
---number
---@param m number
---And this is also
---
---@return number
function U.missing_desc(n, m)
    return n + m
end

return U
