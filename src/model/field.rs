use core::fmt;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::str::FromStr;
use std::vec::IntoIter;

use crate::model::generated::generated::{Field, MsgTypeField};
use itertools::Itertools;

// todo thinking, nothing here should be generated; those impls in a different file

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MsgCat {
    Admin,
    App,
}

impl FromStr for MsgCat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(MsgCat::Admin),
            "app" => Ok(MsgCat::App),
            _ => Err(()),
        }
    }
}

// --- FieldSet -----------

#[derive(Default, Debug, Clone)]
pub struct FieldSet {
    fields: HashMap<u16, Field>,
}

impl FieldSet {
    ///
    /// # Errors
    ///
    pub fn get_msg_type(&self) -> Result<MsgTypeField, UnknownField> {
        self.iter()
            .find_or_first(|p| p.tag() == MsgTypeField::tag())
            .map(|i| MsgTypeField { fd: i.clone() })
            .ok_or(UnknownField {})
    }

    #[must_use]
    pub fn with(fields: Vec<Field>) -> Self {
        Self {
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
    pub fn get_field(&self, tag: u16) -> Result<&Field, NoSuchField> {
        self.fields.get(&tag).ok_or(NoSuchField { tag })
    }

    // todo impl trait
    #[must_use]
    pub fn iter(&self) -> IntoIter<&Field> {
        self.fields.iter().map(|s| s.1).sorted_by_key(|k| k.tag())
    }

    // todo impl trait
    #[must_use]
    pub fn into_iter(self) -> IntoIter<Field> {
        self.fields
            .iter()
            .map(|s| s.1.clone())
            .sorted_by_key(Field::tag)
    }
}

/// --- Field impls which _do not_ need to be generated

impl Field {
    // todo do we need is_body and is_trailer also?
    // todo NEED THIS but should pull from config
    #[must_use]
    pub const fn is_header_field(&self) -> bool {
        matches!(self.tag(), 8 | 35)
    }

    #[must_use]
    pub fn to_bytes(&self) -> Box<[u8]> {
        // let separator = '\x01';
        Box::from(self.to_string().as_bytes())
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let separator = '|';
        write!(f, "{}", self.to_delimited_string(separator))
    }
}

// --- Errors.....

/// Field was not found on the Message
#[derive(Debug)]
pub struct NoSuchField {
    pub tag: u16,
}
impl fmt::Display for NoSuchField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Field {} not found in message", self.tag)
    }
}

#[derive(Debug, Clone)]
pub struct FieldTypeMismatchError;
impl fmt::Display for FieldTypeMismatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Type mismatch")
    }
}

#[derive(Debug, Clone)]
pub struct UnknownField;
impl std::fmt::Display for UnknownField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error message here")
    }
}
