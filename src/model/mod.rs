pub mod field;
pub mod generated;
pub mod message;
pub mod message_type;

#[cfg(test)]
mod tests {

    use std::ops::AddAssign;

    use crate::model::field::FieldSet;
    use crate::model::generated::fields::Field;
    use crate::model::message::Message;

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

        let msg_type = Field::String(35, "A".to_string());

        assert_eq!(msg_type, msg.get_field(35).clone());
        // assert_eq!(1, msg.header().iter().count());
    }
}

#[derive(Copy, Debug, Clone)]
pub enum BeginString {
    Fix40,
    Fix41,
    Fix42,
    Fix43,
    Fix44,
    Fix50,
    Fix50Sp1,
    Fix50Sp2,
    Fixt11,
}
