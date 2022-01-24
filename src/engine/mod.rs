use crate::session::{SessionID, Settings};

pub struct Engine {
    sessions: Vec<SessionID>
}

impl Default for Engine {
    fn default() -> Self {
        Engine { sessions: vec![] }
    }
}

impl Engine {
    pub fn create_session(&mut self, _settings: Settings) -> Option<&SessionID> {
        let sid = SessionID {};
        self.sessions.push(sid);
        self.sessions.last()

    }
    pub fn sessions(&self) -> &[SessionID] {
        self.sessions.as_slice()
    }
}
