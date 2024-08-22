use vimcats::{vimdoc::VimDoc, FromLuaCATS, Settings, VimCats};

#[cfg(test)]
macro_rules! vimcat {
    ($($src: expr),*) => {{
        let mut vimcats = VimCats::default();
        let s = Settings::default();
        $(
            vimcats.for_help($src, &s).unwrap();
        )*
        VimDoc::from_emmy(&vimcats, &s).to_string()
    }};
}

#[test]
fn multi_module() {
    let src = "
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
    ";

    let src2 = r#"
    ---@mod wo.desc

    local U = {}

    return U
    "#;

    assert_eq!(
        vimcat!(src, src2),
        "\
==============================================================================
Introduction                                                         *mod.intro*


We can have multiple `---@mod` tags so that we can have a block only for text.
This is for the cases where you want bunch of block only just for text
and does not contains any code.


==============================================================================
Human module                                                         *mod.Human*

Human                                                                    *Human*
    The Homosapien

    Fields: ~
        {legs}   (number)   Total number of legs
        {hands}  (number)   Total number of hands
        {brain}  (boolean)  Does humans have brain?


U.DEFAULT                                                            *U.DEFAULT*
    Default traits of a human

    Type: ~
        (Human)


U:create()                                                            *U:create*
    Creates a Human

    Returns: ~
        (Human)

    Usage: ~
>lua
        local H = require('Human')
        local human = H:create()

        print(getmetatable(human))
<


==============================================================================
                                                                       *wo.desc*

"
    )
}
