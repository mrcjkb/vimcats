use chumsky::{select, Parser};

use crate::{lexer::TagType, parser::impl_parse, Accept, Visitor};

use super::See;

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub values: Vec<EnumValue>,
    pub desc: Vec<String>,
    pub see: See,
}

#[derive(Debug, Clone)]
pub struct EnumValue {
    pub name: String,
    pub desc: Vec<String>,
}

impl_parse!(EnumValue, {
    select! { TagType::Comment(c) => c }
        .repeated()
        .then(select! { TagType::Variable(name) => name })
        .map(|(desc, name)| Self { name, desc })
});

impl_parse!(Enum, {
    select! { TagType::Comment(c) => c }
        .repeated()
        .then(select! { TagType::Enum(name) => name })
        .then(EnumValue::parse().repeated())
        .then(See::parse())
        .map(|(((desc, name), values), see)| Self {
            name,
            values,
            desc,
            see,
        })
});

impl<T: Visitor> Accept<T> for Enum {
    fn accept(&self, n: &T, s: &T::S) -> T::R {
        n.r#enum(self, s)
    }
}
