pub mod filestore;

use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

use crate::engine::FailedToCreateSession::SessionNotFound;
use crate::model::generated::fields::Field;
use crate::model::message::Message;

use crate::session::settings::Settings;
use crate::session::SessionID;

// #[derive(Default)]
pub struct Engine {
    sessions: Vec<SessionID>,
    session_settings: HashMap<SessionID, Settings>,
    session_state: HashMap<SessionID, State>,
    filestore: Box<dyn filestore::Store>,
}

#[derive(Debug)]
pub enum FailedToCreateSession {
    InvalidSettings,
    DuplicateSessionID,
    SessionNotFound,
}

impl fmt::Display for FailedToCreateSession {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "error.")
    }
}

impl Engine {
    #[must_use]
    pub fn create(store: Box<dyn filestore::Store>) -> Self {
        Engine {
            sessions: vec![],
            session_settings: HashMap::default(),
            session_state: HashMap::default(),
            filestore: store,
        }
    }

    #[must_use]
    pub fn sessions(&self) -> &[SessionID] {
        self.sessions.as_slice()
    }

    ///
    /// # Errors
    ///
    pub fn create_session(
        &mut self,
        settings: Settings,
    ) -> Result<&SessionID, FailedToCreateSession> {
        println!("Creating session using {:?}", settings);

        // todo validate settings
        // Err(InvalidSettings...)

        let session_id = SessionID::from_settings(&settings);
        if self.sessions.contains(&session_id) {
            eprintln!("Not creating {}", session_id);
            return Err(FailedToCreateSession::DuplicateSessionID);
        }

        self.filestore.init(&session_id);

        self.sessions.push(session_id.clone());
        self.session_state.insert(session_id, State::Created);
        self.sessions.last().ok_or(SessionNotFound)
    }

    #[must_use]
    pub fn session_status(&self, session_id: &SessionID) -> Option<&State> {
        self.session_state.get(session_id)
    }

    pub fn logon_session(&mut self, session: &SessionID) {
        let mut logon_msg = Message::default(); // todo change this when message generaation is done
        logon_msg.set_field(Field::String(8, session.begin_string.to_string()));
        logon_msg.set_field(Field::String(35, "A".to_string()));

        if self.send(session, logon_msg).is_ok() {
            self.session_state.insert(session.clone(), State::LoginSent); // todo Login Sent
        }
    }

    pub fn logout_session(&self, _session: &SessionID) {}

    pub fn send(&self, session: &SessionID, msg: Message) -> Result<(), ()> {
        println!("{} >> {}", session, msg);
        Ok(())
    }
}

// state machine; state transitions here based on events.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    Created,
    // session has been created but not yet initialised
    Downtime,
    // login msg was sent, waiting for ack...
    LoginSent,
    // session has been initialised, but scheduled downtime is in effect
    LoggedIn,
    // session has been initialised, and is logged in
    LoggedOut, // session has been initialised, but is not logged in (failed? disconnected? etc)
}
