use bytes::Bytes;

use crate::application::FixApp;
use crate::model::field::Field;
use crate::model::message::Logon;
use crate::model::tag::Tag;

struct Cracker<'a> {
    app: Box<dyn FixApp + 'a>, // app: dyn &FixApp,
}

// todo link to FixApplication...
impl Cracker<'_> {
    pub(crate) fn crack(&mut self, _msg: Bytes) {
        // todo wow.
        // Ok(Box::new(Logon::default()))

        self.app.on_logon(Logon::default());
    }
}

impl From<&[u8]> for Field {
    fn from(_data: &[u8]) -> Self {
        Field::String(Tag::Text, "wer".to_string())
    }
}

#[cfg(test)]
mod tests {
    use bytes::{BufMut, Bytes, BytesMut};
    use std::any::Any;

    use crate::application::FixApp;
    use crate::cracker::Cracker;
    use crate::model::message::{Logon, Message};

    struct TestApp {
        messages: Vec<Box<dyn Message>>,
    }
    impl FixApp for TestApp {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn on_logon(&mut self, message: Logon) {
            self.messages.push(Box::new(message));
        }

        fn on_message(&mut self, message: Box<dyn Message>) {
            self.messages.push(message);
        }
    }

    #[test]
    fn basic_cracker() {
        // build up a FIX message, that we 'received'
        let fields = vec!["35=A", "58=Test"];
        let mut buf = BytesMut::with_capacity(1024);
        for x in fields {
            buf.put_slice(x.as_bytes());
            buf.put(&b"\x01"[..]);
        }

        let app = TestApp { messages: vec![] };
        let mut cracker = Cracker { app: Box::new(app) };

        let b = Bytes::from(buf);
        let _m = cracker.crack(b);

        let a: Box<dyn FixApp> = cracker.app;
        let b: &TestApp = match a.as_any().downcast_ref::<TestApp>() {
            Some(b) => b,
            None => panic!("&a isn't a B!"),
        };

        let i = b.messages.len();
        assert_eq!(1, i);
    }
}
