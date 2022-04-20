use std::fmt::Display;

use chumsky::{prelude::just, select, Parser};

use crate::{impl_parse, TagType};

/// ---@brief [[ TEXT @brief ]]
#[derive(Debug, Clone)]
pub struct Brief {
    pub desc: Vec<String>,
}

impl_parse!(Brief, {
    select! {
        TagType::Comment(x) => x,
        TagType::Empty => "".into()
    }
    .repeated()
    .delimited_by(just(TagType::BriefStart), just(TagType::BriefEnd))
    .map(|desc| Self { desc })
});

impl Display for Brief {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: add formatting
        writeln!(f, "{}", self.desc.join("\n"))
    }
}