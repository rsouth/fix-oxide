pub mod field;
pub mod message;
pub mod message_type;
pub mod twopointoh;

#[cfg(test)]
mod tests {
    use std::ops::AddAssign;

    use crate::model::field::FieldSet;
    use crate::model::message::Message;
    use crate::model::twopointoh::{Field, MsgType};

    #[test]
    fn basic_fieldset_tests() {
        let mut fields = FieldSet::default();
        fields.set_field(Field::String(58, "Test".to_string()));

        let text = fields.get_field(58).ok().unwrap();
        assert_eq!(text.tag(), 58);
        assert_eq!(text.as_str_safe().unwrap(), "Test");

        // only added 1 tag
        assert_eq!(1, fields.iter().count());

        let mut cnt = 0;

        fields.iter().for_each(|_| cnt.add_assign(1));
        assert_eq!(1, cnt);
    }

    #[test]
    fn basic_logout_message_test() {
        let msg = Message::of_type("A");

        let msg_type = MsgType {
            fd: Field::String(35, "A".to_string()),
        };
        assert_eq!(msg_type, msg.msg_type().unwrap());
        // assert_eq!(1, msg.header().iter().count());
    }
}
