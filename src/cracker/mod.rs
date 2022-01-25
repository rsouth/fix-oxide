use std::io::BufRead;
use std::ops::Index;
// use std::io::BufRead;
use bytes::{Buf, Bytes};
use tokio::io::AsyncBufReadExt;
use crate::model::field::Field;
use crate::model::message::{Logon, Message};
use crate::model::message::MsgType;
use crate::model::tag::Tag;

struct Cracker;

// todo link to FixApplication...
impl Cracker {
    pub(crate) fn crack(&self, msg: Bytes) -> Result<Box<dyn Message>, ()> {
        // msg.split(|f| f.is_ascii())
        //     .map(|s| {
        //         s.
        //         // let t: Tag = (x.next().unwrap().unwrap() as u16).into();
        //         // let v = (x.next().unwrap().unwrap() as u16).into();
        //
        //     }).into_iter();
        //     // .filter(|l| )
        //
        //
        //
        // Err(())



        Ok(Box::new(Logon::default()))
    }
}

impl From<&[u8]> for Field {
    fn from(data: &[u8]) -> Self {
        Field::String(Tag::Text, "wer".to_string())
    }
}


#[cfg(test)]
mod tests {
    use crate::cracker::Cracker;
    use bytes::{BufMut, Bytes, BytesMut};
    use crate::model::message::MsgType;

    #[test]
    fn basic_cracker() {

        // build up a FIX message, that we 'recieved'
        let fields = vec!["35=A", "58=Test"];
        let mut buf = BytesMut::with_capacity(1024);
        for x in fields {
            buf.put_slice(x.as_bytes());
            buf.put(&b"\x01"[..]);
        }

        let cracker = Cracker {};

        let b = Bytes::from(buf);

        let m = cracker.crack(b);

        // println!("{}", buf)
        // let mm = m.unwrap();
        assert_eq!(MsgType::Logon, m.unwrap().msg_type());

        //println!("{:#?}", m.unwrap().to_string());


        // let buf = Bytes::from(&b"35=8\x01".to_owned());
        // println!("{:#?}", buf);

    }
}