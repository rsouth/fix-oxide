use bytes::Bytes;
use itertools::Itertools;

use crate::application::FixApp;
use crate::model::field::{FieldSet, NoSuchField};
use crate::model::generated::fields::Field;
use crate::model::message::Message;
use crate::model::BeginString;
use crate::session::Crack;

struct Cracker<'a> {
    app: Box<dyn FixApp + 'a>,
}

// todo link to FixApplication...
impl Cracker<'_> {
    pub fn crack(&mut self, msg: &Bytes) {
        let msg_string = String::from_utf8_lossy(msg.as_ref()).to_string();

        let begin_string = BeginString::crack(msg_string.as_str());
        let begin_string = begin_string.unwrap_or_else(|s| panic!("{:?}", s));

        // let fields: Vec<Field> = self.get_cracker(begin_string).crack(msg_string);

        let fields: Vec<Field> = msg_string
            .split('\x01')
            // .filter(|p| p.len() == 2)
            .into_iter()
            // .map(|s| Field::cra ::try_from(s.to_string()))
            .filter(|p| !p.is_empty())
            .map(|s| Field::crack(begin_string, s))
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
        let fields = vec!["8=FIX.4.2", "35=A", "58=Test", "38=123"];
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

    #[test]
    fn basic_cracker_fix_4_0() {
        // build up a FIX message, that we 'received'
        let fields = vec!["8=FIX.4.0", "35=A", "58=Test", "38=123"];
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

        message.get_field(35).as_str();
        // message.get_field(35).as_char();

        // assert_eq!('A', message.get_field(35).as_char());
        assert_eq!("A", message.get_field(35).as_str());
        assert_eq!("Test", message.get_field(58).as_str());

        assert_eq!("8=FIX.4.0|35=A|38=123|58=Test|", message.to_string());
        // println!("{}", message);
    }

    #[test]
    fn basic_cracker_mixed_specs() {
        let app = TestApp { messages: vec![] };
        let mut cracker = Cracker { app: Box::new(app) };

        // FIX.4.0 message
        let fields = vec!["8=FIX.4.0", "35=A", "58=Fix 4.0 Test", "38=123"];
        let mut buf = BytesMut::with_capacity(1024);
        for field in fields {
            buf.put_slice(field.as_bytes());
            buf.put(&b"\x01"[..]);
        }

        let b = Bytes::from(buf);
        cracker.crack(&b);

        ////////
        // FIX.4.4 message
        let fields = vec!["8=FIX.4.4", "35=A", "58=Fix 4.4 Test", "38=123"];
        let mut buf = BytesMut::with_capacity(1024);
        for field in fields {
            buf.put_slice(field.as_bytes());
            buf.put(&b"\x01"[..]);
        }

        let b = Bytes::from(buf);
        cracker.crack(&b);

        let a: Box<dyn FixApp> = cracker.app;
        let b: &TestApp = match a.as_any().downcast_ref::<TestApp>() {
            Some(b) => b,
            None => panic!("&a isn't a B!"),
        };

        let msg_count = b.messages.len();
        assert_eq!(2, msg_count);

        let fix40message = b.messages.iter().nth(0).unwrap();
        let fix44message = b.messages.iter().nth(1).unwrap();
        assert_eq!("Fix 4.0 Test", fix40message.get_field(58).as_str());
        if let Ok(text) = fix40message.get_field_safe(58) {
            assert_eq!("Fix 4.0 Test", text.as_str_safe().unwrap());
        }

        assert_eq!("Fix 4.4 Test", fix44message.get_field(58).as_str());
        if let Ok(text) = fix44message.get_field_safe(58) {
            assert_eq!("Fix 4.4 Test", text.as_str_safe().unwrap());
        }

        // FIX.4.0 defines MsgType as CHAR (which is, in effect, a string)
        // FIX.4.4 (actually 4.2 onwards) differentiate char (single char) and
        // string, and define MsgType as String explicitly
        assert_eq!("A", fix40message.get_field(35).as_str());
        assert_eq!("A", fix40message.get_field(35).as_str());

        assert_eq!(
            "8=FIX.4.0|35=A|38=123|58=Fix 4.0 Test|",
            fix40message.to_string()
        );
        assert_eq!(
            "8=FIX.4.4|35=A|38=123|58=Fix 4.4 Test|",
            fix44message.to_string()
        );
    }
}
