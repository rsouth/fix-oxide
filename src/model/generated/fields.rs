use std::fmt::Formatter;
use std::str::FromStr;
use itertools::Itertools;

use rust_decimal::Decimal;

use crate::model::field::{FieldSet, FieldTypeMismatchError};
use crate::model::message_type::UnknownMsgTypeError;
use crate::model::BeginString;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Field {
    Char(u16, char),
    Int(u16, i32),
    String(u16, String),
    Decimal(u16, Decimal),
}

impl Field {
   pub fn crack(bs: BeginString, to_parse: &str) -> Result<Field, ()> {
       match bs {
           BeginString::Fix40 => Ok(crate::model::generated::fix40::FIX40CrackerUtils::crack(
               to_parse,
           ).unwrap()),
           BeginString::Fix41 => Ok(crate::model::generated::fix41::FIX41CrackerUtils::crack(
               to_parse,
           ).unwrap()),
           BeginString::Fix42 => Ok(crate::model::generated::fix42::FIX42CrackerUtils::crack(
               to_parse,
           ).unwrap()),
           BeginString::Fix43 => Ok(crate::model::generated::fix43::FIX43CrackerUtils::crack(
               to_parse,
           ).unwrap()),
           BeginString::Fix44 => Ok(crate::model::generated::fix44::FIX44CrackerUtils::crack(
               to_parse,
           ).unwrap()),
           BeginString::Fix50 => Ok(crate::model::generated::fix50::FIX50CrackerUtils::crack(
               to_parse,
           ).unwrap()),
           BeginString::Fix50Sp1 => Ok(crate::model::generated::fix50sp1::FIX50SP1CrackerUtils::crack(
               to_parse,
           ).unwrap()),
           BeginString::Fix50Sp2 => Ok(crate::model::generated::fix50sp2::FIX50SP2CrackerUtils::crack(
               to_parse,
           ).unwrap()),
           BeginString::Fixt11 => Ok(crate::model::generated::fixt11::FIXT11CrackerUtils::crack(
               to_parse,
           ).unwrap()),
       }
   }
}
impl Field {

    ///
    /// # Errors
    ///
    pub const fn as_char_safe(&self) -> Result<char, FieldTypeMismatchError> {
        match self {
            Field::Char(_, v) => Ok(*v),
            _ => Err(FieldTypeMismatchError {}),
        }
    }

    ///
    /// # Errors
    ///
    /// # Panics
    ///
    pub fn as_char(&self) -> char {
        match self {
            Field::Char(_, v) => *v,
            _ => panic!(),
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
    pub fn as_i32(&self) -> i32 {
        match self {
            Field::Int(_, v) => *v,
            _ => panic!(),
        }
    }

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
    pub fn as_str(&self) -> &str {
        match self {
            Field::String(_, v) => v,
            _ => panic!(),
        }
    }

    ///
    /// # Errors
    ///
    pub const fn as_decimal_safe(&self) -> Result<Decimal, FieldTypeMismatchError> {
        match self {
            Field::Decimal(_, v) => Ok(*v),
            _ => Err(FieldTypeMismatchError {}),
        }
    }

    ///
    /// # Errors
    ///
    /// # Panics
    ///
    pub fn as_decimal(&self) -> Decimal {
        match self {
            Field::Decimal(_, v) => *v,
            _ => panic!(),
        }
    }

    #[must_use]
    pub const fn tag(&self) -> u16 {
        match self {
            Field::String(t, _) 
            | Field::Char(t, _) 
            | Field::Decimal(t, _) 
            | Field::Int(t, _) 
            => *t,
    }
}

    #[must_use]
    pub fn to_delimited_string(&self, separator: char) -> String {
        match self {
            // char
            Field::Char(t, v) => format!("{}={}{}", t, v, separator),
            // i32
            Field::Int(t, v) => format!("{}={}{}", t, v, separator),
            // &str
            Field::String(t, v) => format!("{}={}{}", t, v, separator),
            // Decimal
            Field::Decimal(t, v) => format!("{}={}{}", t, v, separator),
        }
    }
}
impl FieldSet {
    /// for use by custom fields
    pub fn get_str_field(&self, tag: u16) -> Result<StringField, FieldTypeMismatchError> {
        let f = self.iter().find_or_first(|p| p.tag() == tag).unwrap();
        match f {
            Field::String(t, v) => Ok(StringField {
                tag: *t,
                fd: Field::String(*t, v.to_string()),
            }),
            _ => Err(FieldTypeMismatchError {}),
        }
    }
    /// for use by custom fields
    pub fn get_char_field(&self, tag: u16) -> Result<CharField, FieldTypeMismatchError> {
        let f = self.iter().find_or_first(|p| p.tag() == tag).unwrap();
        match f {
            Field::Char(t, v) => Ok(CharField {
                tag: *t,
                fd: Field::Char(*t, *v),
            }),
            _ => Err(FieldTypeMismatchError {}),
        }
    }
    /// for use by custom fields
    pub fn get_decimal_field(&self, tag: u16) -> Result<DecimalField, FieldTypeMismatchError> {
        let f = self.iter().find_or_first(|p| p.tag() == tag).unwrap();
        match f {
            Field::Decimal(t, v) => Ok(DecimalField {
                tag: *t,
                fd: Field::Decimal(*t, *v),
            }),
            _ => Err(FieldTypeMismatchError {}),
        }
    }
    /// for use by custom fields
    pub fn get_i32_field(&self, tag: u16) -> Result<IntField, FieldTypeMismatchError> {
        let f = self.iter().find_or_first(|p| p.tag() == tag).unwrap();
        match f {
            Field::Int(t, v) => Ok(IntField {
                tag: *t,
                fd: Field::Int(*t, *v),
            }),
            _ => Err(FieldTypeMismatchError {}),
        }
    }
}
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
pub struct CharField {
    tag: u16,
    fd: Field,
}

impl CharField {
    const fn tag(&self) -> u16 {
        self.tag
    }

    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CharField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", self.tag(), self.value())
    }
}
pub struct DecimalField {
    tag: u16,
    fd: Field,
}

impl DecimalField {
    const fn tag(&self) -> u16 {
        self.tag
    }

    fn value(&self) -> Decimal {
        match &self.fd {
            Field::Decimal(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DecimalField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", self.tag(), self.value())
    }
}
pub struct IntField {
    tag: u16,
    fd: Field,
}

impl IntField {
    const fn tag(&self) -> u16 {
        self.tag
    }

    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IntField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", self.tag(), self.value())
    }
}
