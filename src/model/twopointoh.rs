use std::fmt::Formatter;

use itertools::Itertools;

use crate::model::field::{Field, FieldSet};
use crate::model::tag::Tag;

impl FieldSet {
    fn get_msg_type(&self) -> Result<StringField, UnknownField> {
        let f = self
            .iter()
            .find_or_first(|p| p.tag() == Tag::MsgType)
            .unwrap();
        match f {
            Field::String(t, _) => Ok(StringField {
                tag: t.num(),
                fd: f.clone(),
            }),
            _ => Err(UnknownField {}),
        }
    }

    fn get_string_field(&self, tag: u16) -> Result<StringField, UnknownField> {
        let f = self.iter().find_or_first(|p| p.tag().num() == tag).unwrap();
        match f {
            Field::String(t, v) => Ok(StringField {
                tag: t.num(),
                fd: Field::String(*t, v.to_string()),
            }),
            _ => Err(UnknownField {}),
        }
    }
}

// --- Example of a concrete / standard field

pub struct MsgType {
    fd: Field,
}

impl MsgType {
    const fn tag() -> u16 {
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

// ----- test / usage

#[cfg(test)]
mod tests {
    use crate::model::field::FieldSet;
    use crate::model::tag::Tag;
    use crate::model::twopointoh::Field;

    #[test]
    fn it_works() {
        let f = Field::String(Tag::MsgType, "D".to_string());
        let mut fs = FieldSet::default();
        fs.set_field(f);

        let expected = "35=D|";
        assert_eq!(expected, fs.get_msg_type().ok().unwrap().to_string());
        assert_eq!(expected, fs.get_string_field(35).ok().unwrap().to_string());
    }
}
