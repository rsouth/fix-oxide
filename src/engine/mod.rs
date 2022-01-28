use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

use crate::engine::FailedToCreateSession::SessionNotFound;
use crate::session::{SessionID, Settings};

#[derive(Default)]
pub struct Engine {
    sessions: Vec<SessionID>,
    session_settings: HashMap<SessionID, Settings>,
    session_state: HashMap<SessionID, State>,
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
        println!("Creating session using {:#?}", settings);

        // todo validate settings
        // Err(InvalidSettings...)

        let session_id = SessionID::from_settings(&settings);
        if self.sessions.contains(&session_id) {
            return Err(FailedToCreateSession::DuplicateSessionID);
        }

        self.sessions.push(session_id.clone());
        self.session_state.insert(session_id, State::Created);
        self.sessions.last().ok_or(SessionNotFound)
    }

    #[must_use]
    pub fn session_status(&self, session_id: &SessionID) -> Option<&State> {
        self.session_state.get(session_id)
    }

    pub fn logon_session(&self, _session: &SessionID) {}

    pub fn logout_session(&self, _session: &SessionID) {}
}

// state machine; state transitions here based on events.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    Created,
    // session has been created but not yet initialised
    Downtime,
    // session has been initialised, but scheduled downtime is in effect
    LoggedIn,
    // session has been initialised, and is logged in
    LoggedOut, // session has been initialised, but is not logged in (failed? disconnected? etc)
}
