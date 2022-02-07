pub mod settings;

use settings::Settings;
use std::fmt::{Display, Formatter};

// used to refer to a session
// todo session qualifier
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SessionID {
    pub begin_string: String,
    pub sender_comp_id: String,
    pub target_comp_id: String,
}

impl Display for SessionID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}->{}",
            self.begin_string, self.sender_comp_id, self.target_comp_id
        )
    }
}

impl SessionID {
    #[must_use]
    pub fn from_settings(settings: &Settings) -> Self {
        Self {
            begin_string: settings.begin_string.into(),
            sender_comp_id: settings.sender_comp_id.clone(),
            target_comp_id: settings.target_comp_id.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::filestore::FileStore;
    use crate::engine::Engine;
    use crate::model::BeginString;
    use crate::session::settings::{SessionType, Settings, SettingsBuilder};
    use crate::session::State;

    #[test]
    fn it_works() {
        let mut engine = Engine::create(Box::new(FileStore {
            directory: "".to_string(),
        }));

        let settings = SettingsBuilder::new(
            BeginString::Fix42,
            "Rocks".to_string(),
            "Exchange".to_string(),
            SessionType::Initiator,
        )
        .build();
        let settings2 = SettingsBuilder::new(
            BeginString::Fix40,
            "Rocks".to_string(),
            "Venue".to_string(),
            SessionType::Initiator,
        )
        .with_hb_interval(10_000)
        .build();

        let session_id = engine.create_session(settings).unwrap().clone();
        let session_id2 = engine.create_session(settings2).unwrap().clone();
        assert_eq!(2, engine.sessions().len());

        // prevents duplicate sessions being created
        let settings = SettingsBuilder::new(
            BeginString::Fix42,
            "Rocks".to_string(),
            "Exchange".to_string(),
            SessionType::Initiator,
        )
        .build();
        assert!(engine.create_session(settings).is_err());
        assert_eq!(2, engine.sessions().len());

        // let mut engine = engine;
        // let engine = engine;
        assert_eq!(&State::Created, engine.session_status(&session_id).unwrap());
        assert_eq!(
            &State::Created,
            engine.session_status(&session_id2).unwrap()
        );

        engine.logon_session(&session_id2);
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
