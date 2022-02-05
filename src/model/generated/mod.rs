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
    use crate::model::generated::fields::{Field, StringField};
    use crate::model::generated::fix42::MsgTypeField;
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

    #[test]
    fn test_fix4_0() {
        use self::fix40::*;
        let mut fs = FieldSet::default(); // ::with(ss);

        // todo need to support pre-FIX.4.2 char-string type.
        fs.set_field_2(MsgTypeField::new('A'));
        fs.set_field_2(AccountField::new('1'));
        fs.set_field_2(ClOrdIDField::new("123"));
        fs.set_field_2(OrderQtyField::new(123));
        fs.set_field_2(Field::String(60000, "BOOK SHELF".to_string()));
        println!("{}", fs);

        fs.set_field_2(MsgTypeField::new("AI"));
        // let f = Field::String(123, "TEST".to_string());
        // let v = f.value();

        println!("{}", fs);
    }

    #[test]
    fn test_fix4_2() {
        use self::fix42::*;
        let mut fs = FieldSet::default(); // ::with(ss);

        fs.set_field_2(MsgTypeField::new("A"));
        fs.set_field_2(AccountField::new("123"));
        fs.set_field_2(ClOrdIDField::new("123"));
        fs.set_field_2(OrderQtyField::new(123));
        fs.set_field_2(Field::String(60000, "BOOK SHELF".to_string()));
        // todo u16 max val too low?

        println!("{}", fs);
    }
}
