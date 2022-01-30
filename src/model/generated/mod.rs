pub mod generated;

#[cfg(test)]
mod tests {
    use crate::model::field::FieldSet;
    use crate::model::generated::generated::{Field, MsgTypeField};
    use crate::model::message::Message;

    #[test]
    fn it_works() {
        let f = Field::String(MsgTypeField::tag(), "D".to_string());
        let mut fs = FieldSet::default();
        fs.set_field(f.clone());

        let expected = "35=D|";
        assert_eq!(expected, fs.get_msg_type().ok().unwrap().to_string());
        assert_eq!(expected, fs.get_str_field(35).ok().unwrap().to_string());

        // message.set_field(fs.get_msg_type().ok().unwrap().clone());
        // let msg_type = fs.get_msg_type().ok().unwrap().clone();
        let mut message = Message::default();
        message.set_field(f);
    }
}
