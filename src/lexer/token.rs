use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Member {
    Literal(String),
    Ident(String),
}

impl Display for Member {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(lit) => f.write_str(&format!(
                r#""{}""#,
                lit.trim_start_matches('"').trim_end_matches('"')
            )),
            Self::Ident(ident) => f.write_str(ident),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TagType {
    /// ```lua
    /// ---@toc <name>
    /// ```
    Toc(String),
    /// ```lua
    /// ---@mod <name> [desc]
    /// ```
    Module(String, Option<String>),
    /// ```lua
    /// ---@divider <char>
    /// ```
    Divider(char),
    /// ```lua
    /// function one.two() end
    /// one.two = function() end
    /// ```
    Func(String, Op),
    /// ```lua
    /// one = 1
    /// one.two = 12
    /// ```
    Expr(String, Op),
    /// ```lua
    /// ---@export <module>
    /// or
    /// return <module>\eof
    /// ```
    Export(String),
    /// ```lua
    /// ---@brief [[
    /// ```
    BriefStart,
    /// ```lua
    /// ---@brief ]]
    /// ```
    BriefEnd,
    /// ```lua
    /// ---@param <name[?]> <type[|type...]> [description]
    /// ```
    Param(Name, Ty, Option<String>),
    /// ```lua
    /// ---@return <type> [<name> [comment] | [name] #<comment>]
    /// ```
    Return(Ty, Option<String>, Option<String>),
    /// ```lua
    /// ---@class <name>[: <parent>]
    /// ```
    Class(String, Option<String>),
    /// ```lua
    /// ---@field [public|private|protected] <name[?]> <type> [description]
    /// ```
    Field(Scope, Name, Ty, Option<String>),
    /// ```lua
    /// <variable> = <value>
    /// ```
    Variable(String),
    /// ```lua
    /// ---@enum <name>
    /// ```
    Enum(String),
    /// ```lua
    /// -- Simple Alias
    /// ---@alias <name> <type>
    ///
    /// -- Enum alias
    /// ---@alias <name>
    /// ```
    Alias(String, Option<Ty>),
    /// ```lua
    /// ---| '<literal>' [# description]
    ///
    /// -- or
    ///
    /// ---| `<ident>` [# description]
    /// ```
    Variant(Member, Option<String>),
    /// ```lua
    /// ---@type <type> [desc]
    /// ```
    Type(Ty, Option<String>),
    /// ```lua
    /// ---@tag <name>
    /// ```
    Tag(String),
    /// ```lua
    /// ---@see <name>
    /// ```
    See(String),
    /// ```lua
    /// ---@usage [lang] `<code>`
    /// ```
    Usage(Option<String>, String),
    /// ```lua
    /// ---@usage [lang] [[
    /// ```
    UsageStart(Option<String>),
    /// ```lua
    /// ---@usage ]]
    /// ```
    UsageEnd,
    /// ```lua
    /// ---TEXT
    /// ```
    Comment(String),
    /// Text nodes which are not needed
    Skip,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Op {
    Deep(Vec<Op>),
    Dot(String),
    Colon(String),
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Deep(mixed) => {
                for mix in mixed {
                    mix.fmt(f)?;
                }
                Ok(())
            }
            Self::Dot(dot) => {
                f.write_str(".")?;
                f.write_str(dot)
            }
            Self::Colon(colon) => {
                f.write_str(":")?;
                f.write_str(colon)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Scope {
    Public,
    Private,
    Protected,
    Package,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Name {
    Req(String),
    Opt(String),
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Req(n) => f.write_str(n),
            Self::Opt(n) => {
                f.write_str(n)?;
                f.write_str("?")
            }
        }
    }
}

// Source: https://github.com/sumneko/lua-language-server/wiki/Annotations#documenting-types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    Nil,
    Any,
    Unknown,
    Boolean,
    String,
    Number,
    Integer,
    Function,
    Thread,
    Userdata,
    Lightuserdata,
    Ref(String),
    Member(Member),
    Array(Box<Ty>),
    Table(Option<(Box<Ty>, Box<Ty>)>),
    Fun(Vec<(Name, Ty)>, Option<Vec<Ty>>),
    Dict(Vec<(Name, Ty)>),
    Union(Box<Ty>, Box<Ty>),
}

impl Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn list_like(args: &[(Name, Ty)]) -> String {
            args.iter()
                .map(|(n, t)| format!("{n}:{t}"))
                .collect::<Vec<String>>()
                .join(",")
        }

        match self {
            Self::Nil => f.write_str("nil"),
            Self::Any => f.write_str("any"),
            Self::Unknown => f.write_str("unknown"),
            Self::Boolean => f.write_str("boolean"),
            Self::String => f.write_str("string"),
            Self::Number => f.write_str("number"),
            Self::Integer => f.write_str("integer"),
            Self::Function => f.write_str("function"),
            Self::Thread => f.write_str("thread"),
            Self::Userdata => f.write_str("userdata"),
            Self::Lightuserdata => f.write_str("lightuserdata"),
            Self::Ref(id) => f.write_str(id),
            Self::Array(ty) => {
                f.write_str(&ty.to_string())?;
                f.write_str("[]")
            }
            Self::Table(kv) => match kv {
                Some((k, v)) => {
                    f.write_str("table<")?;
                    f.write_str(&k.to_string())?;
                    f.write_str(",")?;
                    f.write_str(&v.to_string())?;
                    f.write_str(">")
                }
                None => f.write_str("table"),
            },
            Self::Fun(args, ret) => {
                f.write_str("fun(")?;
                f.write_str(&list_like(args))?;
                f.write_str(")")?;
                if let Some(ret) = ret {
                    f.write_str(":")?;
                    f.write_str(
                        &ret.iter()
                            .map(|r| r.to_string())
                            .collect::<Vec<String>>()
                            .join(","),
                    )?;
                }
                Ok(())
            }
            Self::Dict(kv) => {
                f.write_str("{")?;
                f.write_str(&list_like(kv))?;
                f.write_str("}")
            }
            Self::Union(rhs, lhs) => {
                f.write_str(&rhs.to_string())?;
                f.write_str("|")?;
                f.write_str(&lhs.to_string())
            }
            Self::Member(mem) => mem.fmt(f),
        }
    }
}
