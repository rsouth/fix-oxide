use core::fmt;
use itertools::Itertools;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::vec::IntoIter;

use crate::model::tag::Tag;

#[derive(Default, Debug, Clone)]
pub struct FieldSet {
    fields: HashMap<Tag, Field>,
}

#[derive(Debug)]
pub struct NoSuchField {
    pub(crate) tag: Tag,
}

impl fmt::Display for NoSuchField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Field {} not found in message", self.tag)
    }
}

impl FieldSet {
    #[must_use]
    pub fn with(fields: Vec<Field>) -> FieldSet {
        FieldSet {
            fields: fields.into_iter().map(|k| (k.tag(), k)).collect(),
        }
    }

    pub fn set_field(&mut self, field: Field) {
        let key = field.tag();
        self.fields.borrow_mut().insert(key, field);
    }

    /// # Errors
    ///
    /// Will return `Err` if a field for `tag` is not present in the `FieldSet`
    pub fn get_field(&self, tag: Tag) -> Result<&Field, NoSuchField> {
        self.fields.get(&tag).ok_or(NoSuchField { tag })
    }

    // todo impl trait
    #[must_use]
    pub fn iter(&self) -> IntoIter<&Field> {
        self.fields
            .iter()
            .map(|s| s.1)
            .sorted_by_key(|k| k.tag().num())
    }

    // todo impl trait
    #[must_use]
    pub fn into_iter(self) -> IntoIter<Field> {
        self.fields
            .iter()
            .map(|s| s.1.clone())
            .sorted_by_key(|k| k.tag().num())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Field {
    Int(Tag, i32),
    TagNum(Tag, u128), // todo check
    SeqNum(Tag, u128),

    String(Tag, String),
    Char(Tag, char),
}

impl Field {
    pub(crate) fn is_header_field(&self) -> bool {
        match self.tag() {
            Tag::MsgType => true,
            Tag::Text | Tag::ClOrdId => false,
        }
    }
}

impl TryFrom<String> for Field {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        println!("From<String> for Field: {}", &s);
        let two_parts = s.split_once('=');
        match two_parts {
            None => Err(()),
            Some((s_tag, s_value)) => {
                println!("parsing tag: {}, field: {} into Field", s_tag, s_value);

                // figure out the tag
                let tag_num: u16 = s_tag.parse::<u16>().unwrap();
                let tag = Tag::from(tag_num);

                // build field using the tag & value
                let field = match tag {
                    Tag::MsgType | Tag::Text | Tag::ClOrdId => {
                        Field::String(tag, s_value.to_string())
                    }
                };
                Ok(field)
            }
        }
    }
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
        write!(f, "{}", self.to_delimited_string(separator))
    }
}

impl Field {
    #[must_use]
    pub fn to_delimited_string(&self, separator: char) -> String {
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

    #[must_use]
    pub fn to_bytes(&self) -> Box<[u8]> {
        // let separator = '\x01';
        Box::from(self.to_string().as_bytes().clone())
    }
}
