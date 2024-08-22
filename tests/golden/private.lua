local U = {}

---@private
---This is a private function which is exported
---But not considered as part of the API
function U.private()
    print('I am private!')
end

---Only this will be documented
function U.ok()
    print('Ok! I am exported')
end

---@protected
function U.no_emmy()
    print('Protected func with no emmylua!')
end

return U
