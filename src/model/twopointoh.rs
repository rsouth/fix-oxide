use crate::model::field::{Field, FieldSet, NoSuchField};
use crate::model::tag::Tag;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::path::Display;
use std::str::FromStr;

// #[derive(Debug, Clone, PartialOrd, PartialEq)]
// pub enum Field {
//     String(Tag, String),
//     Char(Tag, char),
//     Price(Tag, f32),
//     Int(Tag, i32),
//     Amt(Tag, i32),
//     Qty(Tag, i32),
//     Currency(Tag, String),
// }

// impl Field {
//     fn get_val(&self, val: &mut MsgType) {}
// }

// impl TryFrom<Field> for String {
//     type Error = InvalidConversion;
//
//     fn try_from(value: Field) -> Result<Self, Self::Error> {
//         match value {
//             Field::String(_, x) | Field::Currency(_, x) => Ok(x),
//             _ => Err(InvalidConversion {}),
//         }
//     }
// }

// impl TryFrom<Field> for i32 {
//     type Error = InvalidConversion;
//
//     fn try_from(value: Field) -> Result<Self, Self::Error> {
//         match value {
//             Field::Int(_, x) | Field::Amt(_, x) | Field::Qty(_, x) => Ok(x),
//             _ => Err(InvalidConversion {}),
//         }
//     }
// }

impl FieldSet {
    fn get_msg_type(&self) -> Result<StringField, NoSuchField> {
        let f = self
            .iter()
            .find_or_first(|p| p.tag() == Tag::MsgType)
            .unwrap();
        match f {
            Field::String(t, v) => Ok(StringField {
                tag: Tag::MsgType.num(),
                fd: f.clone(),
            }),
            _ => Err(NoSuchField { tag: Tag::MsgType }),
        }
    }

    fn get_string_field(&self, tag: u16) -> Result<StringField, NoSuchField> {
        let f = self.iter().find_or_first(|p| p.tag().num() == tag).unwrap();
        match f {
            Field::String(t, v) => Ok(StringField {
                tag: t.num(),
                fd: Field::String(t.clone(), v.to_string()),
            }),
            _ => Err(NoSuchField { tag: Tag::MsgType }),
        }
    }
}

impl TryFrom<Field> for MsgType {
    type Error = InvalidConversion;

    fn try_from(value: Field) -> Result<Self, Self::Error> {
        match value {
            Field::String(tag, value) => Ok(MsgType {
                fd: Field::String(tag, value),
            }),
            _ => Err(InvalidConversion {}),
        }
    }
}

pub struct MsgType {
    fd: Field,
}

pub struct StringField {
    tag: u16,
    fd: Field,
}

impl StringField {
    fn tag(&self) -> u16 {
        self.tag.into()
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

impl MsgType {
    fn tag(&self) -> u16 {
        35
    }

    fn value(&self) -> &str {
        match &self.fd {
            Field::String(t, v) => v,
            _ => "",
        }
    }
}

impl std::fmt::Display for MsgType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}|", self.tag(), self.value())
    }
}

#[cfg(test)]
mod tests {
    use crate::model::field::FieldSet;
    use crate::model::tag::Tag;
    use crate::model::twopointoh::{Field, MsgType};

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

// couldn't convert 35=D| to Field
pub struct InvalidConversion;
impl std::fmt::Display for InvalidConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

// unable to convert from i32 to FieldType
pub struct UnknownField;
impl std::fmt::Display for UnknownField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
enum FieldType {
    String,
    Int,
}

struct Dict {
    data: HashMap<i32, FieldType>,
}
impl Dict {
    // load data from XML into here
    fn init(/* xml file / data */) -> Dict {
        let mut m = HashMap::<i32, FieldType>::new();
        m.insert(60000, FieldType::String);

        Dict { data: m }
    }
}

impl TryFrom<i32> for FieldType {
    type Error = InvalidConversion;

    // from 35 to String
    fn try_from(fix_field_tag: i32) -> Result<Self, Self::Error> {
        match fix_field_tag {
            1 => Ok(FieldType::String),
            2 => Ok(FieldType::Int),
            _ => Err(InvalidConversion),
        }
    }
}

impl FromStr for Field {
    type Err = InvalidConversion;

    // from 35=D| to Field(Tag::MsgType, "D".to_string())
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "123" => Ok(Field::String(Tag::MsgType, s.to_string())),
            _ => Err(InvalidConversion {}),
        }
    }
}
