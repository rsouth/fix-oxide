use core::convert::Into;
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
    pub const fn num(&self) -> u16 {
        match *self {
            Tag::MsgType => 35,
            Tag::Text => 58,
            Tag::ClOrdId => 11,
        }
    }
}

impl From<u16> for Tag {
    fn from(_: u16) -> Self {
        Tag::Text
    }
}

impl Into<Tag> for &Field {
    fn into(self) -> Tag {
        match self {
            Field::String(t, _)
            | Field::Char(t, _)
            | Field::Int(t, _)
            | Field::TagNum(t, _)
            | Field::SeqNum(t, _) => t.clone(),
        }
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tag[{}]", self.num())
    }
}
