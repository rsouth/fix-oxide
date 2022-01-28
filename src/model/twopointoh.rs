use std::fmt::Formatter;

use itertools::Itertools;

use crate::model::field::FieldSet;

impl FieldSet {
    fn get_msg_type(&self) -> Result<MsgType, UnknownField> {
        self.iter()
            .find_or_first(|p| p.tag() == MsgType::tag())
            .map(|i| MsgType { fd: i.clone() })
            .ok_or(UnknownField {})
    }

    fn get_string_field(&self, tag: u16) -> Result<StringField, UnknownField> {
        let f = self.iter().find_or_first(|p| p.tag() == tag).unwrap();
        match f {
            Field::String(t, v) => Ok(StringField {
                tag: *t,
                fd: Field::String(*t, v.to_string()),
            }),
            _ => Err(UnknownField {}),
        }
    }
}

// --- Example of a concrete / standard field

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MsgType {
    pub fd: Field,
}

impl MsgType {
    #[must_use]
    pub const fn tag() -> u16 {
        35
    }

    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => "",
        }
    }
}

impl std::fmt::Display for MsgType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}

// ----- Example of a custom field

pub struct StringField {
    tag: u16,
    fd: Field,
}

impl StringField {
    const fn tag(&self) -> u16 {
        self.tag
    }

    fn value(&self) -> &str {
        match &self.fd {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for StringField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", self.tag(), self.value())
    }
}

// ----- Errors...

// unable to convert from i32 to FieldType
pub struct UnknownField;

impl std::fmt::Display for UnknownField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error message here")
    }
}

// ------- Field Types ???

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Field {
    Int(u16, i32),
    TagNum(u16, u128),
    // todo check
    SeqNum(u16, u128),
    String(u16, String),
    Char(u16, char),
}

// ----- test / usage

#[cfg(test)]
mod tests {
    use crate::model::field::FieldSet;
    use crate::model::message::Message;
    use crate::model::twopointoh::{Field, MsgType};

    #[test]
    fn it_works() {
        let f = Field::String(MsgType::tag(), "D".to_string());
        let mut fs = FieldSet::default();
        fs.set_field(f.clone());

        let expected = "35=D|";
        assert_eq!(expected, fs.get_msg_type().ok().unwrap().to_string());
        assert_eq!(expected, fs.get_string_field(35).ok().unwrap().to_string());

        // message.set_field(fs.get_msg_type().ok().unwrap().clone());
        // let msg_type = fs.get_msg_type().ok().unwrap().clone();
        let mut message = Message::default();
        message.set_field(f);
    }
}
