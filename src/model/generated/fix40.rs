use std::fmt::Formatter;
use std::str::FromStr;
use itertools::Itertools;

use rust_decimal::Decimal;

use crate::model::field::{FieldSet, FieldTypeMismatchError};
use crate::model::message_type::UnknownMsgTypeError;
use crate::model::BeginString;
use crate::model::generated::fields::Field;

pub struct FIX40CrackerUtils;
// parse string (35=D) into Field{35, "D"}
impl FIX40CrackerUtils {
    pub fn crack(s: &str) -> Result<Field, ()> {
        println!("crack for Field: {}", &s);
        let two_parts = s.split_once('=');
        match two_parts {
            Some((s_tag, s_value)) => {
                println!("parsing tag: {}, field: {} into Field", s_tag, s_value);
                  
                // figure out the tag
                let tag: u16 = s_tag.parse::<u16>().unwrap();
                   
                // build field using the tag & value
                match tag {
                    1 | 4 | 5 | 8 | 10 | 11 | 13 | 15 | 18 | 20 | 21 | 22 | 24 | 25 | 27 | 28 | 29 | 30 | 35 | 37 | 39 | 40 | 41 | 43 | 46 | 47 | 48 | 49 | 50 | 54 | 55 | 56 | 57 | 58 | 59 | 61 | 63 | 65 | 66 | 69 | 71 | 76 | 77 | 79 | 81 | 86 | 92 | 94 | 97 | 100 | 104 | 105 | 106 | 107 | 109 | 112 | 113 | 114 | 115 | 116 | 117 | 120 | 121 | 123 | 125 | 127 | 128 | 129 | 130 | 131 | 138 | 139 => Ok(Field::Char(tag, str::parse::<char>(s_value).unwrap())),

                    2 | 3 | 7 | 9 | 14 | 16 | 17 | 19 | 23 | 26 | 32 | 33 | 34 | 36 | 38 | 45 | 53 | 67 | 68 | 70 | 72 | 73 | 74 | 78 | 80 | 82 | 83 | 84 | 85 | 87 | 88 | 98 | 102 | 103 | 108 | 110 | 111 | 124 | 134 | 135 | 136 => Ok(Field::Int(tag, str::parse::<i32>(s_value).unwrap())),

                    _ => Err(()),
                }
            }
            None => Err(()),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AccountField {
    pub fd: Field,
}

impl AccountField {
    #[must_use]
    pub const fn tag() -> u16 {
        1
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AccountField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AccountField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AdvIdField {
    pub fd: Field,
}

impl AdvIdField {
    #[must_use]
    pub const fn tag() -> u16 {
        2
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AdvIdField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AdvIdField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AdvRefIDField {
    pub fd: Field,
}

impl AdvRefIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        3
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AdvRefIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AdvRefIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AdvSideField {
    pub fd: Field,
}

impl AdvSideField {
    #[must_use]
    pub const fn tag() -> u16 {
        4
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AdvSideField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AdvSideField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AdvTransTypeField {
    pub fd: Field,
}

impl AdvTransTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        5
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AdvTransTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AdvTransTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BeginSeqNoField {
    pub fd: Field,
}

impl BeginSeqNoField {
    #[must_use]
    pub const fn tag() -> u16 {
        7
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BeginSeqNoField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for BeginSeqNoField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BeginStringField {
    pub fd: Field,
}

impl BeginStringField {
    #[must_use]
    pub const fn tag() -> u16 {
        8
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BeginStringField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for BeginStringField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BodyLengthField {
    pub fd: Field,
}

impl BodyLengthField {
    #[must_use]
    pub const fn tag() -> u16 {
        9
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BodyLengthField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for BodyLengthField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CheckSumField {
    pub fd: Field,
}

impl CheckSumField {
    #[must_use]
    pub const fn tag() -> u16 {
        10
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CheckSumField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CheckSumField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClOrdIDField {
    pub fd: Field,
}

impl ClOrdIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        11
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ClOrdIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ClOrdIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CommTypeField {
    pub fd: Field,
}

impl CommTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        13
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CommTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CommTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CumQtyField {
    pub fd: Field,
}

impl CumQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        14
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CumQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CumQtyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CurrencyField {
    pub fd: Field,
}

impl CurrencyField {
    #[must_use]
    pub const fn tag() -> u16 {
        15
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CurrencyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CurrencyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EndSeqNoField {
    pub fd: Field,
}

impl EndSeqNoField {
    #[must_use]
    pub const fn tag() -> u16 {
        16
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for EndSeqNoField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for EndSeqNoField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExecIDField {
    pub fd: Field,
}

impl ExecIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        17
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ExecIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ExecIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExecInstField {
    pub fd: Field,
}

impl ExecInstField {
    #[must_use]
    pub const fn tag() -> u16 {
        18
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ExecInstField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ExecInstField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExecRefIDField {
    pub fd: Field,
}

impl ExecRefIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        19
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ExecRefIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ExecRefIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExecTransTypeField {
    pub fd: Field,
}

impl ExecTransTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        20
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ExecTransTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ExecTransTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HandlInstField {
    pub fd: Field,
}

impl HandlInstField {
    #[must_use]
    pub const fn tag() -> u16 {
        21
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for HandlInstField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for HandlInstField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IDSourceField {
    pub fd: Field,
}

impl IDSourceField {
    #[must_use]
    pub const fn tag() -> u16 {
        22
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IDSourceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for IDSourceField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IOIidField {
    pub fd: Field,
}

impl IOIidField {
    #[must_use]
    pub const fn tag() -> u16 {
        23
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IOIidField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for IOIidField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IOIOthSvcField {
    pub fd: Field,
}

impl IOIOthSvcField {
    #[must_use]
    pub const fn tag() -> u16 {
        24
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IOIOthSvcField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for IOIOthSvcField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IOIQltyIndField {
    pub fd: Field,
}

impl IOIQltyIndField {
    #[must_use]
    pub const fn tag() -> u16 {
        25
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IOIQltyIndField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for IOIQltyIndField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IOIRefIDField {
    pub fd: Field,
}

impl IOIRefIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        26
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IOIRefIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for IOIRefIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IOISharesField {
    pub fd: Field,
}

impl IOISharesField {
    #[must_use]
    pub const fn tag() -> u16 {
        27
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IOISharesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for IOISharesField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IOITransTypeField {
    pub fd: Field,
}

impl IOITransTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        28
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IOITransTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for IOITransTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LastCapacityField {
    pub fd: Field,
}

impl LastCapacityField {
    #[must_use]
    pub const fn tag() -> u16 {
        29
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LastCapacityField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LastCapacityField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LastMktField {
    pub fd: Field,
}

impl LastMktField {
    #[must_use]
    pub const fn tag() -> u16 {
        30
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LastMktField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LastMktField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LastSharesField {
    pub fd: Field,
}

impl LastSharesField {
    #[must_use]
    pub const fn tag() -> u16 {
        32
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LastSharesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LastSharesField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LinesOfTextField {
    pub fd: Field,
}

impl LinesOfTextField {
    #[must_use]
    pub const fn tag() -> u16 {
        33
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LinesOfTextField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LinesOfTextField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MsgSeqNumField {
    pub fd: Field,
}

impl MsgSeqNumField {
    #[must_use]
    pub const fn tag() -> u16 {
        34
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MsgSeqNumField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MsgSeqNumField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MsgTypeField {
    pub fd: Field,
}

impl MsgTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        35
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MsgTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MsgTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NewSeqNoField {
    pub fd: Field,
}

impl NewSeqNoField {
    #[must_use]
    pub const fn tag() -> u16 {
        36
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NewSeqNoField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NewSeqNoField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OrderIDField {
    pub fd: Field,
}

impl OrderIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        37
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrderIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OrderIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OrderQtyField {
    pub fd: Field,
}

impl OrderQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        38
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrderQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OrderQtyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OrdStatusField {
    pub fd: Field,
}

impl OrdStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        39
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrdStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OrdStatusField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OrdTypeField {
    pub fd: Field,
}

impl OrdTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        40
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrdTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OrdTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OrigClOrdIDField {
    pub fd: Field,
}

impl OrigClOrdIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        41
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrigClOrdIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OrigClOrdIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PossDupFlagField {
    pub fd: Field,
}

impl PossDupFlagField {
    #[must_use]
    pub const fn tag() -> u16 {
        43
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PossDupFlagField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PossDupFlagField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RefSeqNumField {
    pub fd: Field,
}

impl RefSeqNumField {
    #[must_use]
    pub const fn tag() -> u16 {
        45
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RefSeqNumField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RefSeqNumField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RelatdSymField {
    pub fd: Field,
}

impl RelatdSymField {
    #[must_use]
    pub const fn tag() -> u16 {
        46
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RelatdSymField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RelatdSymField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Rule80AField {
    pub fd: Field,
}

impl Rule80AField {
    #[must_use]
    pub const fn tag() -> u16 {
        47
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for Rule80AField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for Rule80AField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityIDField {
    pub fd: Field,
}

impl SecurityIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        48
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecurityIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SenderCompIDField {
    pub fd: Field,
}

impl SenderCompIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        49
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SenderCompIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SenderCompIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SenderSubIDField {
    pub fd: Field,
}

impl SenderSubIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        50
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SenderSubIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SenderSubIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SharesField {
    pub fd: Field,
}

impl SharesField {
    #[must_use]
    pub const fn tag() -> u16 {
        53
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SharesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SharesField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SideField {
    pub fd: Field,
}

impl SideField {
    #[must_use]
    pub const fn tag() -> u16 {
        54
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SideField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SideField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SymbolField {
    pub fd: Field,
}

impl SymbolField {
    #[must_use]
    pub const fn tag() -> u16 {
        55
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SymbolField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SymbolField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TargetCompIDField {
    pub fd: Field,
}

impl TargetCompIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        56
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TargetCompIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TargetCompIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TargetSubIDField {
    pub fd: Field,
}

impl TargetSubIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        57
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TargetSubIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TargetSubIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TextField {
    pub fd: Field,
}

impl TextField {
    #[must_use]
    pub const fn tag() -> u16 {
        58
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TextField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TextField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TimeInForceField {
    pub fd: Field,
}

impl TimeInForceField {
    #[must_use]
    pub const fn tag() -> u16 {
        59
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TimeInForceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TimeInForceField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UrgencyField {
    pub fd: Field,
}

impl UrgencyField {
    #[must_use]
    pub const fn tag() -> u16 {
        61
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for UrgencyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for UrgencyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SettlmntTypField {
    pub fd: Field,
}

impl SettlmntTypField {
    #[must_use]
    pub const fn tag() -> u16 {
        63
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlmntTypField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SettlmntTypField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SymbolSfxField {
    pub fd: Field,
}

impl SymbolSfxField {
    #[must_use]
    pub const fn tag() -> u16 {
        65
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SymbolSfxField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SymbolSfxField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListIDField {
    pub fd: Field,
}

impl ListIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        66
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ListIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ListIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListSeqNoField {
    pub fd: Field,
}

impl ListSeqNoField {
    #[must_use]
    pub const fn tag() -> u16 {
        67
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ListSeqNoField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ListSeqNoField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListNoOrdsField {
    pub fd: Field,
}

impl ListNoOrdsField {
    #[must_use]
    pub const fn tag() -> u16 {
        68
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ListNoOrdsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ListNoOrdsField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListExecInstField {
    pub fd: Field,
}

impl ListExecInstField {
    #[must_use]
    pub const fn tag() -> u16 {
        69
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ListExecInstField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ListExecInstField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllocIDField {
    pub fd: Field,
}

impl AllocIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        70
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllocTransTypeField {
    pub fd: Field,
}

impl AllocTransTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        71
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocTransTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocTransTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RefAllocIDField {
    pub fd: Field,
}

impl RefAllocIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        72
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RefAllocIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RefAllocIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoOrdersField {
    pub fd: Field,
}

impl NoOrdersField {
    #[must_use]
    pub const fn tag() -> u16 {
        73
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoOrdersField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NoOrdersField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AvgPrxPrecisionField {
    pub fd: Field,
}

impl AvgPrxPrecisionField {
    #[must_use]
    pub const fn tag() -> u16 {
        74
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AvgPrxPrecisionField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AvgPrxPrecisionField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExecBrokerField {
    pub fd: Field,
}

impl ExecBrokerField {
    #[must_use]
    pub const fn tag() -> u16 {
        76
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ExecBrokerField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ExecBrokerField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OpenCloseField {
    pub fd: Field,
}

impl OpenCloseField {
    #[must_use]
    pub const fn tag() -> u16 {
        77
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OpenCloseField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OpenCloseField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoAllocsField {
    pub fd: Field,
}

impl NoAllocsField {
    #[must_use]
    pub const fn tag() -> u16 {
        78
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoAllocsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NoAllocsField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllocAccountField {
    pub fd: Field,
}

impl AllocAccountField {
    #[must_use]
    pub const fn tag() -> u16 {
        79
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocAccountField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocAccountField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllocSharesField {
    pub fd: Field,
}

impl AllocSharesField {
    #[must_use]
    pub const fn tag() -> u16 {
        80
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocSharesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocSharesField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProcessCodeField {
    pub fd: Field,
}

impl ProcessCodeField {
    #[must_use]
    pub const fn tag() -> u16 {
        81
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ProcessCodeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ProcessCodeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoRptsField {
    pub fd: Field,
}

impl NoRptsField {
    #[must_use]
    pub const fn tag() -> u16 {
        82
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoRptsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NoRptsField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RptSeqField {
    pub fd: Field,
}

impl RptSeqField {
    #[must_use]
    pub const fn tag() -> u16 {
        83
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for RptSeqField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for RptSeqField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CxlQtyField {
    pub fd: Field,
}

impl CxlQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        84
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CxlQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CxlQtyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoDlvyInstField {
    pub fd: Field,
}

impl NoDlvyInstField {
    #[must_use]
    pub const fn tag() -> u16 {
        85
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoDlvyInstField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NoDlvyInstField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DlvyInstField {
    pub fd: Field,
}

impl DlvyInstField {
    #[must_use]
    pub const fn tag() -> u16 {
        86
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DlvyInstField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DlvyInstField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllocStatusField {
    pub fd: Field,
}

impl AllocStatusField {
    #[must_use]
    pub const fn tag() -> u16 {
        87
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocStatusField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocStatusField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllocRejCodeField {
    pub fd: Field,
}

impl AllocRejCodeField {
    #[must_use]
    pub const fn tag() -> u16 {
        88
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for AllocRejCodeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for AllocRejCodeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BrokerOfCreditField {
    pub fd: Field,
}

impl BrokerOfCreditField {
    #[must_use]
    pub const fn tag() -> u16 {
        92
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BrokerOfCreditField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for BrokerOfCreditField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EmailTypeField {
    pub fd: Field,
}

impl EmailTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        94
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for EmailTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for EmailTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PossResendField {
    pub fd: Field,
}

impl PossResendField {
    #[must_use]
    pub const fn tag() -> u16 {
        97
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for PossResendField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for PossResendField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EncryptMethodField {
    pub fd: Field,
}

impl EncryptMethodField {
    #[must_use]
    pub const fn tag() -> u16 {
        98
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for EncryptMethodField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for EncryptMethodField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExDestinationField {
    pub fd: Field,
}

impl ExDestinationField {
    #[must_use]
    pub const fn tag() -> u16 {
        100
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ExDestinationField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ExDestinationField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CxlRejReasonField {
    pub fd: Field,
}

impl CxlRejReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        102
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CxlRejReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CxlRejReasonField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OrdRejReasonField {
    pub fd: Field,
}

impl OrdRejReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        103
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OrdRejReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OrdRejReasonField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IOIQualifierField {
    pub fd: Field,
}

impl IOIQualifierField {
    #[must_use]
    pub const fn tag() -> u16 {
        104
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IOIQualifierField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for IOIQualifierField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WaveNoField {
    pub fd: Field,
}

impl WaveNoField {
    #[must_use]
    pub const fn tag() -> u16 {
        105
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for WaveNoField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for WaveNoField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IssuerField {
    pub fd: Field,
}

impl IssuerField {
    #[must_use]
    pub const fn tag() -> u16 {
        106
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IssuerField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for IssuerField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityDescField {
    pub fd: Field,
}

impl SecurityDescField {
    #[must_use]
    pub const fn tag() -> u16 {
        107
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SecurityDescField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SecurityDescField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HeartBtIntField {
    pub fd: Field,
}

impl HeartBtIntField {
    #[must_use]
    pub const fn tag() -> u16 {
        108
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for HeartBtIntField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for HeartBtIntField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClientIDField {
    pub fd: Field,
}

impl ClientIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        109
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ClientIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ClientIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MinQtyField {
    pub fd: Field,
}

impl MinQtyField {
    #[must_use]
    pub const fn tag() -> u16 {
        110
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MinQtyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MinQtyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MaxFloorField {
    pub fd: Field,
}

impl MaxFloorField {
    #[must_use]
    pub const fn tag() -> u16 {
        111
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MaxFloorField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MaxFloorField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TestReqIDField {
    pub fd: Field,
}

impl TestReqIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        112
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for TestReqIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for TestReqIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReportToExchField {
    pub fd: Field,
}

impl ReportToExchField {
    #[must_use]
    pub const fn tag() -> u16 {
        113
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ReportToExchField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ReportToExchField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LocateReqdField {
    pub fd: Field,
}

impl LocateReqdField {
    #[must_use]
    pub const fn tag() -> u16 {
        114
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for LocateReqdField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for LocateReqdField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OnBehalfOfCompIDField {
    pub fd: Field,
}

impl OnBehalfOfCompIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        115
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OnBehalfOfCompIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OnBehalfOfCompIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OnBehalfOfSubIDField {
    pub fd: Field,
}

impl OnBehalfOfSubIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        116
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OnBehalfOfSubIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OnBehalfOfSubIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuoteIDField {
    pub fd: Field,
}

impl QuoteIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        117
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for QuoteIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for QuoteIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SettlCurrencyField {
    pub fd: Field,
}

impl SettlCurrencyField {
    #[must_use]
    pub const fn tag() -> u16 {
        120
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for SettlCurrencyField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for SettlCurrencyField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ForexReqField {
    pub fd: Field,
}

impl ForexReqField {
    #[must_use]
    pub const fn tag() -> u16 {
        121
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for ForexReqField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for ForexReqField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GapFillFlagField {
    pub fd: Field,
}

impl GapFillFlagField {
    #[must_use]
    pub const fn tag() -> u16 {
        123
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for GapFillFlagField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for GapFillFlagField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoExecsField {
    pub fd: Field,
}

impl NoExecsField {
    #[must_use]
    pub const fn tag() -> u16 {
        124
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoExecsField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NoExecsField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CxlTypeField {
    pub fd: Field,
}

impl CxlTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        125
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for CxlTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for CxlTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DKReasonField {
    pub fd: Field,
}

impl DKReasonField {
    #[must_use]
    pub const fn tag() -> u16 {
        127
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DKReasonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DKReasonField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DeliverToCompIDField {
    pub fd: Field,
}

impl DeliverToCompIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        128
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DeliverToCompIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DeliverToCompIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DeliverToSubIDField {
    pub fd: Field,
}

impl DeliverToSubIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        129
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for DeliverToSubIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for DeliverToSubIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IOINaturalFlagField {
    pub fd: Field,
}

impl IOINaturalFlagField {
    #[must_use]
    pub const fn tag() -> u16 {
        130
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for IOINaturalFlagField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for IOINaturalFlagField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuoteReqIDField {
    pub fd: Field,
}

impl QuoteReqIDField {
    #[must_use]
    pub const fn tag() -> u16 {
        131
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for QuoteReqIDField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for QuoteReqIDField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BidSizeField {
    pub fd: Field,
}

impl BidSizeField {
    #[must_use]
    pub const fn tag() -> u16 {
        134
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for BidSizeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for BidSizeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OfferSizeField {
    pub fd: Field,
}

impl OfferSizeField {
    #[must_use]
    pub const fn tag() -> u16 {
        135
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for OfferSizeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for OfferSizeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoMiscFeesField {
    pub fd: Field,
}

impl NoMiscFeesField {
    #[must_use]
    pub const fn tag() -> u16 {
        136
    }

    // tag_type: INT
    fn value(&self) -> i32 {
        match &self.fd {
            Field::Int(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for NoMiscFeesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for NoMiscFeesField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Int(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MiscFeeCurrField {
    pub fd: Field,
}

impl MiscFeeCurrField {
    #[must_use]
    pub const fn tag() -> u16 {
        138
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MiscFeeCurrField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MiscFeeCurrField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MiscFeeTypeField {
    pub fd: Field,
}

impl MiscFeeTypeField {
    #[must_use]
    pub const fn tag() -> u16 {
        139
    }

    // tag_type: CHAR
    fn value(&self) -> char {
        match &self.fd {
            Field::Char(_, v) => *v,
            _ => panic!(""),
        }
    }
}

impl std::fmt::Display for MiscFeeTypeField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", Self::tag(), self.value())
    }
}
impl TryFrom<&Field> for MiscFeeTypeField {
    type Error = UnknownMsgTypeError;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value {
            Field::Char(_, _) => Ok(Self { fd: value.clone() }),
            _ => Err(UnknownMsgTypeError {
                val: value.to_string(),
            }),
        }
    }
}
