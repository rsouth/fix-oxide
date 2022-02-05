use crate::session::SessionID;
use std::future::Future;
use std::io::Read;
use tokio::fs::File;
use tokio::io;
use tokio::io::AsyncReadExt;

pub trait Store {
    fn init(&mut self, session_id: &SessionID);

    // todo u32 is large enough for more than 1500 messages per second for a month.
}

impl Store for FileStore {
    fn init(&mut self, session_id: &SessionID) {
        // let load = tokio::fs::File::open(self.to_sender_seqnum_file(session_id));
        // tokio::spawn(load);

        // let initial_sender_seqnum = self.ssn.read_u16().await;
        // Ok(())
        // // .await.and_then(|mut l| l.read_to_end(&mut buf).await);
        // match f.await {
        //     Ok(file) => self.ssn = file,
        //     Err(_) => panic!(""),
        // }
    }
}

pub struct FileStore {
    pub(crate) directory: String,
}

impl FileStore {
    fn to_sender_seqnum_file(&self, session_id: &SessionID) -> String {
        format!("{}.seqnum", session_id.to_string())
    }
}
