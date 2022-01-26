use core::convert::Into;
use core::default::Default;
use std::fmt;

use crate::model::field::{Field, FieldSet};
use crate::model::message;
use crate::model::tag::Tag;

#[derive(Debug, Eq, PartialEq)]
pub enum MsgType {
    Unknown,
    NotSet,
    Logon,
    // NewOrderSingle,
}

#[derive(Debug, Clone)]
struct UnknownMsgTypeError {
    val: String,
}

impl fmt::Display for UnknownMsgTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown MsgType value: {}", self.val)
    }
}

impl From<&str> for MsgType {
    fn from(msg_type: &str) -> Self {
        match msg_type {
            "A" => MsgType::Logon,
            _ => MsgType::Unknown,
        }
    }
}

impl From<MsgType> for &str {
    fn from(msg_type: MsgType) -> Self {
        match msg_type {
            MsgType::Logon => "A",
            MsgType::Unknown | MsgType::NotSet => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::field::Field;
    use crate::model::message::{Logon, Message};
    use crate::model::tag::Tag;

    #[test]
    fn basic_fieldset_tests() {
        let mut msg = Logon::default();

        let fld = Field::String(Tag::Text, "Testing".to_string());
        msg.body_mut().set_field(fld);

        let expected = "35=A|58=Testing|";
        assert_eq!(expected, msg.to_string());

        let expected = vec![
            51_u8, 53_u8, 61_u8, 65_u8, 1_u8, 53_u8, 56_u8, 61_u8, 84_u8, 101_u8, 115_u8, 116_u8,
            105_u8, 110_u8, 103_u8, 1_u8,
        ];
        assert_eq!(expected, msg.to_bytes());
    }
}

impl TryFrom<FieldSet> for Box<dyn Message> {
    type Error = ();

    fn try_from(field_set: FieldSet) -> Result<Self, Self::Error> {
        let maybe_message = match field_set.get_field(Tag::MsgType) {
            Ok(field) => {
                let msg_type = MsgType::from(field.string_value().unwrap());
                match msg_type {
                    MsgType::Logon => Ok(Box::new(message::Logon::default())),
                    // MsgType::NewOrderSingle => Ok(crate::model::message::NewOrderSingle::default()),
                    _ => Err(()),
                }
            }
            Err(_) => Err(()),
        };

        match maybe_message {
            Ok(mut boxed_message) => {
                for field in field_set.iter() {
                    if field.tag() != Tag::MsgType {
                        //  todo shouldn't need this. is there a way we can determine which fields are for header? a mapping mayhaps
                        boxed_message.body_mut().set_field(field.clone());
                    }
                }
                Ok(boxed_message)
            }
            Err(_) => Err(()),
        }
    }
}

pub trait Message {
    fn msg_type(&self) -> MsgType;

    fn header(&self) -> &FieldSet;
    fn body(&self) -> &FieldSet;
    fn body_mut(&mut self) -> &mut FieldSet;
    fn trailer(&self) -> &FieldSet;

    fn to_string(&self) -> String {
        // let begin_string = "FIX.4.2";
        self.header()
            .iter()
            .chain(self.body().iter())
            .chain(self.trailer().iter())
            .map(std::string::ToString::to_string)
            .collect()
    }

    fn to_bytes(&self) -> Vec<u8> {
        // todo temp. impl.
        self.to_string()
            .replace('|', '\x01'.to_string().as_str())
            .into_bytes()
    }
}

#[derive(Debug, Clone)]
pub struct Logon {
    header: FieldSet,
    body: FieldSet,
    trailer: FieldSet,
}

impl Default for Logon {
    fn default() -> Self {
        let mut header = FieldSet::default();
        header.set_field(Field::String(
            Tag::MsgType,
            TryInto::<&str>::try_into(MsgType::Logon)
                .unwrap()
                .to_string(),
        ));
        Logon {
            header,
            body: FieldSet::default(),
            trailer: FieldSet::default(),
        }
    }
}

impl Message for Logon {
    fn msg_type(&self) -> MsgType {
        match self.header().get_field(Tag::MsgType) {
            Ok(field) => field.string_value().unwrap().into(),
            Err(_) => MsgType::NotSet,
        }
    }

    fn header(&self) -> &FieldSet {
        &self.header
    }

    fn body(&self) -> &FieldSet {
        &self.body
    }

    fn body_mut(&mut self) -> &mut FieldSet {
        &mut self.body
    }

    fn trailer(&self) -> &FieldSet {
        &self.trailer
    }
}
