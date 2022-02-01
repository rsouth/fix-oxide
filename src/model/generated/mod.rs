pub mod fields;
pub mod fix40;
pub mod fix41;
pub mod fix42;
pub mod fix43;
pub mod fix44;
pub mod fix50;
pub mod fix50sp1;
pub mod fix50sp2;
pub mod fixt11;

#[cfg(test)]
mod tests {
    use crate::model;
    use crate::model::field::FieldSet;
    use crate::model::generated::fields::Field;
    use crate::model::generated::*;
    use crate::model::message::Message;

    #[test]
    fn it_works() {
        let f = Field::String(fix42::MsgTypeField::tag(), "D".to_string());
        let mut fs = FieldSet::default();
        fs.set_field(f.clone());

        let expected = "D";
        assert_eq!(expected, fs.get_msg_type().ok().unwrap().to_string());
        let expected = "35=D|";
        assert_eq!(expected, fs.get_str_field(35).ok().unwrap().to_string());

        // message.set_field(fs.get_msg_type().ok().unwrap().clone());
        // let msg_type = fs.get_msg_type().ok().unwrap().clone();
        let mut message = Message::default();
        message.set_field(f);
    }
}
