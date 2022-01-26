use core::fmt;

use crate::model::field::Field;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Tag {
    MsgType,
    Text,
    ClOrdId,
}

impl Tag {
    #[must_use]
    pub const fn num(&self) -> u16 {
        match *self {
            Tag::MsgType => 35,
            Tag::Text => 58,
            Tag::ClOrdId => 11,
        }
    }
}

impl From<u16> for Tag {
    fn from(input: u16) -> Self {
        match input {
            11 => Self::ClOrdId,
            35 => Self::MsgType,
            58 => Self::Text,
            _ => Self::Text, // change this to try_from ??? and Err here
        }
    }
}

impl From<&Field> for Tag {
    fn from(value: &Field) -> Self {
        match value {
            Field::String(t, _)
            | Field::Char(t, _)
            | Field::Int(t, _)
            | Field::TagNum(t, _)
            | Field::SeqNum(t, _) => *t,
        }
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tag[{}]", self.num())
    }
}
