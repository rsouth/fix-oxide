use crate::model::tag::Tag;
use core::fmt;
use std::borrow::BorrowMut;
use std::collections::hash_map::Values;
use std::collections::HashMap;
use std::fmt::Formatter;

#[derive(Default, Debug, Clone)]
pub struct FieldSet {
    fields: HashMap<Tag, Field>,
}

pub struct NoSuchField {
    tag: Tag,
}

impl fmt::Display for NoSuchField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Field {} not found in message", self.tag)
    }
}

impl FieldSet {
    pub fn set_field(&mut self, field: Field) {
        let key = field.tag();
        self.fields.borrow_mut().insert(key, field);
    }

    pub fn get_field(&self, tag: Tag) -> Result<&Field, NoSuchField> {
        self.fields.get(&tag).ok_or(NoSuchField { tag })
    }

    pub fn iter(&self) -> Values<'_, Tag, Field> {
        self.fields.values()
    }
}

#[derive(Debug, Clone)]
pub enum Field {
    Int(Tag, i32),
    TagNum(Tag, u128), // todo check
    SeqNum(Tag, u128),

    String(Tag, String),
    Char(Tag, char),
}

#[derive(Debug, Clone)]
pub struct FieldTypeMismatchError;

impl fmt::Display for FieldTypeMismatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Type mismatch")
    }
}

impl Field {
    pub(crate) fn string_value(&self) -> Result<&str, FieldTypeMismatchError> {
        match self {
            Field::String(_, v) => Ok(v),
            _ => Err(FieldTypeMismatchError {}),
        }
    }

    pub(crate) fn tag(&self) -> Tag {
        self.into()
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let separator = '|';
        write!(f, "{}", self.to_ssttrr(separator))
        // match self {
        //     // &char
        //     Field::Char(t, v) => write!(f, "{}={}{}", t.num(), v, separator),
        //     // &i32
        //     Field::Int(t, v) => write!(f, "{}={}{}", t.num(), v, separator),
        //     // &u128
        //     Field::TagNum(t, v) | Field::SeqNum(t, v) => {
        //         write!(f, "{}={}{}", t.num(), v, separator)
        //     }
        //     // &String
        //     Field::String(t, v) => write!(f, "{}={}{}", t.num(), v, separator),
        // }
    }
}

impl Field {
    pub fn to_ssttrr(&self, separator: char) -> String {
        match self {
            // &char
            Field::Char(t, v) => format!("{}={}{}", t.num(), v, separator),
            // &i32
            Field::Int(t, v) => format!("{}={}{}", t.num(), v, separator),
            // &String
            Field::String(t, v) => format!("{}={}{}", t.num(), v, separator),
            // &u128
            Field::TagNum(t, v) | Field::SeqNum(t, v) => {
                format!("{}={}{}", t.num(), v, separator)
            }
        }
        .to_string()
    }

    pub fn to_bytes(&self) -> Box<[u8]> {
        // let separator = '\x01';
        Box::from(self.to_string().as_bytes().clone())
    }
}
