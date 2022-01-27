use crate::model::field::{FieldSet, NoSuchField};
use crate::model::tag::Tag;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum MsgType {
    Logon,
    Custom(String),
}

#[derive(Debug, Clone)]
struct UnknownMsgTypeError {
    val: String,
}

impl fmt::Display for UnknownMsgTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown MsgType value: {}", self.val)
    }
}

impl FromStr for MsgType {
    type Err = NoSuchField;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let msg_type = match s {
            "A" => MsgType::Logon,
            _ => MsgType::Custom(s.to_string()),
        };
        Ok(msg_type)
    }
}

impl Display for MsgType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MsgType::Logon => write!(f, "A"),
            MsgType::Custom(t) => write!(f, "{}", t),
        }
    }
}

impl TryFrom<&FieldSet> for MsgType {
    type Error = ();

    fn try_from(fs: &FieldSet) -> Result<Self, Self::Error> {
        match fs.get_field(Tag::MsgType) {
            Ok(a) => Ok(MsgType::from_str(a.string_value().unwrap()).unwrap()),
            Err(_) => Err(()),
        }
    }
}
