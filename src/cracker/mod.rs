use bytes::Bytes;

use crate::application::FixApp;
use crate::model::field::FieldSet;
use crate::model::generated::generated::Field;
use crate::model::message::Message;

struct Cracker<'a> {
    app: Box<dyn FixApp + 'a>,
}

// todo link to FixApplication...
impl Cracker<'_> {
    pub fn crack(&mut self, msg: &Bytes) {
        let msg_string = String::from_utf8_lossy(msg.as_ref()).to_string();
        let fields: Vec<Field> = msg_string
            .split('\x01')
            .into_iter()
            .map(|s| Field::try_from(s.to_string()))
            .filter_map(|s| match s {
                Ok(o) => Some(o),
                Err(_) => None,
            })
            .collect();

        let field_set = FieldSet::with(fields);
        let message: Message = field_set.into();

        self.app.on_message(message);
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    use bytes::{BufMut, Bytes, BytesMut};

    use crate::application::FixApp;
    use crate::cracker::Cracker;
    use crate::model::message::Message;

    struct TestApp {
        messages: Vec<Message>,
    }

    impl FixApp for TestApp {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn on_message(&mut self, message: Message) {
            println!("TestApp adding: {:#?}", message.to_string());
            self.messages.push(message);
        }
    }

    #[test]
    fn basic_cracker() {
        // build up a FIX message, that we 'received'
        let fields = vec!["35=A", "58=Test", "38=123"];
        let mut buf = BytesMut::with_capacity(1024);
        for field in fields {
            buf.put_slice(field.as_bytes());
            buf.put(&b"\x01"[..]);
        }

        let app = TestApp { messages: vec![] };
        let mut cracker = Cracker { app: Box::new(app) };

        let b = Bytes::from(buf);
        cracker.crack(&b);

        let a: Box<dyn FixApp> = cracker.app;
        let b: &TestApp = match a.as_any().downcast_ref::<TestApp>() {
            Some(b) => b,
            None => panic!("&a isn't a B!"),
        };

        let msg_count = b.messages.len();
        assert_eq!(1, msg_count);

        let message = b.messages.first().unwrap();
        if let Ok(text) = message.get_field_safe(58) {
            assert_eq!("Test", text.as_str_safe().unwrap());
        }
        assert_eq!("Test", message.get_field_safe(58).unwrap().as_str());
        if let Ok(one) = message.get_field_safe(38) {
            assert_eq!(123, one.as_i32_safe().unwrap());
        }
        assert_eq!(123, message.get_field(38).as_i32());
    }
}
