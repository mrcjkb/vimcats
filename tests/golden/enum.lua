---Experimental features that can be enabled
---@enum rocks.ExperimentalFeature
config.ExperimentalFeature = {
    --- Whether to install rocks stubs when using extensions
    --- like rocks-git.nvim or rocks-dev.nvim
    --- so that luarocks recognises them as dependencies
    ext_module_dependency_stubs = "ext_module_dependency_stubs",
    
    --- Some other experimental feature
    some_other_experimental_feature = "some_other_experimental_feature",
}

---@enum RustAnalyzerCmd
local RustAnalyzerCmd = {
  start = 'start',
  stop = 'stop',
  restart = 'restart',
  reload_settings = 'reloadSettings',
}

local U = {}

return U
