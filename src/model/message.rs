use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

use crate::model::field::{Field, FieldSet, NoSuchField};
use crate::model::message_type::MsgType;
use crate::model::tag::Tag;

#[derive(Default, Debug)]
pub struct Message {
    header: FieldSet,
    body: FieldSet,
    trailer: FieldSet,
}

// todo tryfrom?
impl From<&Field> for MsgType {
    fn from(s: &Field) -> Self {
        MsgType::from_str(s.string_value().unwrap()).unwrap()
    }
}

impl Message {
    #[must_use]
    pub fn of_type(msg_type: MsgType) -> Message {
        Message {
            header: FieldSet::with(vec![Field::String(Tag::MsgType, msg_type.to_string())]),
            body: FieldSet::default(),
            trailer: FieldSet::default(),
        }
    }

    ///
    /// # Errors
    ///
    pub fn msg_type(&self) -> Result<MsgType, NoSuchField> {
        match self.header.get_field(Tag::MsgType) {
            Ok(o) => Ok(o.into()),
            Err(e) => Err(e),
        }
    }

    // todo search all fielsets
    ///
    /// # Errors
    ///
    pub fn get_field(&self, tag: Tag) -> Result<&Field, NoSuchField> {
        if let Ok(s) = self.header.get_field(tag) {
            Ok(s)
        } else if let Ok(s) = self.body.get_field(tag) {
            Ok(s)
        } else {
            Err(NoSuchField { tag })
        }
    }

    pub fn set_field(&mut self, field: Field) {
        if field.is_header_field() {
            self.header.set_field(field);
        } else {
            self.body.set_field(field);
        }
    }

    pub fn to_wire_bytes(&self) -> Vec<u8> {
        // todo temp. impl.
        self.to_string()
            .replace('|', '\x01'.to_string().as_str())
            .into_bytes()
    }

    fn add_all(&mut self, field_set: FieldSet) {
        field_set.into_iter().for_each(|l| self.body.set_field(l));
    }
}

impl From<FieldSet> for Message {
    fn from(field_set: FieldSet) -> Self {
        let mut message = Message::default();
        field_set.into_iter().for_each(|field| {
            message.set_field(field);
        });
        message
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let pretty_string: String = self
            .header
            .iter()
            .chain(self.body.iter())
            .chain(self.trailer.iter())
            .map(std::string::ToString::to_string)
            .collect();
        write!(f, "{}", pretty_string)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::field::Field;
    use crate::model::message::Message;
    use crate::model::message_type::MsgType;
    use crate::model::tag::Tag;

    #[test]
    fn basic_logon_message_test() {
        let mut logon_msg = Message::of_type(MsgType::Logon);

        let text_field = Field::String(Tag::Text, "Testing".to_string());
        logon_msg.set_field(text_field);

        let expected = "35=A|58=Testing|";
        assert_eq!(expected, logon_msg.to_string());

        let expected = vec![
            51_u8, 53_u8, 61_u8, 65_u8, 1_u8, 53_u8, 56_u8, 61_u8, 84_u8, 101_u8, 115_u8, 116_u8,
            105_u8, 110_u8, 103_u8, 1_u8,
        ];
        assert_eq!(expected, logon_msg.to_wire_bytes());
    }

    #[test]
    fn adding_fields() {
        let mut msg = Message::of_type(MsgType::Custom("ABC".to_string()));

        let field = Field::String(Tag::ClOrdId, "ClOrdID123".to_string());
        msg.set_field(field);

        let expected = "35=ABC|11=ClOrdID123|";
        assert_eq!(expected, msg.to_string());

        // Add Text and update ClOrdID
        let field = Field::String(Tag::Text, "Testing!".to_string());
        msg.set_field(field);
        let field = Field::String(Tag::ClOrdId, "NewClOrdID".to_string());
        msg.set_field(field);

        let expected = "35=ABC|11=NewClOrdID|58=Testing!|";
        assert_eq!(expected, msg.to_string());
    }
}
