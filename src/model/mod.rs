pub mod field;
pub mod message;
pub mod tag;

#[cfg(test)]
mod tests {
    use crate::model::field::{Field, FieldSet};
    use crate::model::message::{Logon, Message, MsgType};
    use crate::model::tag::Tag;
    use std::ops::AddAssign;

    #[test]
    fn basic_fieldset_tests() {
        let mut fields = FieldSet::default();
        fields.set_field(Field::String(Tag::Text, "Test".to_string()));

        let text = fields.get_field(Tag::Text).ok().unwrap();
        assert_eq!(text.tag(), Tag::Text);
        assert_eq!(text.string_value().unwrap(), "Test");

        // only added 1 tag
        assert_eq!(1, fields.iter().count());

        let mut cnt = 0;

        fields.iter().for_each(|_| cnt.add_assign(1));
        assert_eq!(1, cnt);
    }

    #[test]
    fn basic_logout_message_test() {
        let msg = Logon::default();

        assert_eq!(MsgType::Logon, msg.msg_type());
        assert_eq!(1, msg.header().iter().count());
    }
}
