use core::fmt;
use std::any::Any;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::vec::IntoIter;

use crate::model::generated::fields::Field;
use crate::model::generated::fix42::MsgTypeField;
use crate::model::BeginString;
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

impl Display for FieldSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.fields.iter().map(|t| t.1.to_string()).join("")
        )
    }
}

impl FieldSet {
    ///
    /// # Errors
    ///
    pub fn get_msg_type(&self) -> Result<&str, UnknownField> {
        self.iter()
            .find_or_first(|p| p.tag() == 35)
            .map(|i| i.as_str())
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

    pub fn set_msg_type<T: Into<MsgTypeField>>(&mut self, field: T) {
        let ket = field.into(); //.tag();
        self.fields.borrow_mut().insert(MsgTypeField::tag(), ket.fd);
    }

    pub fn set_field_2<T: Into<Field>>(&mut self, field: T) {
        let key = field.into();
        let tag = key.tag();
        self.fields.borrow_mut().insert(key.tag(), key);
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

impl TryFrom<String> for BeginString {
    type Error = NoSuchField;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "FIX.4.0" => Ok(BeginString::Fix40),
            "FIX.4.1" => Ok(BeginString::Fix41),
            "FIX.4.2" => Ok(BeginString::Fix42),
            "FIX.4.3" => Ok(BeginString::Fix43),
            "FIX.4.4" => Ok(BeginString::Fix44),
            "FIX.5.0" => Ok(BeginString::Fix50),
            _ => Err(NoSuchField { tag: 8 }),
        }
    }
}

impl From<BeginString> for String {
    fn from(begin_string: BeginString) -> Self {
        match begin_string {
            BeginString::Fix40 => "FIX.4.0".to_string(),
            BeginString::Fix41 => "FIX.4.1".to_string(),
            BeginString::Fix42 => "FIX.4.2".to_string(),
            BeginString::Fix43 => "FIX.4.3".to_string(),
            BeginString::Fix44 => "FIX.4.4".to_string(),
            BeginString::Fix50 => "FIX.5.0".to_string(),
            BeginString::Fix50Sp1 => "TODO".to_string(),
            BeginString::Fix50Sp2 => "TODO".to_string(),
            BeginString::Fixt11 => "TODO".to_string(),
        }
    }
}
