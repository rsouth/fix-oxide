use std::fmt::Formatter;
use std::str::FromStr;

use itertools::Itertools;
use rust_decimal::Decimal;

use crate::model::field::{FieldSet, FieldTypeMismatchError, MsgCat};

// ------- Field Types ???

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Field {
    Int(u16, i32),
    TagNum(u16, u128),
    SeqNum(u16, u128),
    String(u16, String),
    Char(u16, char),
    Price(u16, Decimal),
}

impl Field {
    ///
    /// # Errors
    ///
    pub fn as_str_safe(&self) -> Result<&str, FieldTypeMismatchError> {
        match self {
            Field::String(_, v) => Ok(v),
            _ => Err(FieldTypeMismatchError {}),
        }
    }

    ///
    /// # Errors
    ///
    /// # Panics
    ///
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Field::String(_, v) => v,
            _ => panic!(""),
        }
    }

    ///
    /// # Errors
    ///
    pub const fn as_i32_safe(&self) -> Result<i32, FieldTypeMismatchError> {
        match self {
            Field::Int(_, v) => Ok(*v),
            _ => Err(FieldTypeMismatchError {}),
        }
    }

    ///
    /// # Errors
    ///
    /// # Panics
    ///
    #[must_use]
    pub const fn as_i32(&self) -> i32 {
        match self {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }

    #[must_use]
    pub const fn tag(&self) -> u16 {
        match self {
            Field::Price(t, _)
            | Field::Int(t, _)
            | Field::TagNum(t, _)
            | Field::SeqNum(t, _)
            | Field::String(t, _)
            | Field::Char(t, _) => *t,
        }
    }

    #[must_use]
    pub fn to_delimited_string(&self, separator: char) -> String {
        match self {
            // &char
            Field::Char(t, v) => format!("{}={}{}", t, v, separator),
            // &i32
            Field::Int(t, v) => format!("{}={}{}", t, v, separator),
            // &String
            Field::String(t, v) => format!("{}={}{}", t, v, separator),
            // &u128
            Field::TagNum(t, v) | Field::SeqNum(t, v) => {
                format!("{}={}{}", t, v, separator)
            }
            // Decimal
            Field::Price(t, v) => format!("{}={}{}", t, v, separator),
        }
    }
}

// parse string (35=D) into Field{35, "D"}
impl TryFrom<String> for Field {
    type Error = (); // todo error type...

    fn try_from(s: String) -> Result<Self, Self::Error> {
        println!("From<String> for Field: {}", &s);
        let two_parts = s.split_once('=');
        match two_parts {
            Some((s_tag, s_value)) => {
                println!("parsing tag: {}, field: {} into Field", s_tag, s_value);

                // figure out the tag
                let tag: u16 = s_tag.parse::<u16>().unwrap();

                // build field using the tag & value
                match tag {
                    // todo generate this
                    58 | 35 => Ok(Self::String(tag, s_value.to_string())),
                    1 => Ok(Self::Int(tag, str::parse::<i32>(s_value).unwrap())),
                    5 => Ok(Self::Char(tag, str::parse::<char>(s_value).unwrap())),
                    54 => Ok(Self::Price(tag, Decimal::from_str(s_value).unwrap())),
                    _ => Err(()),
                }
            }
            None => Err(()),
        }
    }
}

// ------ FieldSet

// todo this whole jobbo will be generated
impl FieldSet {
    // todo generate get_clordid(&self) etc

    // todo generate get_int(&self, tag: u16) etc.

    /// for use by custom fields
    fn get_string_field(&self, tag: u16) -> Result<StringField, FieldTypeMismatchError> {
        let f = self.iter().find_or_first(|p| p.tag() == tag).unwrap();
        match f {
            Field::String(t, v) => Ok(StringField {
                tag: *t,
                fd: Field::String(*t, v.to_string()),
            }),
            _ => Err(FieldTypeMismatchError {}),
        }
    }
}

// --- Example of a concrete / standard field
// todo all types should be generated

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MsgTypeField {
    pub fd: Field,
}

impl MsgTypeField {
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

impl std::fmt::Display for MsgTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}

// ----- Example of a custom field
// todo should be generated for all field types

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

// ----- test / usage

#[cfg(test)]
mod tests {
    use crate::model::field::FieldSet;
    use crate::model::message::Message;
    use crate::model::twopointoh::{Field, MsgTypeField};

    #[test]
    fn it_works() {
        let f = Field::String(MsgTypeField::tag(), "D".to_string());
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
