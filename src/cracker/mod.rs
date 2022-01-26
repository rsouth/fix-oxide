use bytes::Bytes;

use crate::application::FixApp;
use crate::model::field::{Field, FieldSet};
use crate::model::message::Message;
use crate::model::tag::Tag;

struct Cracker<'a> {
    app: Box<dyn FixApp + 'a>, // app: dyn &FixApp,
}

// todo link to FixApplication...
impl Cracker<'_> {
    pub(crate) fn crack(&mut self, msg: Bytes) {
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
        let message: Result<Box<dyn Message>, ()> = field_set.try_into();

        match message {
            Ok(boxed_message) => self.app.on_message(boxed_message),
            Err(_) => println!("didn't manage to create a message!"),
        }
    }
}

impl From<&[u8]> for Field {
    fn from(_data: &[u8]) -> Self {
        Field::String(Tag::Text, "wer".to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    use bytes::{BufMut, Bytes, BytesMut};

    use crate::application::FixApp;
    use crate::cracker::Cracker;
    use crate::model::message::{Logon, Message};
    use crate::model::tag::Tag;

    struct TestApp {
        messages: Vec<Box<dyn Message>>,
    }
    impl FixApp for TestApp {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn on_logon(&mut self, message: Logon) {
            println!("TestApp adding Logon: {:#?}", message.to_string());
            self.messages.push(Box::new(message));
        }

        fn on_message(&mut self, message: Box<dyn Message>) {
            println!("TestApp adding: {:#?}", message.to_string());
            self.messages.push(message);
        }
    }

    #[test]
    fn basic_cracker() {
        // build up a FIX message, that we 'received'
        let fields = vec!["35=A", "58=Test"];
        let mut buf = BytesMut::with_capacity(1024);
        for field in fields {
            buf.put_slice(field.as_bytes());
            buf.put(&b"\x01"[..]);
        }

        let app = TestApp { messages: vec![] };
        let mut cracker = Cracker { app: Box::new(app) };

        let b = Bytes::from(buf);
        cracker.crack(b);

        let a: Box<dyn FixApp> = cracker.app;
        let b: &TestApp = match a.as_any().downcast_ref::<TestApp>() {
            Some(b) => b,
            None => panic!("&a isn't a B!"),
        };

        let msg_count = b.messages.len();
        assert_eq!(1, msg_count);

        let mmssssgggg = b.messages.first().unwrap();
        assert_eq!(
            "A",
            mmssssgggg
                .header()
                .get_field(Tag::MsgType)
                .ok()
                .unwrap()
                .string_value()
                .unwrap()
        );
        assert_eq!(
            "Test",
            mmssssgggg
                .body()
                .get_field(Tag::Text)
                .ok()
                .unwrap()
                .string_value()
                .unwrap()
        );
    }
}
